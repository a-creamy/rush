pub enum Ast {
    Command(Vec<String>),
    Pipe(Box<Ast>, Box<Ast>),
    AndLogical(Box<Ast>, Box<Ast>),
    OrLogical(Box<Ast>, Box<Ast>),
    Background(Box<Ast>, Box<Ast>),
    Separator(Box<Ast>, Box<Ast>),
    OutputRedirection(Box<Ast>, Box<Ast>),
    InputRedirection(Box<Ast>, Box<Ast>),
    OverwriteRedirection(Box<Ast>, Box<Ast>),
    AppendRedirection(Box<Ast>, Box<Ast>),
    ErrorRedirection(Box<Ast>, Box<Ast>),
}

pub enum Token {
    Arg(String),
    Pipe,
    AndLogical,
    OrLogical,
    Background,
    Separator,
    OutputRedirection,
    InputRedirection,
    OverwriteRedirection,
    AppendRedirection,
    ErrorRedirection,
}
