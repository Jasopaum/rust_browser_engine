mod dom;
mod html;

fn main() {
    let n = html::parse(r#"
<top>
    <elem toto = "tutu"  tata="lol" >
        test text node
    </elem>
    <dummy> </dummy>
</top>
"#.to_string());
    println!("{:?}", n);
    println!("{}", n);
    println!("=====");
    println!("{}", n);
}
