use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }
    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(str_classes) => str_classes.split_whitespace().collect(),
            None => HashSet::new(),
        }
    }
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
        fn pretty_fmt(node: &Node, prefix: String, f: &mut fmt::Formatter) -> fmt::Result {
            match &node.node_type {
                NodeType::Text(text) => {
                    let to_write = if text.len() > 50 {text[..50].to_string()} else {text.clone()};
                    writeln!(f, "{}{}", prefix, to_write)?;
                },
                NodeType::Element(ElementData{tag_name, attributes}) => {
                    let mut str_attrs = "".to_string();
                    for (attr, value) in attributes {
                        if !str_attrs.is_empty() {str_attrs.push_str(", ")}
                        str_attrs.push_str(&format!("{}={}", attr, value))
                    }
                    writeln!(f, "{}{} ({})", prefix, tag_name, str_attrs)?;
                },
            }
            for child in &node.children {
                let mut prefix = prefix.clone();
                prefix.push_str("    ");
                pretty_fmt(child, prefix, f)?;
            }
            Ok(())
        }
        pretty_fmt(&self, "".to_string(), f)?;
        Ok(())
    }
}
