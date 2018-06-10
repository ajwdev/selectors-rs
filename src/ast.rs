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
    Exists(Box<LabelKey<'input>>),
    NotExists(Box<LabelKey<'input>>),
    Op(Box<LabelKey<'input>>, Operator, Box<LabelValue<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub enum LabelKey<'input> {
    WithPrefix(&'input str, &'input str),
    NoPrefix(&'input str),
}

#[derive(Debug)]
pub enum LabelValue<'input> {
    Value(&'input str),
    Empty
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
