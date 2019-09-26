use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

#[derive(Debug)]
enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text_node(text: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(text),
    }
}

pub fn element_node(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn pretty_fmt(node: &Node, prefix: String, f: &mut fmt::Formatter) {
            match &node.node_type {
                NodeType::Text(text) => {
                    let to_write = if text.len() > 50 {text[..50].to_string()} else {text.clone()};
                    writeln!(f, "{}{}", prefix, to_write);
                },
                NodeType::Element(ElementData{tag_name, attributes}) => {
                    let mut str_attrs = "".to_string();
                    for (attr, value) in attributes {
                        if !str_attrs.is_empty() {str_attrs.push_str(", ")}
                        str_attrs.push_str(&format!("{}={}", attr, value))
                    }
                    writeln!(f, "{}{} ({})", prefix, tag_name, str_attrs);
                },
            }
            for child in &node.children {
                let mut prefix = prefix.clone();
                prefix.push_str("    ");
                pretty_fmt(child, prefix, f);
            }
        }
        pretty_fmt(&self, "".to_string(), f);
        Ok(())
    }
}
