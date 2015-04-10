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
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry){
	reg.register_macro("css",expand_css);
}

fn expand_css<'context>(context: &'context mut ExtCtxt,span: Span,tts: &[ast::TokenTree]) -> Box<MacResult + 'context>{
	let input = match tts{
		[ast::TtToken(_,token::Literal(token::Lit::Str_(s),_))] => token::get_name(s),
		[ref tt] => {
			let span = tt.get_span();
			context.span_err(span,"Expected a string literal");
			return DummyResult::any(span);
		},
		_ => {
			context.span_err(span,"Expected a single string literal as the arguments");
			return DummyResult::any(span);
		}
	};
println!("{}",input);
	let mut parser = css::Parser::new(&*input);
	let mut output = String::new();

	while let Ok(i) = parser.next(){
		output.push_str(&*i.to_css_string());

		match i{
			block @ css::Token::ParenthesisBlock |
			block @ css::Token::CurlyBracketBlock |
			block @ css::Token::SquareBracketBlock => parser.parse_nested_block(|parser|{
				while let Ok(i) = parser.next(){
					output.push_str(&*i.to_css_string());
				}

				output.push_str(&*match block{
					css::Token::ParenthesisBlock   => css::Token::CloseParenthesis,
					css::Token::CurlyBracketBlock  => css::Token::CloseCurlyBracket,
					css::Token::SquareBracketBlock => css::Token::CloseSquareBracket,
					_ => unreachable!()
				}.to_css_string());

				Ok(())
			}).unwrap(),
			_ => {}
		};
	}

	MacEager::expr(context.expr_str(span,token::intern_and_get_ident(&*output)))
}
