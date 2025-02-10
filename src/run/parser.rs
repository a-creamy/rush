use crate::run::node;
use std::path::PathBuf;

pub fn parse(input: &str) -> node::AST {
    let and_groups: Vec<&str> = input.split("&&").collect();
    let mut commands = Vec::new();

    for group in and_groups {
        let ast = parse_single_command(group.trim());
        commands.push(ast);
    }

    if commands.len() == 1 {
        commands.pop().unwrap()
    } else {
        node::AST::AndList(commands)
    }
}

pub fn parse_single_command(input: &str) -> node::AST {
    let mut parts = input.split_whitespace().peekable();
    let mut commands = Vec::new();
    let mut current_cmd = Vec::new();
    let mut output_file = None;
    let mut append = false;
    let mut background = false;

    while let Some(token) = parts.next() {
        match token {
            "|" => {
                if !current_cmd.is_empty() {
                    commands.push(current_cmd);
                    current_cmd = Vec::new();
                }
            }
            ">" | ">>" => {
                if let Some(file) = parts.next() {
                    output_file = Some(PathBuf::from(file));
                    append = token == ">>";
                }
            }
            "&" => {
                if parts.peek().is_none() {
                    background = true;
                } else {
                    current_cmd.push(token.to_string());
                }
            }
            _ => {
                current_cmd.push(token.to_string());
            }
        }
    }

    if !current_cmd.is_empty() {
        commands.push(current_cmd);
    }

    if commands.len() == 1 {
        node::AST::Command(commands.remove(0), output_file.map(|f| (f, append)), background)
    } else {
        node::AST::Pipeline(commands, output_file.map(|f| (f, append)), background)
    }
}
