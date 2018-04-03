extern crate bigdecimal;
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

mod errors;
mod eval;
#[cfg(test)]
mod eval_tests;
mod parse;

use bigdecimal::BigDecimal;

pub use parse::parse_expr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Value(Value),
    Sum(Vec<Expr>),
    Product(Vec<Expr>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Value {
    num: Num,
    unit: Option<CompoundUnit>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Num {
    real: BigDecimal,
}

// TODO: Need to think about whether this derived Eq implementation does what I mean.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompoundUnit {
    upper: Vec<QualifiedUnit>,
    lower: Vec<QualifiedUnit>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct QualifiedUnit {
    si_prefix: SiPrefix,
    simple_unit: SimpleUnit,
    power: i16,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SimpleUnit {
    Meter,

    Second,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SiPrefix {
    None,
    TenToThe(i16),
}

impl Num {
    fn from_digits<S: AsRef<str>>(digits: S) -> Num {
        use std::str::FromStr;
        Num {
            real: BigDecimal::from_str(digits.as_ref()).unwrap(),
        }
    }
}

impl Value {
    fn from_int(val: i64) -> Value {
        Value {
            num: Num { real: val.into() },
            unit: None,
        }
    }
}
