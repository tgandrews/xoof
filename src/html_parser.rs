use dom;

pub fn parse(html: String) -> Result<dom::Node, String> {
    Ok(dom::text("Hello world!".to_string()))
}
