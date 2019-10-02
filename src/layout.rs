use crate::style;
use crate::css;
use crate::style::StyledNode;

#[derive(Debug)]
pub struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType,
    styled_node: &'a StyledNode<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Debug)]
enum BoxType {
    BlockNode,
    InlineNode,
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
    bottom: f32,
    left: f32,
    right: f32,
}

impl Dimensions {
    fn padding_box(&self) -> Rect {
        self.content.expanded_by(&self.padding)
    }
    fn border_box(&self) -> Rect {
        self.padding_box().expanded_by(&self.border)
    }
    fn margin_box(&self) -> Rect {
        self.border_box().expanded_by(&self.margin)
    }
}
impl Rect {
    fn expanded_by(&self, edges: &EdgeSizes) -> Rect {
        Rect {
            x: self.x - edges.left,
            y: self.y - edges.top,
            width: self.width + edges.left + edges.right,
            height: self.height + edges.top + edges.bottom,
        }
    }
}

impl LayoutBox<'_> {
    fn new<'a>(styled_node: &'a StyledNode) -> LayoutBox<'a> {
        let box_type =  match styled_node.get_display() {
            style::Display::Block => BoxType::BlockNode,   
            style::Display::Inline => BoxType::InlineNode, 
            style::Display::None => panic!("Cannot build layout box from node with no display."), 
        };
        LayoutBox{
            dimensions: Default::default(),
            box_type: box_type,
            styled_node: styled_node,
            children: Vec::new(),
        }
    }

    fn compute_dimensions(&mut self, dim_parent: &Dimensions) {
        match &self.box_type {
            BoxType::BlockNode => self.compute_block_dimensions(dim_parent),
            BoxType::InlineNode => {},
            BoxType::AnonymousBlock => {},
        }
    }

    fn compute_block_dimensions(&mut self, dim_parent: &Dimensions) {
        self.compute_block_width(dim_parent);
        self.compute_block_position(dim_parent);
        for child in &mut self.children {
            child.compute_dimensions(&self.dimensions);
            self.dimensions.content.height += child.dimensions.margin_box().height;
            //TODO change height of parent after knowing height of child
        }
        self.compute_block_height();
    }

    fn compute_block_width(&mut self, dim_parent: &Dimensions) {
        //TODO clean this function
        let style = &self.styled_node;
        let dims = &mut self.dimensions;
        
        // If width not precised, set to auto
        let auto = css::Value::Keyword("auto".to_string());
        let width = style.get_property("margin-left").unwrap_or(&auto);

        // If margin/border/padding not precised, set to 0
        let zero = css::Value::Length(0.0, css::Unit::Px);

        let mut margin_left = style.get_property("margin-left").unwrap_or(&zero);
        let mut margin_right = style.get_property("margin-right").unwrap_or(&zero);
        let border_left = style.get_property("border-left").unwrap_or(&zero);
        let border_right = style.get_property("border-right").unwrap_or(&zero);
        let padding_left = style.get_property("padding-left").unwrap_or(&zero);
        let padding_right = style.get_property("padding-right").unwrap_or(&zero);

        let total: f32 = [&margin_left, &margin_right, &border_left, &border_right,
                     &padding_left, &padding_right, &width].iter().map(|l| l.to_px()).sum();

        let underflow = dim_parent.content.width - total;

        if width != &auto && underflow < 0.0 {  // if overflows
            if margin_left == &auto {
                margin_left = &css::Value::Length(0.0, css::Unit::Px);
            }
            if margin_right == &auto {
                margin_right = &css::Value::Length(0.0, css::Unit::Px);
            }
        }

        dims.content.width = width.to_px();
        dims.padding.left = padding_left.to_px();
        dims.padding.right = padding_right.to_px();
        dims.border.left = border_left.to_px();
        dims.border.right = border_right.to_px();
        dims.margin.left = margin_left.to_px();
        dims.margin.right = margin_right.to_px();

        match (width == &auto, margin_left == &auto, margin_right == &auto) {
            (false, false, false) => {
                dims.margin.right = margin_right.to_px() + underflow;
            },
            (false, true, false) => {
                dims.margin.left = underflow;
            },
            (false, false, true) => {
                dims.margin.right = underflow;
            },
            (false, true, true) => {
                dims.margin.left = underflow / 2.0;
                dims.margin.right = underflow / 2.0;
            },
            (true, _, _) => {
                if margin_left == &auto { dims.margin.left = 0.0; }    
                if margin_right == &auto { dims.margin.right = 0.0; }    

                if underflow >= 0.0 {
                    dims.content.width = underflow;
                } else {
                    dims.content.width = 0.0;
                    dims.margin.right = margin_right.to_px() + underflow;
                }
            },
        }
    }
    fn compute_block_position(&mut self, dim_parent: &Dimensions) {
        let style = &self.styled_node;
        let dims = &mut self.dimensions;
        // If margin/border/padding not precised, set to 0
        let zero = css::Value::Length(0.0, css::Unit::Px);

        dims.margin.top = style.get_property("margin-top").unwrap_or(&zero).to_px();
        dims.margin.bottom = style.get_property("margin-bottom").unwrap_or(&zero).to_px();
        dims.border.top = style.get_property("border-top").unwrap_or(&zero).to_px();
        dims.border.bottom = style.get_property("border-bottom").unwrap_or(&zero).to_px();
        dims.padding.top = style.get_property("padding-top").unwrap_or(&zero).to_px();
        dims.padding.bottom = style.get_property("padding-bottom").unwrap_or(&zero).to_px();

        dims.content.x = dim_parent.content.x + 
            dims.margin.left + dims.border.left + dims.padding.left;
        // Place block below all other nodes in parent. Parent's height updated each time a new
        // child is added.
        dims.content.y = dim_parent.content.height + dim_parent.content.y +
            dims.margin.top + dims.border.top + dims.padding.top;
    }
    fn compute_block_height(&mut self) {
        if let Some(h) = self.styled_node.get_property("height") {
            self.dimensions.content.height = h.to_px();
        }
    }
}

// To build the layout tree, traverse StyleTree and imbricate LayoutBoxes
pub fn build_layout_tree<'a>(node: &'a StyledNode) -> LayoutBox<'a> {
    let mut ret = LayoutBox::new(node);
    for style_child in &node.children {
        match style_child.get_display() {
            style::Display::None => {},
            _ => ret.children.push(build_layout_tree(style_child)),
        }
    }
    //TODO get browser dimensions
    let browser_dims = Dimensions{
        content: Rect{x: 1., y: 1., width: 1., height:0.},
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };
    ret.compute_dimensions(&browser_dims);
    ret
}
