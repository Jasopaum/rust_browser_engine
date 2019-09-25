use std::collections::HashMap;

use crate::dom;
use crate::dom::AttrMap;

struct Parser {
    source: String,
}

impl Parser {

    fn extract_name(&mut self) -> String {
        self.consume_spaces();
        let end_name = self.source.find(|c: char| !c.is_alphanumeric()).unwrap_or(self.source.len());
        self.source.drain(..end_name).collect() 
    }

    fn extract_attribute(&mut self) -> (String, String) {
        let name_attr = self.extract_name(); 

        self.consume_spaces();
        assert!(self.source.drain(..1).next() == Some('='));

        self.consume_spaces();
        assert!(self.source.drain(..1).next() == Some('"'));

        let value = self.extract_name(); 

        self.consume_spaces();
        assert!(self.source.drain(..1).next() == Some('"'));

        return (name_attr, value)
    }

    fn extract_attributes(&mut self) -> AttrMap {
        let mut attrs = HashMap::new();
        loop {
            self.consume_spaces();
            if self.source.starts_with('>') {
                self.source.drain(..1);
                break
            }
            let (attr, val) = self.extract_attribute();
            attrs.insert(attr, val);
        }
        return attrs;
    }

    fn parse_element(&mut self) -> dom::Node {
        // Remove the '<'
        self.source.drain(..1);
        let tag_name = self.extract_name();
        let attributes = self.extract_attributes();

        let children = self.parse_nodes();

        assert!(self.source.drain(..1).next() == Some('<')); 
        assert!(self.source.drain(..1).next() == Some('/')); 
        assert!(self.extract_name() == tag_name);
        assert!(self.source.drain(..1).next() == Some('>')); 
        
        dom::element_node(tag_name, attributes, children)
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
            if self.eof() || self.source.starts_with("</") {
                break
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    fn eof(&self) -> bool {
        self.source.is_empty()
    }
    fn consume_spaces(&mut self) {
        let end_space = self.source.find(|c: char| !c.is_whitespace()).unwrap_or(self.source.len());
        self.source.drain(..end_space);
    }
}

pub fn parse(source: String) -> Vec<dom::Node> {
    let nodes = Parser { source }.parse_nodes();

    return nodes;
}
