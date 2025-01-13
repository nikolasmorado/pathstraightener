use crate::parser::{Node, NodeType};

pub fn transpile(ast: Node, depth: u8) -> String {
    let mut res = String::new();

    let prefix = "\t".repeat(depth as usize);

    if depth == 0 {
        res.push_str(r###"import * as React from "react""###);
        res.push('\n');
        res.push_str("const SVG = (props) => {");
        res.push('\n');
    }

    match ast.tag_type {
        NodeType::Element => {
            res.push_str(&prefix);
            res.push_str("<");
            res.push_str(&ast.tag_name);

            for p in ast.properties {
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

            res.push_str(">");
            res.push('\n');

            for c in ast.children {
                res.push_str(&transpile(c, depth + 1));
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
        res.push_str("export default SVG;");
    }

    return res;
}

