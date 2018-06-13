use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
}

#[derive(Copy, Clone)]
pub enum SetOperator {
    In,
    NotIn,
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

impl Debug for SetOperator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::SetOperator::*;

        match *self {
            In => write!(fmt, "In"),
            NotIn => write!(fmt, "NotIn"),
        }
    }
}
