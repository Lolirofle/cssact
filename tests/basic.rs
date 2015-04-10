#![feature(plugin)]

#![plugin(csspp)]

#[test]fn basic(){
	assert_eq!(
		css!("  div  {  color: green;   }   "),
		"div{color:green;}"
	);
}

#[test]fn multiple(){
	assert_eq!(
		css!("  div  {  color: green;   }   span{color:red;}"),
		"div{color:green;}span{color:red;}"
	);
}

#[test]fn url(){
	assert_eq!(
		css!("*{background: url(http://example.org/bg.jpg)}"),
		"*{background:url(\"http://example.org/bg.jpg\")}"
	);
}

#[test]fn content_before(){
	assert_eq!(
		css!("a:before{content: \"Yeah\"}"),
		"a:before{content:\"Yeah\"}"
	);
}

#[test]fn zero_value(){
	assert_eq!(
		css!("a{
			width:0px;
			height:+0em;

			top:-0ex;
			bottom:0.0px;
			left:-0.0px;
			right:-0.0001px;

			right:-0.0000;
			right:0.0000;
		}"),
		"a{width:0;height:+0;top:-0;bottom:0;left:-0;right:-0.0001px;right:-0;right:0;}"
	);
}

#[test]fn adjacent_selector(){
	assert_eq!(
		css!("a + div span{color:green;}"),
		"a+div span{color:green;}"
	);
}

#[test]fn multiple_value_rule(){
	assert_eq!(
		css!("a{background: black url(http://example.org/bg.jpg);}"),
		"a{background:black url(http://example.org/bg.jpg);}"
	);
}

/*#[test]fn hex_colors(){
	assert_eq!(
		css!("a{
			color: #19F;
			color: #1199FF;
			color: #102233;
			color: #112033;
			color: #112230;
			color: #aABbCC;
		}"),
		"a{color:#19F;color:#19F;color:#102233;color:#112033;color:#112230;color:#ABC;}"
	);
}*/

//#[test]fn f01(){assert_eq!(css!(include_str!("css/comments.css"       )),include_str!("css/comments.min.css"       ));}
//#[test]fn f02(){assert_eq!(css!(include_str!("css/hacks.css"          )),include_str!("css/hacks.min.css"          ));}
//#[test]fn f03(){assert_eq!(css!(include_str!("css/issue62.css"        )),include_str!("css/issue62.min.css"        ));}
//#[test]fn f04(){assert_eq!(css!(include_str!("css/issue210.css"       )),include_str!("css/issue210.min.css"       ));}
//#[test]fn f05(){assert_eq!(css!(include_str!("css/paths_prepend.css"  )),include_str!("css/paths_prepend.min.css"  ));}
//#[test]fn f06(){assert_eq!(css!(include_str!("css/paths_rewrite.css"  )),include_str!("css/paths_rewrite.min.css"  ));}
//#[test]fn f07(){assert_eq!(css!(include_str!("css/selectors.css"      )),include_str!("css/selectors.min.css"      ));}
//#[test]fn f08(){assert_eq!(css!(include_str!("css/styles.css"         )),include_str!("css/styles.min.css"         ));}
//#[test]fn f09(){assert_eq!(css!(include_str!("css/subsilver.css"      )),include_str!("css/subsilver.min.css"      ));}
//#[test]fn f10(){assert_eq!(css!(include_str!("css/unusual_strings.css")),include_str!("css/unusual_strings.min.css"));}
//#[test]fn f11(){assert_eq!(css!(include_str!("css/vladmirated.css"    )),include_str!("css/vladmirated.min.css"    ));}
