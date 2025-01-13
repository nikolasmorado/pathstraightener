use crate::parser::Node;

pub fn optimize(ast: Node, depth: u8) -> Node {
    for i in ast.clone().children {
        optimize(i, depth + 1);
    }
    return ast;
}
