mod optimizer;
mod parser;
mod tokenizer;
mod transpiler;

use std::fs;

use clap::Parser;
use optimizer::optimize;
use parser::parse;
use tokenizer::tokenize;
use transpiler::transpile;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file to process
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Cli::parse();

    let file_content = match fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", args.input, e);
            std::process::exit(1);
        }
    };

    let mut tokens = tokenize(file_content);
    let node = parse(&mut tokens);
    let ast = optimize(node, 0);
    let res = transpile(ast, 0);

    println!("{}", res);
}
