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
    Summands,
    SummandsFollow,
    Factors,
    FactorsFollow,
    Value,
    NumExpr,
    Digits,
    SmallSignedInt,
    UnitExpr,
    SiUnit,
    SiUnitThen,
    QualifiedUnit,
    SimpleUnit,
    SiPrefix,
    SimplePower,
    SimplePowerNum,
    UnitOverJune,
}

named!(input<CompleteStr, Expr>, add_return_error!(Fail::Input.into(),
    alt_complete!(
        summands
    )
));

named!(summands<CompleteStr, Expr>, add_return_error!(Fail::Summands.into(),
    do_parse!(
        list: separated_nonempty_list_complete!(
            tag!("+"),
            return_error!(Fail::SummandsFollow.into(), factors)) >>
        ( Expr::Sum(list) )
)));

named!(factors<CompleteStr, Expr>, add_return_error!(Fail::Factors.into(),
    do_parse!(
        list: separated_nonempty_list_complete!(
            tag!("*"),
            return_error!(Fail::FactorsFollow.into(), value_expr)) >>
        ( Expr::Product(list) )
)));

named!(value_expr<CompleteStr, Expr>,
    do_parse!(
        value: value >>
        ( Expr::Value(value) )
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
        unit_over_june
)));

named!(unit_over_june<CompleteStr, CompoundUnit>, add_return_error!(Fail::UnitOverJune.into(),
    do_parse!(
        upper: unit_list >>
        lower: opt!(preceded!(
            alt_complete!(tag!("/") | tag!(" per ")),
            unit_list
        )) >>
        ( CompoundUnit { upper, lower: lower.unwrap_or_else(Vec::new) } )
)));

named!(unit_list<CompleteStr, Vec<QualifiedUnit>>,
    many1!(qualified_unit)
);

named!(digits<CompleteStr, CompleteStr>, add_return_error!(Fail::Digits.into(),
    take_while1_s!(digit)
));

named!(small_signed_int<CompleteStr, i16>, add_return_error!(Fail::SmallSignedInt.into(),
    flat_map!(recognize!(pair!(opt!(tag!("-")), digits)), parse_to!(i16))
));

named!(si_unit<CompleteStr, (SiPrefix, SimpleUnit)>, add_return_error!(Fail::SiUnit.into(),
    alt_complete!(
        pair!(si_prefix, simple_unit) |
        simple_unit => { |unit| (SiPrefix::None, unit) }
    )
));

named!(qualified_unit<CompleteStr, QualifiedUnit>, add_return_error!(Fail::QualifiedUnit.into(),
    do_parse!(
        si_unit: si_unit >>
        power: opt!(simple_power) >>
        ( QualifiedUnit {
            si_prefix: si_unit.0,
            simple_unit: si_unit.1,
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
