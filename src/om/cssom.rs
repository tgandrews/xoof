#[derive(Clone)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(Clone)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Clone)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Clone)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Clone)]
pub enum Value {
    // Keyword(String),
    Length(f32, Unit),
    // ColorValue(Color),
}

#[derive(Clone, Copy)]
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
