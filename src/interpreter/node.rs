#[allow(dead_code)]
#[derive(Debug)]
pub enum Ast {
    Command(Vec<String>),
    Pipe(Box<Ast>, Box<Ast>),
    Background(Box<Ast>, Box<Ast>),
    Logic(Box<Ast>, Box<Ast>, LogicType),
    Redirect(Box<Ast>, Box<Ast>, RedirectType),
}

#[derive(Debug, Clone)]
pub enum LogicType {
    And,
    Or
}

#[derive(Debug, Clone)]
pub enum RedirectType {
    Output,
    Anything,
    Input,
    Append,
    Error,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Token {
    Arg(String),
    Pipe,
    Background,
    Logic(LogicType),
    Redirect(RedirectType),
}
