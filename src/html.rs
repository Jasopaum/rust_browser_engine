use std::collections::HashMap;

use crate::dom;

struct Parser {
    source: String,
}

impl Parser {

    fn extract_tag_name(source: &mut String) -> String {
        let end_name = source.find(|c: char| !c.is_alphanumeric()).unwrap_or(source.len());
        source.drain(..end_name).collect() 
    }

    fn extract_attributes(source: &mut String) {
        println!("{}", source)
    }

    fn element_to_node(source: &mut String) -> dom::Node {
        source.drain(..1);
        let tag_name = Self::extract_tag_name(source);
        let attributes = Self::extract_attributes(source);
        dom::element_node(tag_name, HashMap::new(), Vec::new())
    }

    fn parse_element(&mut self) -> dom::Node {
        let offset = self.source.find('>').unwrap_or(self.source.len()) + 1;
        Self::element_to_node(&mut self.source.drain(..offset).collect())
    }

    fn parse_text(&mut self) -> dom::Node {
        let offset = self.source.find('<').unwrap_or(self.source.len());
        dom::text_node(self.source.drain(..offset).collect())    
    }

    fn parse_node(&mut self) -> dom::Node {
        if self.source.starts_with('<') {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            if self.eof() {
                break
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    fn eof(&self) -> bool {
        self.source.is_empty()
    }
}

pub fn parse(source: String) -> Vec<dom::Node> {
    let nodes = Parser { source }.parse_nodes();

    return nodes;
}
