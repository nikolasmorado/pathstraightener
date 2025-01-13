use std::fs;
use psx::run;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file to process
    #[arg(value_name = "INPUT")]
    input: String,

    /// Component name (optional, defaults to "svg")
    #[arg(value_name = "COMPONENT_NAME", default_value = "svg")]
    component_name: String,

    #[arg(short = 't', long = "typescript")]
    typescript: bool,
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

    let c_name = if let Some(first_char) = args.component_name.chars().next() {
        format!(
            "{}{}",
            first_char.to_uppercase(),
            &args.component_name[first_char.len_utf8()..]
        )
    } else {
        String::new()
    };

    let res = run(file_content, c_name, args.typescript);

    println!("{}", res);
}

