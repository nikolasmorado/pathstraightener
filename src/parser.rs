use crate::tokenizer::Token;
use core::panic;
use std::{self, collections::HashMap};

// implementing this AST https://github.com/syntax-tree/hast

#[derive(Debug, Clone)]
pub enum NodeType {
    Element,
    Text,
}

#[derive(Debug, Clone)]
pub struct Node {
    tag_type: NodeType,
    tag_name: String,
    value: Option<String>,
    properties: HashMap<String, String>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(
        tag_type: NodeType,
        tag_name: String,
        value: Option<String>,
        properties: HashMap<String, String>,
        children: Vec<Node>,
    ) -> Self {
        Self {
            tag_type,
            tag_name,
            value,
            properties,
            children,
        }
    }
}

pub fn parse(tokens: &mut Vec<Token>) -> Vec<Node> {
    if tokens.is_empty() {
        panic!("Unexpected end of tokens");
    }

    let mut stack = Vec::<Node>::new();

    loop {
        if tokens.len() == 0 {
            break;
        }
        let next = tokens.remove(0);
        match next {

            Token::TagOpen(tag_name) => {
                let name = String::from(tag_name);

                let node = Node::new(
                    NodeType::Element,
                    name.clone(),
                    None,
                    HashMap::new(),
                    Vec::new(),
                );

                stack.push(node.clone());
            }

            Token::TagEnd(tag_name) => {
                if let Some(node) = stack.pop() {
                    if node.tag_name != tag_name {
                        panic!(
                            "Expected closing tag for {}, got closing tag for {}",
                            node.tag_name, tag_name
                        )
                    } else {
                        if let Some(mut parent) = stack.pop() {
                            parent.children.push(node.clone());
                            stack.push(parent);
                        } else {
                            stack.push(node.clone());
                        }
                    }
                } else {
                    panic!(
                        "Found closing tag for {} with no related opening node",
                        tag_name
                    )
                }
            }

            Token::Attribute(attr_name) => {
                let mut quotes_found = 0;
                let mut eq_found = false;
                let mut value = String::new();

                loop {
                    if tokens.len() == 0 {
                        break;
                    }

                    if let Some(nx) = tokens.get(0) {

                        match nx {
                            Token::TagClose => {
                                if !eq_found && quotes_found == 0 {
                                    value = String::from("true");
                                } else if eq_found && quotes_found == 1 {
                                    value += ">"
                                } else {
                                    panic!("Attribute {:?} not terminated correctly", attr_name)
                                }
                            }
                            Token::Attribute(text) => {
                                if quotes_found == 0 {
                                    value = String::from("true");
                                    break;
                                } else if quotes_found == 1 && eq_found {
                                    value += text
                                } else {
                                    panic!("Attribute {:?} not terminated correctly", attr_name)
                                }
                            }
                            Token::AssignTo => {
                                if eq_found && quotes_found == 0 {
                                    panic!(
                                        "Attribute {:?} has two equals before opening quote",
                                        attr_name
                                    )
                                } else if !eq_found && quotes_found == 0 {
                                    eq_found = true;
                                } else if eq_found && quotes_found == 1 {
                                    value += "="
                                } else {
                                    panic!(
                                        "Something unexpected happened with an equals and tag {:?}",
                                        attr_name
                                    )
                                }
                            }
                            Token::QuoteOpen => {
                                if eq_found && quotes_found == 0 {
                                    quotes_found += 1;
                                } else {
                                    panic!("Quote found after attribute {:?} without assignment operator", attr_name)
                                }
                            }
                            Token::QuoteClose => {
                                if eq_found && quotes_found == 1 {
                                    break;
                                } else {
                                    panic!("Closing quote found for attribute {:?}", attr_name);
                                }
                            }
                            Token::Text(text) => {
                                if eq_found && quotes_found == 1 {
                                    value += text;
                                } else {
                                    panic!("Text found after attribute {:?} without either an assignment or opening quote", attr_name)
                                }
                            }
                            Token::TagOpen(text) => {
                                if eq_found && quotes_found == 1 {
                                    value += text
                                } else {
                                    panic!("Opening tag found where it shouldnt be {:?}", attr_name)
                                }
                            }
                            Token::TagEnd(text) => {
                                if eq_found && quotes_found == 1 {
                                    value += text
                                } else {
                                    panic!("Opening tag found where it shouldnt be {:?}", attr_name)
                                }
                            }
                        }
                    }

                    tokens.remove(0);
                }

                if let Some(mut parent) = stack.pop() {
                    parent.properties.insert(attr_name, String::from(value));
                    stack.push(parent);
                } else {
                    panic!(
                        "Tried to add attribute {:?}={:?} but no node to attatch it to",
                        attr_name, value
                    )
                }
            }

            // Token::

            _ => {
                continue;
            }
        }
    }

    stack
}
