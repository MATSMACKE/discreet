#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Variable {
    X,
    Y,
}

impl Variable {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'x' => Some(Self::X),
            'y' => Some(Self::Y),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Prod(Vec<Expression>),
    Sum(Vec<Expression>),
    Constant(f64),
    Derivative(Variable, usize),
    SolutionVal,
    FunctionVal(String),
    SymbolicConstant(String),
    CrossDerivative(Vec<Variable>),
    Negate(Box<Expression>),
    Reciprocal(Box<Expression>),
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
            Self::Prod(vec) => Self::Prod(vec.into_iter().map(func).collect()),
            Self::Sum(vec) => Self::Sum(vec.into_iter().map(func).collect()),
            Self::Constant(_) => self,
            Self::Derivative(_, _) => self,
            Self::SolutionVal => self,
            Self::FunctionVal(_) => self,
            Self::SymbolicConstant(_) => self,
            Self::CrossDerivative(_) => self,
            Self::Negate(e) => Self::Negate(Box::new(func(*e))),
            Self::Reciprocal(e) => Self::Reciprocal(Box::new(func(*e))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::Variable;

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

        let expr = Expression::Prod(vec![
            Expression::Constant(0.54),
            Expression::Derivative(Variable::X, 1),
        ]);

        eprintln!("{expr:?}");

        let e = expr.substitute(&sub_fn);

        eprintln!("{e:?}");

        assert_eq!(
            e,
            Expression::Prod(vec![Expression::Constant(0.54), Expression::Constant(42.0)])
        );
    }
}

/// An expression in terms of values on the mesh
pub enum MeshExpr {
    AtOffset(isize, isize),
    Prod(Vec<Expression>),
    Sum(Vec<Expression>),
    Constant(f64),
    FunctionVal(usize),
    Negate(Box<Expression>),
    Reciprocal(Box<Expression>),
}
