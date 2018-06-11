use std::fmt::{Debug, Display, Formatter, Result};

pub enum LabelKey<'input> {
    WithPrefix(&'input str, &'input str),
    NoPrefix(&'input str),
}

impl<'input> LabelKey<'input> {
    // fn new(input: &'input str) -> Self {
    // }

    fn prefix(&self) -> Option<&'input str> {
        use self::LabelKey::*;
        match *self {
            WithPrefix(prefix, _) => Some(prefix),
            NoPrefix(_) => None,
        }
    }
}

impl<'input> Display for LabelKey<'input> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::LabelKey::*;
        match *self {
            WithPrefix(prefix, label) =>
                write!(f, "{}/{}", prefix, label),
            NoPrefix(label) =>
                write!(f, "{}", label),
        }
    }
}

impl<'input> Debug for LabelKey<'input> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::LabelKey::*;
        match *self {
            WithPrefix(prefix, label) =>
                write!(f, "Label(prefix: {}, name: {})", prefix, label),
            NoPrefix(label) =>
                write!(f, "Label(name: {})", label),
        }
    }
}

#[derive(Debug)]
pub enum LabelValue<'input> {
    Value(&'input str),
    Empty
}

impl<'input> Display for LabelValue<'input> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::LabelValue::*;
        match *self {
            Value(s) => write!(f, "{}", s),
            Empty => write!(f, ""),
        }
    }
}
