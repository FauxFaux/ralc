use bigdecimal::BigDecimal;

use Expr;
use Value;

pub fn eval(expr: Expr) -> Value {
    use Expr::*;
    match expr {
        Product(v) => {
            if 1 == v.len() {
                eval(v.into_iter().next().unwrap())
            } else {
                unimplemented!("long product")
            }
        }
        Sum(v) => {
            if 1 == v.len() {
                eval(v.into_iter().next().unwrap())
            } else {
                unimplemented!("long sum")
            }
        }
        Value(v) => v,
        other => unimplemented!("{:?}", other),
    }
}
