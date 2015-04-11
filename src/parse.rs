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

		//For every selector token
		while let Ok(token) = input.next/*_including_whitespace*/(){
			out.push_str(&token.to_css_string());
		}

		Ok(out)
	}

	//When parsing the declarations block
	fn parse_block(&self,mut prelude: <Self as css::QualifiedRuleParser>::Prelude,input: &mut css::Parser) -> Result<<Self as css::QualifiedRuleParser>::QualifiedRule,()>{
		//Block begin
		prelude.push_str("{");

		//For every declaration
		for decl in css::DeclarationListParser::new(input,DeclParser){
			match decl{
				Ok(decl) => prelude.push_str(&decl),
				Err(_)   => return Err(())
			}
		}

		//Block end
		prelude.push_str("}");

		//Accumulated string
		Ok(prelude)
	}
}
impl css::AtRuleParser for RuleParser{
	type Prelude = ();
	type AtRule = String;
}

//When parsing a declaration
pub struct DeclParser;
impl css::DeclarationParser for DeclParser{
	type Declaration = String;

	//When parsing a declaration; a property and its values
	fn parse_value(&self,name: &str,input: &mut css::Parser) -> Result<<Self as css::DeclarationParser>::Declaration,()>{
		let mut out = format!("{}:",name);

		//For the first value (head)
		if let Ok(token) = input.next(){
			out.push_str(&token::value_token_simplify(token).to_css_string());

			//For every other value (tail)
			while let Ok(token) = input.next(){
				out.push_str(" ");
				out.push_str(&token::value_token_simplify(token).to_css_string());
			}
		}
		out.push_str(";");
		Ok(out)
	}
}
impl css::AtRuleParser for DeclParser{
	type Prelude = ();
	type AtRule = String;
}
