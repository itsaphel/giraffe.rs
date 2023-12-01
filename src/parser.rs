use std::fmt::{Display, Formatter};

pub(crate) enum Expr {
    Num(u32),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(print_expr(self).as_str())
    }
}

fn print_expr(e: &Expr) -> String {
    match e {
        Expr::Num(n) => n.to_string(),
        Expr::Plus(e1, e2) => format!("( {} + {} )", print_expr(e1), print_expr(e2)),
        Expr::Minus(e1, e2) => format!("( {} + {} )", print_expr(e1), print_expr(e2)),
        Expr::Times(e1, e2) => format!("( {} + {} )", print_expr(e1), print_expr(e2)),
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct PestParser;

pub(crate) fn parse(input: &str) -> Expr {
    // TODO impl
}