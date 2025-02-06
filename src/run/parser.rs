use crate::run::node;

pub fn parse(input: &str) -> node::AST {
    let commands: Vec<Vec<String>> = input
        .split("|")
        .map(|cmd| cmd.split_whitespace().map(String::from).collect())
        .collect();

    if commands.len() == 1 {
        node::AST::Command(commands.into_iter().next().unwrap())
    } else {
        node::AST::Pipeline(commands)
    }
}
