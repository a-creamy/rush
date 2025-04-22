mod lexer;
mod types;
mod parser;

pub fn eval(input: &str) -> Result<(), String> {
    let mut lexer = lexer::Lexer::new(input);
    println!("{:?}", lexer.lex()?);
    return Ok(());
}
