pub enum Ast {
    Command(Vec<String>),
    Pipeline(Box<Ast>, Box<Ast>),
    AndLogical(Box<Ast>, Box<Ast>),
    OverwriteRedirection(Box<Ast>, Box<Ast>),
    AppendRedirection(Box<Ast>, Box<Ast>),
    ErrorRedirection(Box<Ast>, Box<Ast>),
}

pub enum Token {
    Arg(String),
    And,
    Pipe,
    OverwriteRedirection,
    AppendRedirection,
    ErrorRedirection,
}
