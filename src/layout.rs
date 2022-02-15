use styling::*;

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct Dimensions {
    content: Rect,
}

enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}

struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}

enum Display {
    Inline,
    Block,
    None,
}

pub fn create_layout_tree<'a>(root_node: &'a StyledNode) -> LayoutBox<'a> {}
