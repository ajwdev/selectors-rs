use std::fmt::{Debug, Error, Formatter};

pub enum Expr<'input> {
    Atom(&'input str),
    Op(Box<Expr<'input>>, Operator, Box<Expr<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
}

impl<'input> Debug for Expr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Atom(n) => write!(fmt, "Atom({:?})", n),
            Op(ref l, op, ref r) => {
                match op {
                    Operator::Equal => write!(fmt, "Equal({:?}, {:?})", l, r),
                    Operator::NotEqual => write!(fmt, "NotEqual({:?}, {:?})", l, r),
                }
            }
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            Equal => write!(fmt, "=="),
            NotEqual => write!(fmt, "!="),
        }
    }
}
