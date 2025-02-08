mod bic;
mod executor;
mod node;
mod parser;

pub fn execute(input: &str) {
    executor::execute(parser::parse(input));
}
