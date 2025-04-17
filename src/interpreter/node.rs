#[derive(Debug)]
pub enum Ast {
    Command(Vec<String>),
    Pipe(Box<Ast>, Box<Ast>),
    Background(Box<Ast>, Box<Ast>),
    Logic(Box<Ast>, Box<Ast>, LogicType),
    Redirect(Box<Ast>, Box<Ast>, RedirectType),
    Separator(Box<Ast>, Box<Ast>),
}

#[derive(Debug, Clone)]
pub enum LogicType {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum RedirectType {
    Overwrite,
    Input,
    Append,
    Error,
}

#[derive(Debug, Clone)]
pub enum Token {
    Arg(String),
    Pipe,
    Background,
    Logic(LogicType),
    Redirect(RedirectType),
    Separator,
}
