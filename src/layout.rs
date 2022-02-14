use dom::*;

struct Dimensions {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct LayoutBox<'a> {
    dimensions: Dimensions,
    children: Vec<LayoutBox<'a>>,
    node: &'a Node,
}

pub fn create_layout_tree<'a>(root_node: &'a Node, max_width: f32) -> LayoutBox<'a> {
    let width =

    let dimensions = Dimensions {
        x: 0,
        y: 0,
        width,
        height
    }
}
