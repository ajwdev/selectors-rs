mod labels;
mod operators;
mod expressions;
pub use ast::labels::*;
pub use ast::operators::*;
pub use ast::expressions::*;

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
