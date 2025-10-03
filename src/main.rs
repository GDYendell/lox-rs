use clap::Parser;

mod lexer;
use lexer::Lexer;

use crate::{ast_display::AstDisplay, expression::{Binary, Expr, Grouping, Literal, Unary}};

mod ast_display;
mod expression;
mod token;

#[derive(clap::Parser)]
struct Args {

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Lex {
        file: String,
    },
    PrintAst,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    if let Commands::Lex { file } = args.cmd {
        println!("Lexing '{}'", file);

        let Ok(file_contents) = std::fs::read_to_string(&file) else {
            println!("Failed to read file {}", file);
            return Err("Failed to read file".to_string());
        };

        println!("Tokens:");

        let tokens = Lexer::new(file_contents).scan_tokens();
        for token in tokens {
            println!(" {:?}", token?);
        }
    } else if let Commands::PrintAst = args.cmd {
        let expression = Expr::BinaryExpr(Binary::new(
            Expr::LiteralExpr(Literal::String("one".to_string())),
            token::Token::from(token::TokenKind::Plus),
            Expr::LiteralExpr(Literal::String("two".to_string())),
        ));
        println!("{}", expression.ast());

        let expression = Expr::UnaryExpr(Unary::new(
            token::Token::from(token::TokenKind::Minus),
            Expr::LiteralExpr(Literal::Number(1.0)),
        ));
        println!("{}", expression.ast());

        let expression = Expr::BinaryExpr(Binary::new(
            Expr::UnaryExpr(Unary::new(
                token::Token::from(token::TokenKind::Minus),
                Expr::LiteralExpr(Literal::Number(123.0)),
            )),
            token::Token::from(token::TokenKind::Star),
            Expr::GroupingExpr(Grouping::new(
                Expr::LiteralExpr(Literal::Number(45.67)),
            )),
        ));
        println!("{}", expression.ast());
    }

    Ok(())
}
