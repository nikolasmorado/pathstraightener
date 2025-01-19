mod optimizer;
mod parser;
mod react_native;
mod tokenizer;
mod transpiler;

use optimizer::{optimize, squash};
use parser::parse;
use react_native::find_imports;
use tokenizer::tokenize;
use transpiler::transpile;

pub fn run(input: String, component_name: String, typescript: bool, react_native: bool) -> String {
    let mut tokens = tokenize(input);
    let node = parse(&mut tokens);
    let opt_ast = optimize(node, 0, (0.0, 0.0), String::from(""));
    let ast = squash(opt_ast);
    let mut imports: Vec<String> = vec![];
    if react_native {
        imports = find_imports(ast.clone(), Vec::<String>::new());
    }
    transpile(ast, 0, &component_name, typescript, react_native, imports)
}
