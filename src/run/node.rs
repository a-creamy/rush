pub enum Ast {
    Command(Vec<String>),
    Pipe(Box<Ast>, Box<Ast>),
    AndLogical(Box<Ast>, Box<Ast>),
    OrLogical(Box<Ast>, Box<Ast>),
    OverwriteRedirection(Box<Ast>, Box<Ast>),
    AppendRedirection(Box<Ast>, Box<Ast>),
    ErrorRedirection(Box<Ast>, Box<Ast>),
}

pub enum Token {
    Arg(String),
    Pipe,
    AndLogical,
    OrLogical,
    OverwriteRedirection,
    AppendRedirection,
    ErrorRedirection,
}
