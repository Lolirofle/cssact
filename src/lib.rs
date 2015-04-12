#![feature(collections,plugin_registrar,quote,rustc_private,slice_patterns)]

extern crate collections;
extern crate syntax;
extern crate rustc;
extern crate cssparser as css;

use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt,MacEager,MacResult,DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::token as rust_token;
use syntax::print::pprust;
use syntax::fold::Folder;
use rustc::plugin::Registry;

mod token;
mod parse;

//TODO: Option to issue a warning instead of an error
//TODO: Option to stop at the first error or enumerate all of them
//TODO: Parse the values syntax for each property type in a declaration
//TODO: Shorten color values by comparing the representations (Color name, its rgb(a) string, hex value...) and look for the shortest one
//TODO: Option to just check for errors

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry){
	reg.register_macro("css",expand_css);
}

fn expand_css<'context>(context: &'context mut ExtCtxt,span: Span,tts: &[ast::TokenTree]) -> Box<MacResult + 'context>{
	let (input,input_span) = match parse_single_string_literal(context,tts){
		(Some(s),span) => (s,span),
		(None,span) => return DummyResult::any(span)
	};

	let mut parser = css::Parser::new(&input);
	let mut output = String::new();

	//For every rule
	for rule in css::RuleListParser::new_for_stylesheet(&mut parser,parse::RuleParser){
		match rule{
			Ok(rule_str) => output.push_str(&rule_str),
			Err(pos) => {
				let parser = css::Parser::new(&input);//TODO: Find a method for using the old parser
				let start_location = parser.source_location(pos.start);
				let end_location   = parser.source_location(pos.end);
				context.span_err(input_span,&format!("CSS parsing error @ {}:{} to {}:{}",
					start_location.line,
					start_location.column,
					end_location.line,
					end_location.column,
				));
				return DummyResult::any(input_span);
			}
		}
	}

	//Return the expression, making the macro expand to a string literal
	MacEager::expr(context.expr_str(span,rust_token::intern_and_get_ident(&output)))
}

////////////////////////////////////////////////////////////////////
// This code snippet is copied from the `rust-lang/regex` package and is also slightly modified.
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
fn parse_single_string_literal(cx: &mut ExtCtxt,tts: &[ast::TokenTree]) -> (Option<String>,Span){
	let mut parser = cx.new_parser_from_tts(tts);
	let entry = cx.expander().fold_expr(parser.parse_expr());
	let s = match entry.node{
		ast::ExprLit(ref lit) => {
			match lit.node {
				ast::LitStr(ref s,_) => s.to_string(),
				_ => {
					cx.span_err(entry.span,&format!(
						"expected string literal but got `{}`",
						pprust::lit_to_string(&**lit)));
					return (None,entry.span)
				}
			}
		}
		_ => {
			cx.span_err(entry.span,&format!(
				"expected string literal but got `{}`",
				pprust::expr_to_string(&*entry)));
			return (None,entry.span)
		}
	};
	if !parser.eat(&rust_token::Eof).ok().unwrap(){
		cx.span_err(parser.span,"only one string literal allowed");
		return (None,parser.span);
	}
	(Some(s),entry.span)
}
