#[derive(Debug, PartialEq)]
pub struct Variable;

#[derive(Debug, PartialEq)]
pub enum Expression {
    ConstMul(f64, Box<Expression>),
    Sum(Vec<Expression>),
    Constant(f64),
    Derivative(Box<Expression>, Vec<Variable>),
}

impl Expression {
    pub fn substitute<F: Fn(&Self) -> Option<Self>>(self, func: &F) -> Self {
        match func(&self) {
            Some(expr) => expr,
            None => self.replace_children(|expr| expr.substitute(func)),
        }
    }

    pub fn replace_children<F: Fn(Self) -> Self>(self, func: F) -> Self {
        match self {
            Self::ConstMul(v, e) => Self::ConstMul(v, Box::new(func(*e))),
            Self::Sum(vec) => Self::Sum(vec.into_iter().map(func).collect()),
            Self::Constant(_) => self,
            Self::Derivative(e, v) => Self::Derivative(Box::new(func(*e)), v),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Expression;

    #[test]
    fn substituting_derivatives() {
        fn get_fd_expr() -> Expression {
            Expression::Constant(42.0)
        }

        let sub_fn = |e: &Expression| match e {
            Expression::Derivative(func, variables) => Some(get_fd_expr()),
            _ => None,
        };

        let expr = Expression::ConstMul(
            0.54,
            Box::new(Expression::Derivative(
                Box::new(Expression::Constant(1.0)),
                vec![],
            )),
        );

        eprintln!("{expr:?}");

        let e = expr.substitute(&sub_fn);

        eprintln!("{e:?}");

        assert_eq!(
            e,
            Expression::ConstMul(0.54, Box::new(Expression::Constant(42.0)),)
        );
    }
}
