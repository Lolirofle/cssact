use css;
use css::ToCss;

use token;

//CSS terms:
//  RULE_LIST/RULE_SET = RULE RULE ..RULE
//  RULE               = SELECTOR DECLARATIONS_BLOCK
//  DECLARATIONS_BLOCK = { DECLARATION; DECLARATION; DECLARATION.. }
//  DECLARATION        = PROPERTY: VALUE

//When parsing a rule
pub struct RuleParser;
impl css::QualifiedRuleParser for RuleParser{
	type Prelude = String;
	type QualifiedRule = String;

	//When parsing the selector
	fn parse_prelude(&self,input: &mut css::Parser) -> Result<<Self as css::QualifiedRuleParser>::Prelude,()>{
		let mut out = String::new();
		let mut previous_token = css::Token::Semicolon;

		//For every selector token
		while let Ok(token) = input.next_including_whitespace(){
			match (&previous_token,&token){
				//Whitespace after an identifier: Register the token in case it is a descendant operator
				(&css::Token::Delim('*')        ,&css::Token::WhiteSpace(_)) |
				(&css::Token::Ident(_)          ,&css::Token::WhiteSpace(_)) |
				(&css::Token::IDHash(_)         ,&css::Token::WhiteSpace(_)) |
				(&css::Token::Function(_)       ,&css::Token::WhiteSpace(_)) |
				(&css::Token::SquareBracketBlock,&css::Token::WhiteSpace(_)) => {},

				//All other whitespaces: Ignore
				(_,&css::Token::WhiteSpace(_)) => continue,

				//Unary operator after a whitespace: Previous whitespace was a descendant operator
				(&css::Token::WhiteSpace(_),&css::Token::Delim('*'))         |
				(&css::Token::WhiteSpace(_),&css::Token::Ident(_))           |
				(&css::Token::WhiteSpace(_),&css::Token::IDHash(_))          |
				(&css::Token::WhiteSpace(_),&css::Token::Delim('.'))         |
				(&css::Token::WhiteSpace(_),&css::Token::SquareBracketBlock) |
				(&css::Token::WhiteSpace(_),&css::Token::Colon) => {
					//Write the previously skipped whitespace token using a single whitespace for saving space
					out.push_str(" ");

					//Write the token
					out.push_str(&token.to_css_string());
				},

				_ => {
					//Write the token
					out.push_str(&token.to_css_string());
				}
			};

			match &token{//TODO: Remove quotes when unneccessary, and issue an error when it is required
				//Square bracket blocks: Selector attributes
				block @ &css::Token::SquareBracketBlock |
				block @ &css::Token::Function(_)        => try!(input.parse_nested_block(|input|{
					//Bracket contents
					while let Ok(token) = input.next(){
						out.push_str(&*token::value_token_simplify(token).to_css_string());
					}

					//Close bracket
					out.push_str(&*match block{
						&css::Token::Function(_)        => css::Token::CloseParenthesis,
						&css::Token::SquareBracketBlock => css::Token::CloseSquareBracket,
						_ => unreachable!()
					}.to_css_string());
					Ok(())
				})),
				_ => {}
			}

			//Update the register
			previous_token = token;
		}

		Ok(out)
	}

	//When parsing the declarations block
	fn parse_block(&self,mut prelude: <Self as css::QualifiedRuleParser>::Prelude,input: &mut css::Parser) -> Result<<Self as css::QualifiedRuleParser>::QualifiedRule,()>{
		//Block begin
		prelude.push_str("{");

		//Declarations
		let mut decls = css::DeclarationListParser::new(input,DeclParser);

		//For the first declaration (head)
		if let Some(decl) = decls.next(){
			prelude.push_str(&*try!(decl.map_err(|_| ())));

			//For every other declaration (tail)
			for decl in decls{
				prelude.push_str(";");
				prelude.push_str(&*try!(decl.map_err(|_| ())));
			}
		}else{
			//Empty declaration
			return Ok(String::new())
		}

		//Block end
		prelude.push_str("}");

		//Accumulated string
		Ok(prelude)
	}
}
impl css::AtRuleParser for RuleParser{
	type Prelude = <Self as css::QualifiedRuleParser>::Prelude;
	type AtRule  = <Self as css::QualifiedRuleParser>::QualifiedRule;

	fn parse_prelude(&self,name: &str,input: &mut css::Parser) -> Result<css::AtRuleType<<Self as css::AtRuleParser>::Prelude,<Self as css::AtRuleParser>::AtRule>,()>{
		let mut out = format!("@{}",name);

		while let Ok(token) = input.next(){
			out.push_str(" ");
			out.push_str(&token.to_css_string());
		}

		Ok(css::AtRuleType::OptionalBlock(out))
	}

	fn parse_block(&self,mut prelude: <Self as css::AtRuleParser>::Prelude,input: &mut css::Parser) -> Result<<Self as css::AtRuleParser>::AtRule,()>{
		prelude.push_str("{");

		while let Ok(token) = input.next(){
			prelude.push_str(&token.to_css_string());
		}

		prelude.push_str("}");

		Ok(prelude)
		/*(self as &css::QualifiedRuleParser<
			Prelude       = <Self as css::AtRuleParser>::Prelude,
			QualifiedRule = <Self as css::AtRuleParser>::AtRule,
		>).parse_block(prelude,input)*/
	}

	fn rule_without_block(&self,mut prelude: <Self as css::AtRuleParser>::Prelude) -> <Self as css::AtRuleParser>::AtRule{
		prelude.push_str(";");
		prelude
	}
}

//When parsing a declaration
pub struct DeclParser;
impl css::DeclarationParser for DeclParser{
	type Declaration = String;

	//When parsing a declaration; a property and its values
	fn parse_value(&self,property: &str,input: &mut css::Parser) -> Result<<Self as css::DeclarationParser>::Declaration,()>{
		let mut out = format!("{}:",property);

		fn parse_value_token(property: &str,token: css::Token,input: &mut css::Parser,out: &mut String) -> Result<(),()>{
			let _ = property;

			match token{
				css::Token::Function(name) => input.parse_nested_block(|input|{//TODO: Remove quotes when unneccessary, and issue an error when it is required e.g. in url(..)
					//Function name
					out.push_str(&name);

					//Open bracket
					out.push_str("(");

					//Bracket contents
					while let Ok(token) = input.next(){
						try!(parse_value_token(property,token,input,out));
					}

					//Close bracket
					out.push_str(")");

					Ok(())
				}),

				token => {
					out.push_str(&token::value_token_simplify(token).to_css_string());

					Ok(())
				}
			}
		}

		//For the first value (head)
		if let Ok(token) = input.next(){
			try!(parse_value_token(property,token,input,&mut out));

			//For every other value (tail)
			while let Ok(token) = input.next(){//TODO: Comma and font names, see test f04
				out.push_str(" ");
				try!(parse_value_token(property,token,input,&mut out));
			}
		}
		Ok(out)
	}
}
impl css::AtRuleParser for DeclParser{
	type Prelude = String;
	type AtRule = String;

	fn parse_prelude(&self,name: &str,input: &mut css::Parser) -> Result<css::AtRuleType<<Self as css::AtRuleParser>::Prelude,<Self as css::AtRuleParser>::AtRule>,()>{
		println!("ok1");
		let _ = name;
		let _ = input;
		Ok(css::AtRuleType::OptionalBlock(String::new()))
	}

	fn parse_block(&self,prelude: <Self as css::AtRuleParser>::Prelude,input: &mut css::Parser) -> Result<<Self as css::AtRuleParser>::AtRule,()>{
		println!("ok2");
		let _ = prelude;
		let _ = input;
		Ok(String::new())
	}

	fn rule_without_block(&self,prelude: <Self as css::AtRuleParser>::Prelude) -> <Self as css::AtRuleParser>::AtRule{
		println!("ok3");
		let _ = prelude;
		String::new()
	}
}
