pub mod dom;

fn main() {
    let h1_text = dom::text("Title".to_string());
    let h1 = dom::element("h1".to_string(), dom::AttrMap::new(), vec!(h1_text));
    let text = dom::text("Hello, world!".to_string());
    let mut body_attrs = dom::AttrMap::new();
    body_attrs.insert("id".to_string(), "body".to_string());
    body_attrs.insert("style".to_string(), "height: 100px;".to_string());
    let body = dom::element("body".to_string(), body_attrs, vec!(h1, text));
    let root = dom::element("html".to_string(), dom::AttrMap::new(), vec!(body));
    println!("Tree: \n{}", root);
}
