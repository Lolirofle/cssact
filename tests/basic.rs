#![feature(plugin)]

#![plugin(cssact)]

#[test]fn basic(){
	assert_eq!(
		css!("  div  {  color: green;   }   "),
		"div{color:green}"
	);
}

#[test]fn multiple(){
	assert_eq!(
		css!("  div  {  color: green;   }   span{color:red;}a{color:blue;}"),
		"div{color:green}span{color:red}a{color:blue}"
	);
}

#[test]fn url(){
	assert_eq!(
		css!("*{background: url(http://example.org/bg.jpg);}"),
		"*{background:url(\"http://example.org/bg.jpg\")}"
	);
}

#[test]fn content_before(){
	assert_eq!(
		css!("a:before{content: \"Yeah\";}"),
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
		"a{width:0;height:+0;top:-0;bottom:0;left:-0;right:-0.0001px;right:-0;right:0}"
	);
}

#[test]fn selectors(){
	assert_eq!(
		css!(".a #div . span{color:green;}"),
		".a #div .span{color:green}"
	);
}

#[test]fn adjacent_selector(){
	assert_eq!(
		css!("a + div span{color:green;}"),
		"a+div span{color:green}"
	);
}

#[test]fn multiple_value_rule(){
	assert_eq!(
		css!("a{background: black url(http://example.org/bg.jpg);}"),
		"a{background:black url(\"http://example.org/bg.jpg\")}"
	);
}

#[test]fn hex_colors(){
	assert_eq!(
		css!("a{
			color: #19F;
			color: #1199FF;
			color: #102233;
			color: #112033;
			color: #112230;
			color: #aABbCC;
		}"),
		"a{color:#19F;color:#19F;color:#102233;color:#112033;color:#112230;color:#aBC}"
	);
}

#[test]fn square_bracket_selector(){
	assert_eq!(
		css!("a[ id = \"b\" ] [id=\"c\"] a [class=\"d\"][data-shit=\"e\"]{color:teal;}"),
		"a[id=\"b\"] [id=\"c\"] a [class=\"d\"][data-shit=\"e\"]{color:teal}"
	);
}

#[test]fn semicolons(){
	assert_eq!(
		css!("a{  ;  ;    ;   ;;;color   :     green;;;;;; ;; ;}"),
		"a{color:green}"
	);
}

#[test]fn empty_declarations(){
	assert_eq!(
		css!("a{  ;  ;    ;   ;;;           ;;;;;; ;; ;}b{}c{     }d{;}"),
		""
	);
}

#[test]fn nested_at_rule(){
	assert_eq!(
		css!("@media screen {
                  body { font-size: 13px }
              }"),
		"@media screen{body{font-size:13px}}"
	);
}

#[test]fn function_values(){
	assert_eq!(
		css!("a{color:rgba(4,2,1,0.5)}"),
		     "a{color:rgba(4,2,1,0.5)}"
	);
}

#[test]fn media_query(){
	assert_eq!(
		css!("
			@media (width: 500px){a{color :red}}
			@media       screen     and  print        { a { color : green    }    }
			@media not (width: 500px) ,   print{a{color:blue}}
			@media only (width: 500px) and color{a{color:yellow}}
		"),
		"@media (width:500px){a{color:red}}@media screen and print{a{color:green}}@media not (width:500px),print{a{color:blue}}@media only (width:500px) and color{a{color:yellow}}"
	);
}

/*TODO: Support for YUI compressor style comments?*///#[test]fn f01(){assert_eq!(css!(include_str!("css/comments.css"       )),include_str!("css/comments.min.css"       ));}
/*TODO: Various minimizing issues*///#[test]fn f02(){assert_eq!(css!(include_str!("css/hacks.css"          )),include_str!("css/hacks.min.css"          ));}
/*TODO: Color shortening and f04 problems*///#[test]fn f03(){assert_eq!(css!(include_str!("css/issue62.css"        )),include_str!("css/issue62.min.css"        ));}
/*TODO: Font family whitespace*///#[test]fn f04(){assert_eq!(css!(include_str!("css/issue210.css"       )),include_str!("css/issue210.min.css"       ));}
/*TODO: Comma values, quoted URLs. Prefixing?*///#[test]fn f05(){assert_eq!(css!(include_str!("css/paths_prepend.css"  )),include_str!("css/paths_prepend.min.css"  ));}
/*TODO: Rewriting? Probably same issues as f05*///#[test]fn f06(){assert_eq!(css!(include_str!("css/paths_rewrite.css"  )),include_str!("css/paths_rewrite.min.css"  ));}
#[test]fn f07(){assert_eq!(css!(include_str!("css/selectors.css"      )),include_str!("css/selectors.min.css"      ));}
/*TODO: @ rules*/#[test]//fn f08(){assert_eq!(css!(include_str!("css/styles.css"         )),include_str!("css/styles.min.css"         ));}
/*TODO: Same problem as f07*///#[test]fn f09(){assert_eq!(css!(include_str!("css/subsilver.css"      )),include_str!("css/subsilver.min.css"      ));}
/*TODO: Newline in string*///#[test]fn f10(){assert_eq!(css!(include_str!("css/unusual_strings.css")),include_str!("css/unusual_strings.min.css"));}
//#[test]fn f11(){assert_eq!(css!(include_str!("css/vladmirated.css"    )),include_str!("css/vladmirated.min.css"    ));}
#[test]fn f12(){assert_eq!(css!(include_str!("css/box_model_hack.css"          )),include_str!("css/box_model_hack.min.css"          ));}
#[test]fn f13(){assert_eq!(css!(include_str!("css/backslash_hack.css"          )),include_str!("css/backslash_hack.min.css"          ));}
#[test]fn f14(){assert_eq!(css!(include_str!("css/underscore_hack.css"         )),include_str!("css/underscore_hack.min.css"         ));}
#[test]fn f15(){assert_eq!(css!(include_str!("css/commented_backslash_hack.css")),include_str!("css/commented_backslash_hack.min.css"));}
