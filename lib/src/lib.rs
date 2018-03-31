#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

mod errors;
mod parse;

pub use parse::parse_expr;

#[derive(Debug)]
pub enum Expr {
    Value(Value),
    Sum(Vec<Expr>),
    Product(Vec<Expr>),
}

#[derive(Debug)]
pub struct Value {
    num: Num,
    unit: Option<CompoundUnit>,
}

#[derive(Debug)]
pub struct Num {
    text: String,
}

#[derive(Debug)]
pub struct CompoundUnit {
    upper: Vec<QualifiedUnit>,
    lower: Vec<QualifiedUnit>,
}

#[derive(Copy, Clone, Debug)]
pub struct QualifiedUnit {
    si_prefix: SiPrefix,
    simple_unit: SimpleUnit,
    power: i16,
}

#[derive(Copy, Clone, Debug)]
pub enum SimpleUnit {
    Meter,

    Second,
}
#[derive(Copy, Clone, Debug)]
pub enum SiPrefix {
    None,
    TenToThe(i16),
}

impl Num {
    fn from_digits<S: AsRef<str>>(digits: S) -> Num {
        Num {
            text: digits.as_ref().to_string(),
        }
    }
}
