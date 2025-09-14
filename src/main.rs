use clap::Parser;

mod lexer;
use lexer::scan_tokens;

mod tokens;

#[derive(clap::Parser)]
struct Cli {
    file: String,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    println!("Lexing '{}'", args.file);

    let Ok(file_contents) = std::fs::read_to_string(&args.file) else {
        println!("Failed to read file {}", args.file);
        return Err("Failed to read file".to_string());
    };

    println!("Tokens:");

    let tokens = scan_tokens(&file_contents);
    for token in tokens {
        println!(" {:?}", token?);
    }

    Ok(())
}
