use std::fmt;

use crate::style;
use crate::style::StyledNode;

#[derive(Debug)]
pub struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType,
    styled_node: &'a StyledNode<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

impl LayoutBox<'_> {
    fn new<'a>(styled_node: &'a StyledNode) -> LayoutBox<'a> {
        let box_type =  match styled_node.get_display() {
            style::Display::Inline => BoxType::InlineNode, 
            style::Display::Block => BoxType::BlockNode,   
            style::Display::None => panic!("Cannot build layout box from node with no display."), 
        };
        LayoutBox{
            dimensions: Default::default(),
            box_type: box_type,
            styled_node: styled_node,
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum BoxType {
    InlineNode,
    BlockNode,
    AnonymousBlock,
}

#[derive(Debug, Default)]
struct Dimensions {
   content: Rect, 
   padding: EdgeSizes, 
   border: EdgeSizes, 
   margin: EdgeSizes, 
}

#[derive(Debug, Default)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
#[derive(Debug, Default)]
struct EdgeSizes {
    top: f32,
    down: f32,
    left: f32,
    right: f32,
}

// To build the layout tree, traverse StyleTree and imbricate LayoutBoxes
pub fn build_layout_tree<'a>(node: &'a StyledNode) -> LayoutBox<'a> {
    let mut ret = LayoutBox::new(node);
    for style_child in &node.children {
        match style_child.get_display() {
            style::Display::None => (),
            _ => ret.children.push(build_layout_tree(style_child)),
        }
    }
    ret
}
