use std::collections::HashMap;

use crate::taylor::DerivativeApproximations;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

/// An expression in terms of values on the mesh
#[derive(Clone, Debug)]
pub enum MeshExpr {
    AtOffset(isize, isize),
    Prod(Vec<MeshExpr>),
    Sum(Vec<MeshExpr>),
    Constant(f64),
    SymbolicConst(usize),
    FunctionVal(usize),
    Negate(Box<MeshExpr>),
    Reciprocal(Box<MeshExpr>),
}

impl MeshExpr {
    pub fn from_diff_eq(
        eq: Expression,
        consts: &HashMap<String, usize>,
        fns: &HashMap<String, usize>,
        derivatives: &DerivativeApproximations,
    ) -> Result<Self, String> {
        match eq {
            Expression::Constant(c) => Ok(Self::Constant(c)),
            Expression::Sum(terms) => {
                let mut new_terms = Vec::with_capacity(terms.len());

                for term in terms.into_iter() {
                    new_terms.push(Self::from_diff_eq(term, consts, fns, derivatives)?)
                }

                Ok(Self::Sum(new_terms))
            }
            Expression::Prod(factors) => {
                let mut new_factors = Vec::with_capacity(factors.len());

                for term in factors.into_iter() {
                    new_factors.push(Self::from_diff_eq(term, consts, fns, derivatives)?)
                }

                Ok(Self::Sum(new_factors))
            }
            Expression::FunctionVal(f) => {
                if let Some(&fn_id) = fns.get(&f) {
                    Ok(Self::FunctionVal(fn_id))
                } else {
                    Err("Unknown function.".into())
                }
            }
            Expression::Derivative(v, o) => match derivatives.get(&(v, o)) {
                Some(d) => Ok(d.clone()),
                None => Err("Unknown derivative.".into()),
            },
            Expression::CrossDerivative(_) => todo!(),
            Expression::SolutionVal => Ok(Self::AtOffset(0, 0)),
            Expression::SymbolicConstant(c) => {
                if let Some(&c_id) = consts.get(&c) {
                    Ok(Self::SymbolicConst(c_id))
                } else {
                    Err("Unknown constant.".into())
                }
            }
            Expression::Negate(e) => Ok(Self::Negate(Box::new(Self::from_diff_eq(
                *e,
                consts,
                fns,
                derivatives,
            )?))),
            Expression::Reciprocal(e) => Ok(Self::Reciprocal(Box::new(Self::from_diff_eq(
                *e,
                consts,
                fns,
                derivatives,
            )?))),
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
