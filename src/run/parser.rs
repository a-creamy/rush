use crate::run::node;
use std::path::PathBuf;

pub fn parse(input: &str) -> node::AST {
    let mut parts = input.split_whitespace().peekable();
    let mut commands = Vec::new();
    let mut current_cmd = Vec::new();
    let mut output_file = None;
    let mut append = false;

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
            _ => {
                current_cmd.push(token.to_string());
            }
        }
    }

    if !current_cmd.is_empty() {
        commands.push(current_cmd);
    }

    if commands.len() == 1 {
        return node::AST::Command(commands.remove(0), output_file.map(|f| (f, append)));
    } else {
        return node::AST::Pipeline(commands, output_file.map(|f| (f, append)));
    }
}
