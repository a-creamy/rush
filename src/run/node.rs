pub enum AST {
    Command(Vec<String>),
    Pipeline(Vec<Vec<String>>),
}
