use collections::borrow::Cow;
use css;
use std::ascii::AsciiExt;

pub fn value_token_simplify(token: css::Token) -> css::Token{match token{
	//Handling zero. Remove unit and fraction from 0 (https://developer.mozilla.org/en-US/docs/Web/CSS/length)
	css::Token::Dimension(number @ css::NumericValue{int_value: Some(0),..},_) => {
		css::Token::Number(number)
	},
	css::Token::Dimension(mut number @ css::NumericValue{value: 0.0,..},_) |
	css::Token::Number(mut number @ css::NumericValue{value: 0.0,..})=> {
		number.int_value = Some(0);
		css::Token::Number(number)
	},

	//TODO: 0% = 0? Always true?
	css::Token::Percentage(percentage @ css::PercentageValue{unit_value: 0.0,..}) |
	css::Token::Percentage(percentage @ css::PercentageValue{int_value: Some(0),..}) => {
		css::Token::Number(css::NumericValue{
			value: 0.0,
			int_value: Some(0),
			has_sign: percentage.has_sign
		})
	},

	//Long hex colors
	//TODO: Named colors
	css::Token::Hash  (hex_str) |
	css::Token::IDHash(hex_str) => {
		match (&*hex_str).as_bytes(){
			[a,b,c,d,e,f] if a.to_ascii_uppercase()==b.to_ascii_uppercase()
			              && c.to_ascii_uppercase()==d.to_ascii_uppercase()
			              && e.to_ascii_uppercase()==f.to_ascii_uppercase()
			  => css::Token::Hash(Cow::Owned(unsafe{String::from_utf8_unchecked(vec![a,c,e])})),
			_ => css::Token::Hash(hex_str)//TODO: Can also be a IDHash
		}
	},
	//TODO: http://stackoverflow.com/questions/2168855/is-quoting-the-value-of-url-really-necessary
	//TODO: Pre-calculated calc(..)

	//All other tokens cannot be simplified
	token => token
}}
