use crate::parser::{Node, NodeType};

pub fn transpile(ast: Node, depth: u8, component_name: &str, typescript: bool) -> String {
    let mut res = String::new();

    let prefix = "\t".repeat(1 + depth as usize);

    if depth == 0 {
        res.push_str(r###"import * as React from "react""###);
        res.push('\n');
        if typescript {
            res.push_str(r###"import { SVGProps } from "react""###);
            res.push('\n');
        }
        res.push_str("const ");
        res.push_str(component_name);
        if typescript {
            res.push_str(" = (props: SVGProps<SVGSVGElement>) => {");
        } else {
            res.push_str(" = (props) => {");
        }
        res.push('\n');
    }

    match ast.tag_type {
        NodeType::Element => {
            res.push_str(&prefix);
            res.push_str("<");
            res.push_str(&ast.tag_name);

            let mut properties: Vec<_> = ast
                .properties
                .iter()
                .map(|(key, value)| {
                    let camel_case_key: String = key
                        .chars()
                        .enumerate()
                        .fold(
                            (String::new(), false),
                            |(mut result, mut capitalize_next), (_i, ch)| {
                                if ch == '-' || ch == '_' || ch == ':' {
                                    capitalize_next = true;
                                } else if capitalize_next {
                                    result.push(ch.to_ascii_uppercase());
                                    capitalize_next = false;
                                } else {
                                    result.push(ch);
                                }
                                (result, capitalize_next)
                            },
                        )
                        .0;
                    (camel_case_key, value.clone())
                })
                .collect();
            properties.sort_by(|a, b| a.0.cmp(&b.0));

            for p in properties {
                res.push_str(" ");
                res.push_str(&p.0);
                res.push_str("={");
                if p.1 != "true" && p.1 != "false" && p.1.parse::<f64>().is_err() {
                    res.push('"');
                    res.push_str(&p.1);
                    res.push('"');
                } else {
                    res.push_str(&p.1);
                }
                res.push_str("}");
            }

            if depth == 0 {
                res.push_str(" {...props}");
            }

            res.push_str(">");
            res.push('\n');

            for c in ast.children {
                res.push_str(&transpile(c, depth + 1, component_name, typescript));
                res.push('\n');
            }

            res.push_str(&prefix);
            res.push_str("</");
            res.push_str(&ast.tag_name);
            res.push_str(">");
        }
        NodeType::Text => {
            res.push_str(&prefix);
            res.push_str(&ast.value.unwrap());
        }
    }

    if depth == 0 {
        res.push('\n');
        res.push_str("}");
        res.push('\n');
        res.push_str("export default ");
        res.push_str(component_name);
        res.push_str(";");
    }

    return res;
}
