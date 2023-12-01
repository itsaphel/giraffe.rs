use std::fmt::{Display, Formatter};
use crate::evaluation::Value::StringV;
use crate::parser::Expr;

pub(crate) enum Value {
    NumV(u32),
    StringV(String)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Value::NumV;

        match self {
            NumV(n) => f.write_str(format!("{}", n).as_str()),
            StringV(str) => f.write_str(format!("{}", str).as_str())
        }
    }
}

pub(crate) fn eval(e: &Expr) -> Value {
    use Expr::*;
    use Value::*;

    match e {
        Num(n) => NumV(*n),
        Plus(e1, e2) => Value::add(eval(e1), eval(e2)),
        Minus(e1, e2) => Value::sub(eval(e1), eval(e2)),
        Times(e1, e2) => Value::times(eval(e1), eval(e2)),
    }
}

impl Value {
    fn add(v1: Value, v2: Value) -> Value {
        use Value::NumV;

        match (v1, v2) {
            (NumV(v1), NumV(v2)) => NumV(v1 + v2),
            _ => panic!("unexpected args to add")
        }
    }

    fn sub(v1: Value, v2: Value) -> Value {
        use Value::NumV;

        let tuple = (v1, v2);
        match tuple {
            (NumV(v1), NumV(v2)) => NumV(v1 - v2),
            _ => panic!("unexpected args to add")
        }
    }

    fn times(v1: Value, v2: Value) -> Value {
        use Value::NumV;

        let tuple = (v1, v2);
        match tuple {
            (NumV(v1), NumV(v2)) => NumV(v1 * v2),
            _ => panic!("unexpected args to add")
        }
    }
}
