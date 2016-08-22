pub mod dom;

fn main() {
    let text = dom::text("Hello, world!".to_string());
    let body = dom::element("body".to_string(), dom::AttrMap::new(), vec!(text));
    let root = dom::element("html".to_string(), dom::AttrMap::new(), vec!(body));
    println!("Tree: \n{}", root);
}
