use super::dom::*;

#[derive(Clone)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Clone, Debug)]
pub enum SelectorType {
    SimpleSelector(SimpleSelectorData),
}

#[derive(Clone, Debug)]
pub struct Selector {
    pub selector_type: SelectorType,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        match &self.selector_type {
            SelectorType::SimpleSelector(simple) => {
                let id = simple.id.iter().count();
                let classes = simple.class.len();
                let tags = simple.tag_name.iter().count();
                (id, classes, tags)
            }
        }
    }

    pub fn matches(&self, element: &ElementData) -> bool {
        match &self.selector_type {
            SelectorType::SimpleSelector(simple) => {
                if simple.tag_name.iter().any(|tag| element.tag_name != *tag) {
                    return false;
                }

                if simple.id.iter().any(|id| element.id() != Some(id)) {
                    return false;
                }

                if simple
                    .class
                    .iter()
                    .any(|class| !element.class_list().contains(&**class))
                {
                    return false;
                }

                return true;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSelectorData {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Clone, Debug)]
pub enum Value {
    // Keyword(String),
    Length(f32, Unit),
    // ColorValue(Color),
}

#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Px,
}

// #[derive(Clone, Copy)]
// pub struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
//     a: u8,
// }
