mod dom;
mod html;

fn main() {
    let n = html::parse("<elem toto=tutu> test text node".to_string());
    println!("{:?}", n);
}
