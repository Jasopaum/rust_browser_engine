mod dom;
mod html;
mod css;
mod style;
mod layout;
mod render;

fn main() {
    let html_doc = r#"
<body>
    <p toto = "tutu"  tata="lol" >
        test text node
    </p>
    <dummy> other text but very very very long this time because I need to test how long texts are dsplayed </dummy>
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

    let layout_tree = layout::build_layout_tree(&style_tree);
    println!("{:?}\n", layout_tree);

    render::render_layout_tree(&layout_tree);
}

