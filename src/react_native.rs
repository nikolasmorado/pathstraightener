use crate::parser::{Node, NodeType};

pub fn find_imports(ast: Node, mut imports: Vec<String>) -> Vec<String> {
    if !matches!(ast.tag_type, NodeType::Text)
        && !imports.contains(&ast.tag_name.to_lowercase())
        && !ast.tag_name.starts_with("fe")
        && !(ast.tag_name.to_lowercase() == String::from("svg"))
    {
        imports.push(ast.tag_name.to_lowercase());
    }

    for child in ast.children {
        imports = find_imports(child, imports);
    }

    imports
}
