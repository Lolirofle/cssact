#![feature(plugin_registrar,quote,rustc_private,slice_patterns)]

extern crate syntax;
extern crate rustc;
extern crate cssparser as css;

use css::ToCss;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt,MacEager,MacResult,DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::print::pprust;
use syntax::fold::Folder;
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry){
	reg.register_macro("css",expand_css);
}

fn expand_css<'context>(context: &'context mut ExtCtxt,span: Span,tts: &[ast::TokenTree]) -> Box<MacResult + 'context>{
	let input = match parse_single_string_literal(context,tts){
		Some(s) => s,
		None => return DummyResult::any(span)
	};

	let mut parser = css::Parser::new(&*input);
	let mut output = String::new();

	while let Ok(i) = parser.next(){
		match i{
			//Blocks/brackets
			ref block @ css::Token::ParenthesisBlock |
			ref block @ css::Token::CurlyBracketBlock |
			ref block @ css::Token::SquareBracketBlock => match parser.parse_nested_block(|parser|{
				//Open bracket
				output.push_str(&*i.to_css_string());

				//Bracket contents
				while let Ok(i) = parser.next(){
					match i{
						//Handling zero. Remove unit and fraction from 0 (https://developer.mozilla.org/en-US/docs/Web/CSS/length)
						css::Token::Dimension(number @ css::NumericValue{int_value: Some(0),..},_) => {
							output.push_str(&*css::Token::Number(number).to_css_string());
						},
						css::Token::Dimension(mut number @ css::NumericValue{value: 0.0,..},_) |
						css::Token::Number(mut number @ css::NumericValue{value: 0.0,..})=> {
							number.int_value = Some(0);
							output.push_str(&*css::Token::Number(number).to_css_string());
						},

						//css::Token::Hash(value) => {},

						//All other tokens
						token => output.push_str(&*token.to_css_string())
					}
				}

				//Close bracket
				output.push_str(&*match block{
					&css::Token::ParenthesisBlock   => css::Token::CloseParenthesis,
					&css::Token::CurlyBracketBlock  => css::Token::CloseCurlyBracket,
					&css::Token::SquareBracketBlock => css::Token::CloseSquareBracket,
					_ => unreachable!()
				}.to_css_string());

				Ok(())
			}){
				Ok(_)  => {},
				Err(_) => {
       				 context.span_err(span,"unexpected token somewhere");
					return DummyResult::any(span);
				}
			},

			//All other tokens
			token => {
				output.push_str(&*token.to_css_string());
			}
		};
	}

	MacEager::expr(context.expr_str(span,token::intern_and_get_ident(&*output)))
}

////////////////////////////////////////////////////////////////////
// This code snippet is copied from the `rust-lang/regex` package and slightly modified.
// (Date: 2015-04-10. Version: 0.1.27. Commit: 910aef40aca4f525dd2fecc54a78e9bc183039d1)
// (`https://github.com/rust-lang/regex/blob/910aef40aca4f525dd2fecc54a78e9bc183039d1/regex_macros/src/lib.rs`)
// Credits goes to the developers of the regex package.
////////////////////////////////////////////////////////////////////
// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////
/// Looks for a single string literal and returns it.
/// Otherwise, logs an error with cx.span_err and returns None.
fn parse_single_string_literal(cx: &mut ExtCtxt, tts: &[ast::TokenTree]) -> Option<String> {
    let mut parser = cx.new_parser_from_tts(tts);
    let entry = cx.expander().fold_expr(parser.parse_expr());
    let regex = match entry.node {
        ast::ExprLit(ref lit) => {
            match lit.node {
                ast::LitStr(ref s, _) => s.to_string(),
                _ => {
                    cx.span_err(entry.span, &format!(
                        "expected string literal but got `{}`",
                        pprust::lit_to_string(&**lit)));
                    return None
                }
            }
        }
        _ => {
            cx.span_err(entry.span, &format!(
                "expected string literal but got `{}`",
                pprust::expr_to_string(&*entry)));
            return None
        }
    };
    if !parser.eat(&token::Eof)/*.ok().unwrap()*/ {
        cx.span_err(parser.span, "only one string literal allowed");
        return None;
    }
    Some(regex)
}
