extern crate bigdecimal;
extern crate cast;
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;
extern crate num_bigint;
extern crate num_traits;

mod errors;
mod eval;
#[cfg(test)]
mod eval_tests;
mod parse;

use cast::u64;
use bigdecimal::BigDecimal;
use num_traits::One;
use num_bigint::Sign;
use num_bigint::BigUint;

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
pub struct Rational {
    sign: Sign,
    top: BigUint,
    bottom: BigUint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Num {
    real: Rational,
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

impl Rational {
    fn int_from_str<S: AsRef<str>>(digits: S) -> Rational {
        use std::str::FromStr;
        Rational {
            sign: Sign::Plus,
            top: BigUint::from_str(digits.as_ref()).unwrap(),
            bottom: BigUint::one(),
        }
    }

    fn int_from_i64(val: i64) -> Rational {
        let (sign, pos) = if val < 0 {
            (Sign::Minus, u64(-val).unwrap())
        } else {
            (Sign::Plus, u64(val).unwrap())
        };

        Rational {
            sign,
            top: pos.into(),
            bottom: BigUint::one(),
        }
    }
}

impl Num {
    fn from_digits<S: AsRef<str>>(digits: S) -> Num {
        Num {
            real: Rational::int_from_str(digits),
        }
    }
}

impl std::ops::Add for Num {
    type Output = Num;

    fn add(self, rhs: Num) -> Num {
        Num {
            real: self.real + rhs.real,
        }
    }
}

impl std::ops::Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        assert_eq!(Sign::Plus, self.sign);
        assert_eq!(Sign::Plus, rhs.sign);
        assert_eq!(self.bottom, rhs.bottom);
        Rational {
            top: self.top + rhs.top,
            bottom: self.bottom,
            sign: self.sign,
        }
    }
}

impl std::ops::Mul for Num {
    type Output = Num;

    fn mul(self, rhs: Num) -> Num {
        Num {
            real: self.real * rhs.real,
        }
    }
}

impl std::ops::Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Rational {
        assert_eq!(Sign::Plus, self.sign);
        assert_eq!(Sign::Plus, rhs.sign);
        Rational {
            top: self.top * rhs.top,
            bottom: self.bottom * rhs.bottom,
            sign: self.sign,
        }
    }
}

impl Value {
    fn from_int(val: i64) -> Value {
        Value {
            num: Num {
                real: Rational::int_from_i64(val),
            },
            unit: None,
        }
    }

    fn plus(self, other: Value) -> Value {
        if other.unit == self.unit {
            Value {
                num: self.num + other.num,
                unit: self.unit,
            }
        } else {
            unimplemented!("+: different units: {:?} + {:?}", self.unit, other.unit)
        }
    }

    fn mul(self, other: Value) -> Value {
        if other.unit == self.unit {
            Value {
                num: self.num * other.num,
                unit: self.unit,
            }
        } else {
            unimplemented!("*: different units: {:?} + {:?}", self.unit, other.unit)
        }
    }
}
