use crate::parser::{Node, NodeType};

pub fn optimize(mut ast: Node, depth: u8, translate: (f32, f32), fill: String) -> Node {
    let mut t: (f32, f32) = (translate.0, translate.1);
    let mut f: String = fill;

    let mut updated_properties = ast.properties.clone();

    for (key, value) in ast.properties.iter() {
        if key == "transform" && value.starts_with("translate(") && value.ends_with(")") {
            let transform_value = value[10..value.len() - 1].to_string();
            let components: Vec<&str> = transform_value.split(',').collect();

            for (i, component) in components.iter().enumerate() {
                if let Ok(parsed) = component.trim().parse::<f32>() {
                    match i {
                        0 => t.0 = parsed,
                        1 => t.1 = parsed,
                        _ => break,
                    }
                }
            }

            updated_properties.remove("transform");
        }

        if key == "fill" {
            f = value.to_string();
        }
    }

    let mut propagating_fill = false;

    for child in ast.children.clone() {
        if !child.properties.contains_key("fill") {
            propagating_fill = true;
        }
    }

    if !propagating_fill {
        updated_properties.insert(String::from("fill"), f.clone());
    } else {
        updated_properties.remove("fill");
    }

    ast.properties = updated_properties;

    if ast.children.is_empty() && (t.1 != 0.0 || t.0 != 0.0) {
        let transform_value = format!("translate({}, {})", t.0, t.1);
        ast.properties
            .insert("transform".to_string(), transform_value);
    } else {
        ast.children = ast
            .children
            .into_iter()
            .map(|child| optimize(child, depth + 1, t, f.clone()))
            .collect();
    }

    ast
}

pub fn squash(mut ast: Node) -> Node {
    if !ast.children.is_empty() {
        let mut children = Vec::<Node>::new();

        for child in ast.children.into_iter().map(squash) {
            if child.tag_name == "g"
                && child.properties.len() == 1
                && child.properties.contains_key("id")
            {
                children.extend(child.children);
            } else if (child.tag_name == "defs" || child.tag_name == "g")
                && child.children.len() == 0
            {
                continue;
            } else if (child.tag_name == "desc" || child.tag_name == "title")
                && child.children.len() == 1
                && matches!(child.children[0].tag_type, NodeType::Text)
            {
                continue;
            } else {
                children.push(child);
            }
        }

        ast.children = children;
    }

    ast
}
