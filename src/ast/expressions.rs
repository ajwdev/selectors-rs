use ast::labels::*;
use ast::operators::*;

use std::fmt::{Debug, Error, Formatter};

pub enum Expr<'input> {
    Exists(Box<LabelKey<'input>>),
    NotExists(Box<LabelKey<'input>>),
    Op(Box<LabelKey<'input>>, Operator, Box<LabelValue<'input>>),
    SetOp(Box<LabelKey<'input>>, SetOperator, Vec<Box<LabelValue<'input>>>),
    Error,
}

impl<'input> Debug for Expr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Exists(ref key) => write!(fmt, "Exists({:?})", key),
            NotExists(ref key) => write!(fmt, "NotExists({:?})", key),
            Op(ref l, op, ref r) => {
                match op {
                    Operator::Equal => write!(fmt, "Equal({:?}, {:?})", l, r),
                    Operator::NotEqual => write!(fmt, "NotEqual({:?}, {:?})", l, r),
                }
            }
            SetOp(ref l, op, ref r) => {
                match op {
                    SetOperator::In => write!(fmt, "In({:?}, {:?})", l, r),
                    SetOperator::NotIn => write!(fmt, "NotIn({:?}, {:?})", l, r),
                }
            }
            Error => write!(fmt, "error"),
        }
    }
}
