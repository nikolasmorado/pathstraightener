mod parser;
mod tokenizer;

use parser::parse;
use tokenizer::tokenize;

fn main() {
    let a = String::from(r#"<a><p></p><p></p></a>"#);
    // let a = String::from(r#"<div class="tester"> "womp womp" the big cow said </div>"#);
    let mut tokens = tokenize(a);

    let node = parse(&mut tokens);
    println!("AST HERE: {:?}", node);
}
