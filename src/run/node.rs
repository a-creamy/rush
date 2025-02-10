use std::path::PathBuf;

pub enum AST {
    Command(Vec<String>, Option<(PathBuf, bool)>, bool),
    Pipeline(Vec<Vec<String>>, Option<(PathBuf, bool)>, bool),
    AndList(Vec<AST>),
}
