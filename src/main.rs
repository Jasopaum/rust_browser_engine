mod dom;
mod html;
mod css;
mod style;

fn main() {
    let html_doc = r#"
<body>
    <p toto = "tutu"  tata="lol" >
        test text node
    </p>
    <dummy> </dummy>
</body>
"#.to_string();

    let mut css_doc = r#"
body {
  background-color:#80abAA;
  letter-spacing:10.5px;
}
p {
  font-style:italic;
  font-family:times;
}
"#.to_string();

    let html_tree = html::parse(html_doc);
    println!("{}\n", html_tree);

    let style_sheet = css::parse(&mut css_doc);
    //println!("{:?}", style_sheet);

    let style_tree = style::build_style_tree(&html_tree, &style_sheet);
    println!("{}\n", style_tree);
}
