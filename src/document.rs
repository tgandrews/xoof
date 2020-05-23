use cssom;
use dom;

use parser::*;

pub fn create_document(html_source: String, css_source: String) -> Document {
    let mut warnings = vec![];
    let dom = html_parser::parse(html_source, &mut warnings);
    let style_sheet = css_parser::parse(css_source, &mut warnings);

    return Document {
        dom,
        style_sheet,
        warnings,
    };
}

pub struct Document {
    pub dom: Vec<dom::Node>,
    pub style_sheet: cssom::StyleSheet,
    pub warnings: Vec<String>,
}
