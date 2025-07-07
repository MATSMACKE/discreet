use std::iter::repeat_n;

use syn::Ident;

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
    SymbolicConstant(Ident),
    CrossDerivative(Vec<Variable>),
    Negate(Box<Expression>),
    Reciprocal(Box<Expression>),
}

impl Expression {
    pub fn list_required_derivatives(&self) -> Vec<(Variable, usize)> {
        match self {
            Self::Derivative(v, o) => vec![(*v, *o)],
            Self::Sum(vec) | Self::Prod(vec) => {
                let mut items = Vec::new();
                for item in vec {
                    let mut req = item.list_required_derivatives();
                    items.append(&mut req);
                }
                items
            }
            Self::CrossDerivative(_) => todo!(),
            Self::Negate(e) | Self::Reciprocal(e) => e.list_required_derivatives(),
            _ => vec![],
        }
    }

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
            Self::SymbolicConstant(_) => self,
            Self::CrossDerivative(_) => self,
            Self::Negate(e) => Self::Negate(Box::new(func(*e))),
            Self::Reciprocal(e) => Self::Reciprocal(Box::new(func(*e))),
        }
    }
}

/// An expression in terms of values on the mesh
#[derive(Clone, Debug, PartialEq)]
pub enum MeshExpr {
    AtOffset(isize, isize),
    Prod(Vec<MeshExpr>),
    Sum(Vec<MeshExpr>),
    Constant(f64),
    SymbolicConst(Ident),
    FunctionVal(Ident),
    Negate(Box<MeshExpr>),
    Reciprocal(Box<MeshExpr>),
}

impl MeshExpr {
    pub fn from_diff_eq(
        eq: Expression,
        fns: &[String],
        derivatives: &DerivativeApproximations,
    ) -> Result<Self, String> {
        match eq {
            Expression::Constant(c) => Ok(Self::Constant(c)),
            Expression::Sum(terms) => {
                let mut new_terms = Vec::with_capacity(terms.len());

                for term in terms.into_iter() {
                    new_terms.push(Self::from_diff_eq(term, fns, derivatives)?)
                }

                Ok(Self::Sum(new_terms))
            }
            Expression::Prod(factors) => {
                let mut new_factors = Vec::with_capacity(factors.len());

                for term in factors.into_iter() {
                    new_factors.push(Self::from_diff_eq(term, fns, derivatives)?)
                }

                Ok(Self::Sum(new_factors))
            }
            Expression::SymbolicConstant(c) => Ok(if fns.contains(&format!("{c}")) {
                MeshExpr::FunctionVal(c)
            } else {
                MeshExpr::SymbolicConst(c)
            }),
            Expression::Derivative(v, o) => match derivatives.get(&(v, o)) {
                Some(d) => Ok(d.clone()),
                None => Err("Unknown derivative.".into()),
            },
            Expression::CrossDerivative(_) => todo!(),
            Expression::SolutionVal => Ok(Self::AtOffset(0, 0)),
            Expression::Negate(e) => Ok(Self::Negate(Box::new(Self::from_diff_eq(
                *e,
                fns,
                derivatives,
            )?))),
            Expression::Reciprocal(e) => Ok(Self::Reciprocal(Box::new(Self::from_diff_eq(
                *e,
                fns,
                derivatives,
            )?))),
        }
    }

    pub fn expand(self) -> Self {
        todo!()
    }

    pub fn rearrange_for(self, target: &MeshExpr) -> Result<Self, String> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SquareMat {
    size: usize,
    /// Stored as cols
    vals: Vec<f64>,
}

impl SquareMat {
    pub fn new(cols: Vec<Vec<f64>>) -> Self {
        let size = cols.len();

        let vals: Vec<_> = cols.into_iter().flat_map(Vec::into_iter).collect();

        Self { size, vals }
    }

    pub fn ident(size: usize) -> Self {
        let mut vals: Vec<f64> = repeat_n(0., size * size).collect();

        for i in 0..size {
            let idx = Self::get_index(size, i, i);

            vals[idx] = 1.;
        }

        Self { size, vals }
    }

    pub fn invert(mut self) -> Self {
        let mut inverse = Self::ident(self.size);

        for i in 0..self.size {
            inverse.row_scale(i, 1. / self.get_at(i, i));
            self.row_scale(i, 1. / self.get_at(i, i));

            for j in 0..self.size {
                if i == j {
                    continue;
                }
                inverse.row_sub(j, i, self.get_at(j, i));
                self.row_sub(j, i, self.get_at(j, i));
            }
        }

        inverse
    }

    pub fn row_scale(&mut self, target_row: usize, factor: f64) {
        for i in 0..self.size {
            let target_idx = Self::get_index(self.size, target_row, i);

            self.vals[target_idx] *= factor
        }
    }

    pub fn row_sub(&mut self, target_row: usize, source_row: usize, factor: f64) {
        for i in 0..self.size {
            let source_idx = Self::get_index(self.size, source_row, i);
            let target_idx = Self::get_index(self.size, target_row, i);

            self.vals[target_idx] -= factor * self.vals[source_idx]
        }
    }

    pub fn get_at(&self, row: usize, col: usize) -> f64 {
        let target_idx = Self::get_index(self.size, row, col);
        self.vals[target_idx]
    }

    pub fn get_index(size: usize, row: usize, col: usize) -> usize {
        row + col * size
    }

    pub fn get_cols(&self) -> Vec<Vec<f64>> {
        let mut iter = self.vals.iter();

        let mut cols = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            let mut col = Vec::with_capacity(self.size);
            for _ in 0..self.size {
                col.push(*iter.next().unwrap());
            }
            cols.push(col);
        }

        cols
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::Variable;

    use super::{Expression, SquareMat};

    #[test]
    fn substituting_derivatives() {
        fn get_fd_expr() -> Expression {
            Expression::Constant(42.0)
        }

        let sub_fn = |e: &Expression| match e {
            Expression::Derivative(_, _) => Some(get_fd_expr()),
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

    #[test]
    fn matrix_inv() {
        let mat = SquareMat::new(vec![
            vec![1., -0.5, 1.],
            vec![1., 0., 1.],
            vec![0.5, 1.5, 1.],
        ]);

        let inverse = mat.invert();

        assert_eq!(
            inverse,
            SquareMat::new(vec![
                vec![-6., 8., -2.],
                vec![-2., 2., 0.],
                vec![6., -7., 2.]
            ])
        )
    }

    #[test]
    fn matrix_get_cols() {
        let mat = SquareMat::new(vec![
            vec![1., -0.5, 1.],
            vec![1., 0., 1.],
            vec![0.5, 1.5, 1.],
        ]);

        assert_eq!(
            mat.get_cols(),
            vec![vec![1., -0.5, 1.], vec![1., 0., 1.], vec![0.5, 1.5, 1.],]
        )
    }
}
