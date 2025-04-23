mod lexer;
mod types;
mod parser;

pub fn eval(input: &str) -> Result<(), String> {
    println!("{:?}", parser::Parser::new(&lexer::Lexer::new(input).lex()?).parse()?);
    return Ok(());
}
