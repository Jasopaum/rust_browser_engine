use std::fmt;

use crate::dom;
use crate::css;

use std::collections::HashMap;

#[derive(Debug)]
pub struct StyledNode<'a> {
    html_node: &'a dom::Node,
    properties: Properties,
    pub children: Vec<StyledNode<'a>>,
}

pub enum Display {
    Inline,
    Block,
    None,
}

impl StyledNode<'_> {
    pub fn get_property(&self, prop: &str) -> Option<&css::Value> {
        self.properties.get(prop)
    }

    pub fn get_display(&self) -> Display {
        match self.get_property("display") {
            Some(css::Value::Keyword(val)) => match val.as_str() {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,   
            },
            _ => Display::Inline,
        }
    }
}

type Properties = HashMap<String, css::Value>;

fn simple_selector_matches_node(sel: &css::SimpleSelector, node: &dom::ElementData) -> bool {
    if sel.tag_name.iter().any(|sel_tag| &node.tag_name != sel_tag) {
        return false;
    }
    if sel.id.iter().any(|sel_id| node.id() != Some(sel_id)) {
        return false;
    }
    if sel.classes.iter().any(|sel_class| !node.classes().contains(sel_class.as_str())) {
        return false;
    }
    return true;
}

fn selectors_match_node(selectors: &Vec<css::Selector>, node: &dom::ElementData) -> bool {
    for sel in selectors {
        let res = match sel {
            css::Selector::Simple(sel) => simple_selector_matches_node(sel, &node)
        };
        if res { return true; }
    }
    return false;
}

fn get_matching_rules<'a>(node: &dom::ElementData, sheet: &'a css::StyleSheet) -> Vec<&'a css::Rule> {
    sheet.rules.iter()
               .filter(|rule| selectors_match_node(&rule.selectors, &node))
               .collect()
}

fn get_matching_properties(node: &dom::ElementData, sheet: &css::StyleSheet) -> Properties {
    let matching_rules = get_matching_rules(node, sheet);
    let mut ret_properties = HashMap::new();
    for rule in matching_rules {
        for declaration in &rule.declarations {
            ret_properties.insert(declaration.name.clone(), declaration.value.clone()); 
        }
    }
    ret_properties
}

pub fn build_style_tree<'a>(html_node: &'a dom::Node, sheet: &'a css::StyleSheet) -> StyledNode<'a> {
    StyledNode{
        html_node: html_node,
        properties: match &html_node.node_type {
            dom::NodeType::Element(element_data) => get_matching_properties(element_data, sheet),
            dom::NodeType::Text(_) => HashMap::new(),
        },
        children: html_node.children.iter()
                                    .map(|child| build_style_tree(child, sheet))
                                    .collect(),
    }
}

impl fmt::Display for StyledNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn pretty_fmt(node: &StyledNode, prefix: String, f: &mut fmt::Formatter) -> fmt::Result {
            match &node.html_node.node_type {
                dom::NodeType::Text(text) => {
                    let to_write = if text.len() > 50 {text[..50].to_string()} else {text.clone()};
                    writeln!(f, "{}{}", prefix, to_write)?;
                },
                dom::NodeType::Element(dom::ElementData{tag_name, ..}) => {
                    let mut str_properties = "".to_string();
                    for (name, value) in &node.properties {
                        if !str_properties.is_empty() {str_properties.push_str(", ")}
                        str_properties.push_str(&format!("{}={:?}", name, value))
                    }
                    writeln!(f, "{}{} ({})", prefix, tag_name, str_properties)?;
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
