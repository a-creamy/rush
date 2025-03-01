/*
#[derive(Debug)]
pub enum Ast {
    Command(Vec<String>),
    Pipe(Box<Ast>, Box<Ast>),
    Background(Box<Ast>),
    AndLogical(Box<Ast>, Box<Ast>),
    OrLogical(Box<Ast>, Box<Ast>),
    Redirect(Box<Ast>, Box<Ast>, RedirectType),
}
*/

#[derive(Debug)]
pub enum RedirectType {
    Output,
    Anything,
    Input,
    Append,
    Error,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Arg(String),
    Pipe,
    Background,
    AndLogical,
    OrLogical,
    Redirect(RedirectType),
}
