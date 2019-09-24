use crate::dom;

struct Parser {
    source: String,
}

impl Parser {

    fn parse_element(&mut self) -> dom::Node {

    }

    fn parse_text(&mut self) -> dom::Node {
        let offset = self.source.find('<').unwrap_or(self.source.len());
        dom::text_node(self.source.drain(..offset).collect())    
    }

    fn parse_node(&mut self) -> dom::Node {
        if self.source.starts_with('<') {
            //self.parse_element()
            self.parse_text()
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
