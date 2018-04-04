use bigdecimal::BigDecimal;

use Expr;
use Value;

pub fn eval(expr: Expr) -> Value {
    use Expr::*;
    match expr {
        Product(v) => fodl(v.into_iter().map(eval), |left, right| left.mul(right)),
        Sum(v) => fodl(v.into_iter().map(eval), |left, right| left.plus(right)),
        Value(v) => v,
        other => unimplemented!("{:?}", other),
    }
}

fn fodl<I: IntoIterator<Item = T>, T, F>(iter: I, func: F) -> T
where
    F: Fn(T, T) -> T,
{
    let mut iter = iter.into_iter();
    let init = iter.next().unwrap();
    iter.fold(init, func)
}
