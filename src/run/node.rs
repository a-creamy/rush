use std::path::PathBuf;

pub enum AST {
    Command(Vec<String>, Option<(PathBuf, bool)>),
    Pipeline(Vec<Vec<String>>, Option<(PathBuf, bool)>),
    AndList(Vec<AST>)
}
