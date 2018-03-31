use nom::Err;
use nom::ErrorKind;
use nom::types::CompleteStr;

use CompoundUnit;
use Expr;
use Num;
use QualifiedUnit;
use SiPrefix;
use SimpleUnit;
use Value;
use errors::*;

#[repr(u32)]
#[derive(Copy, Clone)]
enum Fail {
    Input,
    Value,
    NumExpr,
    Digits,
    SmallSignedInt,
    UnitExpr,
    QualifiedUnit,
    SimpleUnit,
    SiPrefix,
    SimplePower,
    SimplePowerNum,
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
        many1!(qualified_unit) => { |inner| CompoundUnit { inner } }
)));

named!(digits<CompleteStr, CompleteStr>, add_return_error!(Fail::Digits.into(),
    take_while1_s!(digit)
));

named!(small_signed_int<CompleteStr, i16>, add_return_error!(Fail::SmallSignedInt.into(),
    flat_map!(recognize!(pair!(opt!(tag!("-")), digits)), parse_to!(i16))
));

named!(qualified_unit<CompleteStr, QualifiedUnit>, add_return_error!(Fail::QualifiedUnit.into(),
    do_parse!(
        si_prefix: opt!(si_prefix) >>
        simple_unit: simple_unit >>
        power: opt!(simple_power) >>
        ( QualifiedUnit {
            si_prefix: si_prefix.unwrap_or(SiPrefix::None),
            simple_unit,
            power: power.unwrap_or(1),
        })
)));

named!(si_prefix<CompleteStr, SiPrefix>, add_return_error!(Fail::SiPrefix.into(),
    alt_complete!(
        tag!("m") => { |_| SiPrefix::TenToThe(-3) }
)));

named!(simple_unit<CompleteStr, SimpleUnit>, add_return_error!(Fail::SimpleUnit.into(),
    alt_complete!(
        tag!("s") => { |_| SimpleUnit::Second } |
        tag!("m") => { |_| SimpleUnit::Meter }
)));

named!(simple_power<CompleteStr, i16>, add_return_error!(Fail::SimplePower.into(),
    preceded!(
        tag!("^"),
        return_error!(Fail::SimplePowerNum.into(), small_signed_int)
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
