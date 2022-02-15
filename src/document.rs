use cssom;
use dom;
use styling;

use parser::*;

pub fn create_document(html_source: String, css_source: String) -> Document<'static> {
    let mut warnings = vec![];
    let nodes = html_parser::parse(html_source, &mut warnings);
    let style_sheet = css_parser::parse(css_source, &mut warnings);

    let root_node = nodes.first().unwrap();

    let mut document = Document {
        dom: root_node,
        style_tree: styling::StyledNode {
            children: vec![],
            node: root_node,
            specified_values: styling::PropertyMap::new(),
        },
        warnings,
        style_sheet,
    };

    document.on_document_changed();

    return document;
}

pub struct Document<'a> {
    pub dom: &'a dom::Node,
    pub style_tree: styling::StyledNode<'a>,
    pub style_sheet: cssom::StyleSheet,
    pub warnings: Vec<String>,
}

impl Document<'_> {
    pub fn on_document_changed(&mut self) {
        self.style_tree = styling::apply_styling(&self.dom, &self.style_sheet);
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
