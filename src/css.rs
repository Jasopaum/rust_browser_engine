#[derive(Debug)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes:Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Value
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
}

impl Value {
    pub fn to_px(&self) -> f32 {
        match self {
            //TODO take unit into account
            Value::Length(x, _) => *x,
            _ => 0.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Px,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn parse_identifier(source: &mut String) -> String {
    let end_name = source.find(|c: char| !c.is_alphanumeric()).unwrap_or(source.len());
    source.drain(..end_name).collect()
}
fn parse_declaration_name(source: &mut String) -> String {
    let end_name = source.find(|c: char| c == ':' || c.is_whitespace()).unwrap_or(source.len());
    source.drain(..end_name).collect()
}

fn parse_keyword_value(source: &mut String) -> Value {
    let end_name = source.find(|c: char| c == ';' || c.is_whitespace()).unwrap_or(source.len());
    let value_str = source.drain(..end_name).collect();
    Value::Keyword(value_str)
}

fn parse_color(source: &mut String) -> Value {
    assert!(source.drain(..1).next() == Some('#'));
    let r_str: String = source.drain(..2).collect();
    let r = u8::from_str_radix(&r_str, 16).unwrap();
    let g_str: String = source.drain(..2).collect();
    let g = u8::from_str_radix(&g_str, 16).unwrap();
    let b_str: String = source.drain(..2).collect();
    let b = u8::from_str_radix(&b_str, 16).unwrap();
    Value::Color(Color{r, g, b})
}

fn parse_length(source: &mut String) -> Value {
    let end_num = source.find(|c: char| c.is_alphabetic()).unwrap();
    let num = source.drain(..end_num).collect::<String>()
                                      .parse::<f32>()
                                      .unwrap();
    let end_unit = source.find(|c: char| !c.is_alphabetic()).unwrap();
    let _unit = source.drain(..end_unit).collect::<String>();
    //TODO parsed unit not considered

    Value::Length(num, Unit::Px)
}

fn parse_selector(source: &mut String) -> Selector {
    let mut selector = SimpleSelector{tag_name: None, id: None, classes: Vec::new()};
    match source.chars().next().unwrap() {
        '#' => {
            source.drain(..1);
            selector.id = Some(parse_identifier(source));
        },
        '.' => {
            source.drain(..1);
            selector.classes.push(parse_identifier(source));
        },
        c if c.is_alphabetic() => selector.tag_name = Some(parse_identifier(source)),
        _ => panic!("err wile parsing selector"),
    };
    return Selector::Simple(selector);
}
fn parse_selectors(source: &mut String) -> Vec<Selector> {
    let mut selectors = Vec::new();
    loop {
        consume_spaces(source);
        match source.chars().next().unwrap() {
            ',' => { let _ = source.drain(..1); consume_spaces(source); },
            '{' => { let _ = source.drain(..1); break; },
            _ => (),
        }
        selectors.push(parse_selector(source));
    }
    return selectors;
}

fn parse_declaration(source: &mut String) -> Declaration {
    let name = parse_declaration_name(source);
    consume_spaces(source);
    assert!(source.drain(..1).next() == Some(':'));

    // TODO can be other than keyword
    let value = match source.chars().next().unwrap() {
        '#' => {
            parse_color(source)
        },
        '0'..='9' => {
            parse_length(source)
        },
        _ => {
            parse_keyword_value(source)
        },
    };
    consume_spaces(source);
    assert!(source.drain(..1).next() == Some(';'));
    return Declaration{name, value}; 
}

fn parse_declarations(source: &mut String) -> Vec<Declaration> {
    let mut declarations = Vec::new();
    loop {
        consume_spaces(source);
        match source.chars().next().unwrap() {
            ';' => { let _ = source.drain(..1); },
            '}' => { let _ = source.drain(..1); break; },
            _ => (),
        }
        declarations.push(parse_declaration(source));
    }
    return declarations;
}

pub fn parse(source: &mut String) -> StyleSheet {
    let mut rules = Vec::new();
    loop {
        consume_spaces(source);
        if source.is_empty() {
            break;
        }
        let selectors = parse_selectors(source);
        let declarations = parse_declarations(source);
        rules.push(Rule{selectors, declarations});
    }
    StyleSheet{rules}
}

fn consume_spaces(source: &mut String) {
    let end_space = source.find(|c: char| !c.is_whitespace()).unwrap_or(source.len());
    source.drain(..end_space);
}
