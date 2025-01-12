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
    println!("TOKENS: {:?}", tokens);
    if tokens.is_empty() {
        panic!("Unexpected end of tokens");
    }

    let mut stack = Vec::<Node>::new();

    loop {
        println!("STACK: {:?}", stack);
        if tokens.len() == 0 {
            break;
        }
        println!("Tokens: {:?}", tokens);
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
                            println!("PARENT, {:?}", parent);
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

            _ => {
                continue;
            }
        }
    }

    stack
}


