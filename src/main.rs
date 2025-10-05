use clap::Parser as ClapParser;

mod token;
use token::{Token, TokenKind};

mod lexer;
use lexer::Lexer;

mod parser;
use parser::Parser;

use crate::{ast_display::AstDisplay, expression::{Binary, Expr, Grouping, Literal, Unary}};

mod ast_display;
mod expression;

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
    Parse {
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
    } else if let Commands::Parse { file } = args.cmd {
        println!("Parsing '{}'", file);

        let Ok(file_contents) = std::fs::read_to_string(&file) else {
            println!("Failed to read file {}", file);
            return Err("Failed to read file".to_string());
        };

        println!("AST:");
        let tokens = Lexer::new(file_contents).scan_tokens();
        let errors = tokens.iter().filter_map(|token| token.as_ref().err()).collect::<Vec<_>>();
        dbg!(&errors);
        let tokens: Vec<token::Token> = tokens.into_iter().filter_map(|token| token.ok()).collect();
        dbg!(&tokens);
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        println!("{}", expression.ast());

    } else if let Commands::PrintAst = args.cmd {
        let expression = Expr::BinaryExpr(Binary::new(
            Expr::LiteralExpr(Literal::String("one".to_string())),
            Token::from(TokenKind::Plus),
            Expr::LiteralExpr(Literal::String("two".to_string())),
        ));
        println!("{}", expression.ast());

        let expression = Expr::UnaryExpr(Unary::new(
            Token::from(TokenKind::Minus),
            Expr::LiteralExpr(Literal::Number(1.0)),
        ));
        println!("{}", expression.ast());

        let expression = Expr::BinaryExpr(Binary::new(
            Expr::UnaryExpr(Unary::new(
                Token::from(TokenKind::Minus),
                Expr::LiteralExpr(Literal::Number(123.0)),
            )),
            Token::from(TokenKind::Star),
            Expr::GroupingExpr(Grouping::new(
                Expr::LiteralExpr(Literal::Number(45.67)),
            )),
        ));
        println!("{}", expression.ast());
    }

    Ok(())
}
