use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
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
