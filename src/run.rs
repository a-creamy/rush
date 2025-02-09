mod bic;
mod executor;
mod node;
mod parser;

pub fn execute(input: &str) {
    match executor::execute(parser::parse(input)) {
        Ok(()) => (),
        Err(e) => eprintln!("{e}")
    }
}
