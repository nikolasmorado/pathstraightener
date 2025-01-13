use std::fs;
use psx::run;

use clap::Parser;
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

    let res = run(file_content);

    println!("{}", res);
}
