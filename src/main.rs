pub mod dom;

fn main() {
    let text = dom::text("Hello, world!".to_string());
    let mut body_attrs = dom::AttrMap::new();
    body_attrs.insert("id".to_string(), "body".to_string());
    body_attrs.insert("style".to_string(), "height: 100px;".to_string());
    let body = dom::element("body".to_string(), body_attrs, vec!(text));
    let root = dom::element("html".to_string(), dom::AttrMap::new(), vec!(body));
    println!("Tree: \n{}", root);
}
