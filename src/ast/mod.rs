mod labels;
mod operators;
mod expressions;
pub use ast::labels::*;
pub use ast::operators::*;
pub use ast::expressions::*;

#[derive(Debug)]
pub struct Selector<'input> {
    expressions: Vec<Expr<'input>>
}

impl<'input> Selector<'input> {
    pub fn new(expr: Vec<Expr<'input>>) -> Self {
        Self {
            expressions: expr,
        }
    }

    pub fn combine(selector: Self, expr: Expr<'input>) -> Self {
        let mut new_selector = Self {
            expressions: selector.expressions,
        };

        new_selector.push(expr);
        new_selector
    }

    pub fn push(&mut self, expr: Expr<'input>) {
        self.expressions.push(expr);
    }
}
