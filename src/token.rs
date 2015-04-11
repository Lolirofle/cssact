use css;

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

	//css::Token::Hash(value) => {},//TODO: Colors
	//TODO: http://stackoverflow.com/questions/2168855/is-quoting-the-value-of-url-really-necessary
	//TODO: 0% = 0? Always true?
	//TODO: Pre-calculated calc(..)

	//All other tokens cannot be simplified
	token => token
}}
