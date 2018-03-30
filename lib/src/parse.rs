use nom::Err;
use nom::ErrorKind;
use nom::types::CompleteStr;

use CompoundUnit;
use Expr;
use Num;
use SimpleUnit;
use Value;
use errors::*;

#[repr(u32)]
#[derive(Copy, Clone)]
enum Fail {
    Input,
    Value,
    NumExpr,
    UnitExpr,
    Digits,
    SimpleUnit,
}

named!(input<CompleteStr, Expr>, add_return_error!(Fail::Input.into(),
    alt_complete!(
        value => { |v| Expr::Value(v) }
    )
));

/// e.g. "¼m", "7/3"
named!(value<CompleteStr, Value>, add_return_error!(Fail::Value.into(),
    do_parse!(
        num: num_expr >>
        unit: opt!(unit_expr) >>
        ( Value { num, unit } )
    )
));

named!(num_expr<CompleteStr, Num>, add_return_error!(Fail::NumExpr.into(),
    alt_complete!(
        digits => { |d: CompleteStr| Num::from_digits(d.0) }
    )
));

named!(unit_expr<CompleteStr, CompoundUnit>, add_return_error!(Fail::UnitExpr.into(),
    alt_complete!(
        many1!(simple_unit) => { |simples| CompoundUnit { simples } }
)));

named!(digits<CompleteStr, CompleteStr>, add_return_error!(Fail::Digits.into(),
    take_while1_s!(digit)
));

named!(simple_unit<CompleteStr, SimpleUnit>, add_return_error!(Fail::SimpleUnit.into(),
    alt_complete!(
        tag!("m") => { |_| SimpleUnit::Meter }
)));

fn digit(c: char) -> bool {
    c.is_numeric()
}

pub fn parse_expr(msg: &str) -> Result<Expr> {
    match input(CompleteStr(msg)) {
        Ok((CompleteStr(""), expr)) => Ok(expr),
        Ok((tail, val)) => bail!(
            "illegal trailing data: {:?}, after successfully parsing: {:?}",
            tail.0,
            val
        ),
        Err(e) => panic!("{:?}", e),
    }
}

impl Fail {
    fn into(self) -> ErrorKind {
        ErrorKind::Custom(self as u32)
    }
}
