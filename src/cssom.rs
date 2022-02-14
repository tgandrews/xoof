use super::dom::*;
use std::cmp::Ordering;

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

pub struct Specificity {
    id: usize,
    classes: usize,
    tags: usize,
}

impl Specificity {
    //  TODO: Hold the position in the document as that should also be used
    pub fn cmp<'a>(&'a self, comparator: &'a Specificity) -> (Ordering, &Specificity) {
        if &self.id < &comparator.id {
            (Ordering::Less, comparator)
        } else if &self.id > &comparator.id {
            (Ordering::Greater, comparator)
        } else if &self.classes < &comparator.classes {
            (Ordering::Less, comparator)
        } else if &self.classes > &comparator.classes {
            (Ordering::Greater, comparator)
        } else if &self.tags < &comparator.tags {
            (Ordering::Less, comparator)
        } else if &self.tags > &comparator.tags {
            (Ordering::Greater, comparator)
        } else {
            (Ordering::Equal, comparator)
        }
    }
}

impl Selector {
    pub fn specificity(&self) -> Specificity {
        match &self.selector_type {
            SelectorType::SimpleSelector(simple) => {
                let id = simple.id.iter().count();
                let classes = simple.class.len();
                let tags = simple.tag_name.iter().count();
                Specificity { id, classes, tags }
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
    pub value: String,
}
