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

impl Document {
    pub fn dump_dom_tree(&self) -> String {
        let mut output = String::new();
        for node in &self.dom {
            output += format!("{}\n", node).as_str();
        }
        return output;
    }

    pub fn dump_styles(&self) -> String {
        let mut output = String::new();
        for style in &self.style_sheet.rules {
            output += format!("{:#?}\n", style).as_str();
        }
        return output;
    }

    pub fn dump_warnings(&self) -> String {
        let mut output = String::new();
        for warning in &self.warnings {
            output += format!("{}\n", warning).as_str();
        }
        return output;
    }
}
