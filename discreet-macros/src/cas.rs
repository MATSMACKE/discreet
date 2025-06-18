pub struct Variable;

pub enum Expression {
    ConstMul(f64, Box<Expression>),
    Sum(Vec<Expression>),
    Constant(f64),
    Derivative(Box<Expression>, Variable, Variable)
}

impl Expression {
    pub fn substitute<F: Fn(&Self) -> Option<Self>>(self, function: F) -> Self {
        match function(&self) {
            Some(expr) => expr,
            None => self,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn example() {
        sub_fn =
    }
}
