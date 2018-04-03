use Value;
use eval::eval;
use parse::parse_expr;

#[test]
fn numbers() {
    assert_eq!(Value::from_int(5), eval(parse_expr("5").unwrap()))
}
