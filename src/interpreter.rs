pub mod error;
pub mod executor;
pub mod lexer;
pub mod parser;
pub mod stream;

pub fn interpret(input: String) -> Result<(), error::ShellError> {
    let tokens = lexer::lex(input)?;
    let expr = parser::parse(&tokens)?;
    executor::execute(expr, None)?.wait()?;

    Ok(())
}
