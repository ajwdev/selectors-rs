use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub struct Selector<'input> {
    expressions: Vec<Box<Expr<'input>>>
}

impl<'input> Selector<'input> {
    pub fn new(expr: Vec<Box<Expr<'input>>>) -> Self {
        Self {
            expressions: expr,
        }
    }

    pub fn combine(selector: Self, expr: Box<Expr<'input>>) -> Self {
        let mut new_selector = Self {
            expressions: selector.expressions,
        };

        new_selector.append(expr);
        new_selector
    }

    pub fn append(&mut self, expr: Box<Expr<'input>>) {
        self.expressions.push(expr);
    }
}

pub enum Expr<'input> {
    Key(&'input str),
    Value(&'input str),
    Op(Box<Expr<'input>>, Operator, Box<Expr<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
}

// #[derive(Copy, Clone)]
// pub enum LabelKey {
//     WithPrefix(&'input str, &'input str),
//     NoPrefix(&'input str),
// }

impl<'input> Debug for Expr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Key(label) => {
                write!(fmt, "Key({:?})", label)
            }
            Value(n) => write!(fmt, "Value({:?})", n),
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
