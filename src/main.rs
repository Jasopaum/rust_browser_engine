mod dom;
mod html;

fn main() {
    let n = html::parse("<elem toto = \"tutu\"  tata=\"lol\" > test text node </elem>".to_string());
    println!("{:?}", n);
}
