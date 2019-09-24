pub mod dom;
pub mod html;

fn main() {
    let n = html::parse("test text node".to_string());
    println!("{:?}", n);
}
