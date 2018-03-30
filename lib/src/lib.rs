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
    simples: Vec<SimpleUnit>,
}

#[derive(Copy, Clone, Debug)]
pub enum SimpleUnit {
    Meter,
}

impl Num {
    fn from_digits<S: AsRef<str>>(digits: S) -> Num {
        Num {
            text: digits.as_ref().to_string(),
        }
    }
}
