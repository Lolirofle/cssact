#![feature(plugin)]

#![plugin(csspp)]

#[test]
fn basic(){
	let s = css!("  div  {  color: green;   }   ");
	assert_eq!(s,"div{color:green;}");
}

#[test]
fn multiple(){
	let s = css!("  div  {  color: green;   }   span{color:red;}");
	println!("{}",s);
	assert_eq!(s,"div{color:green;}span{color:red;}");
}

#[test]
fn url(){
	let s = css!("*{background: url(http://example.org/bg.jpg)}");
	println!("{}",s);
	assert_eq!(s,"*{background:url(\"http://example.org/bg.jpg\")}");
}


#[test]
fn content_before(){
	let s = css!("a:before{content: \"Yeah\"}");
	println!("{}",s);
	assert_eq!(s,"a:before{content:\"Yeah\"}");
}
