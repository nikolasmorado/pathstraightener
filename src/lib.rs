mod optimizer;
mod parser;
mod tokenizer;
mod transpiler;

use optimizer::optimize;
use parser::parse;
use tokenizer::tokenize;
use transpiler::transpile;

pub fn run(input: String, component_name: String, typescript: bool) -> String {
    let mut tokens = tokenize(input);
    let node = parse(&mut tokens);
    let ast = optimize(node, 0);
    transpile(ast, 0, &component_name, typescript)
}
