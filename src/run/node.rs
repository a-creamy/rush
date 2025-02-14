pub enum AST {
    Command(Vec<String>),
    Pipeline(Box<AST>, Box<AST>),
    AndLogical(Box<AST>, Box<AST>),
    OverwriteRedirection(Box<AST>, Box<AST>),
    AppendRedirection(Box<AST>, Box<AST>),
}

pub enum Token {
    Arg(String),
    And,
    Pipe,
    OverwriteRedirection,
    AppendRedirection,
}
