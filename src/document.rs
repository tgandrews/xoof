use cssom;
use dom;
use styling;

use parser::*;

pub fn create_document(html_source: String, css_source: String) -> Document {
    let mut warnings = vec![];
    let dom = html_parser::parse(html_source, &mut warnings);
    let style_sheet = css_parser::parse(css_source, &mut warnings);

    let mut document = Document {
        dom: dom.first().unwrap().clone(),
        style_sheet,
        warnings,
    };

    document.on_document_changed();

    return document;
}

pub struct Document {
    pub dom: dom::Node,
    pub style_sheet: cssom::StyleSheet,
    pub warnings: Vec<String>,
}

impl Document {
    pub fn on_document_changed(&mut self) {
        self.dom = styling::apply_styling(&self.dom, &self.style_sheet);
    }

    pub fn dump_dom_tree(&self) -> String {
        format!("{}", self.dom)
    }

    pub fn dump_styles(&self) -> String {
        let mut output = String::new();
        for style in &self.style_sheet.rules {
            output.push_str(format!("{:#?}\n", style).as_str());
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
