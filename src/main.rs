use Expr::Plus;
use crate::evaluation::eval;
use crate::parser::{Expr};
use crate::parser::Expr::Num;

mod parser;
mod evaluation;

fn main() {
    // TODO make the parser... Manually build ASTs for now
    // let expr = "1 + 2";
    // let ast = parse(expr);
    let ast = Plus(Box::from(Num(1)), Box::from(Num(2)));
    println!("AST: {}", ast);

    let value = eval(&ast);
    println!("Evaluation: {}", value);
}

