use std::iter::repeat_n;

use proc_macro2::TokenStream;
use quote::quote;
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

    pub fn differentiate(&self, variable: &MeshExpr) -> Self {
        if self == variable {
            Self::Constant(1.)
        } else {
            match self {
                Self::Sum(items) => {
                    Self::Sum(items.iter().map(|i| i.differentiate(variable)).collect())
                }
                Self::Prod(items) => {
                    let mut iter = items.iter();
                    let lhs = iter.next().unwrap().clone();
                    let rhs = Self::Prod(iter.cloned().collect()).simplify();

                    lhs.product_rule(&rhs, variable)
                }
                _ => Self::Constant(0.),
            }
        }
    }

    fn product_rule(&self, rhs: &Self, variable: &Self) -> Self {
        Self::Sum(vec![
            Self::Prod(vec![self.clone(), rhs.differentiate(variable)]),
            Self::Prod(vec![self.differentiate(variable), rhs.clone()]),
        ])
    }

    pub fn find_root_linear(self, variable: &MeshExpr) -> Self {
        let derivative = self
            .differentiate(variable)
            .substitute(variable, &Self::Constant(0.));
        let numerator = self.substitute(variable, &Self::Constant(0.));
        MeshExpr::Negate(Box::new(MeshExpr::Prod(vec![
            numerator,
            MeshExpr::Reciprocal(Box::new(derivative)),
        ])))
        .simplify()
    }

    pub fn substitute(self, target: &MeshExpr, replacement: &MeshExpr) -> Self {
        if &self == target {
            return replacement.clone();
        }
        match self {
            Self::Negate(e) => Self::Negate(Box::new(e.substitute(target, replacement))),
            Self::Reciprocal(e) => Self::Reciprocal(Box::new(e.substitute(target, replacement))),
            Self::Sum(items) => Self::Sum(
                items
                    .into_iter()
                    .map(|i| i.substitute(target, replacement))
                    .collect(),
            ),
            Self::Prod(items) => Self::Prod(
                items
                    .into_iter()
                    .map(|i| i.substitute(target, replacement))
                    .collect(),
            ),
            other => other,
        }
    }

    pub fn simplify(self) -> Self {
        match self {
            Self::Sum(items) => {
                let mut items: Vec<_> = items
                    .into_iter()
                    .map(|e| e.simplify())
                    .filter(|e| e != &Self::Constant(0.))
                    .collect();

                let mut new_items = Vec::new();

                for item in items {
                    match item {
                        Self::Sum(mut inner) => new_items.append(&mut inner),
                        other => new_items.push(other),
                    }
                }

                items = new_items;

                if items.len() == 1 {
                    items.into_iter().next().unwrap()
                } else {
                    Self::Sum(items)
                }
            }
            Self::Prod(items) => {
                let items: Vec<_> = items
                    .into_iter()
                    .map(|e| e.simplify())
                    .filter(|e| e != &Self::Constant(1.))
                    .collect();

                if items.len() == 1 {
                    items.into_iter().next().unwrap()
                } else if items.is_empty() {
                    Self::Constant(1.)
                } else if items.contains(&Self::Constant(0.)) {
                    Self::Constant(0.)
                } else {
                    Self::Prod(items)
                }
            }
            Self::Negate(n) if *n == Self::Constant(0.) => Self::Constant(0.),
            Self::Reciprocal(n) if *n == Self::Constant(1.) => Self::Constant(1.),
            Self::Negate(e) => Self::Negate(Box::new(e.simplify())),
            Self::Reciprocal(e) => Self::Reciprocal(Box::new(e.simplify())),
            other => other,
        }
    }

    pub fn render(&self) -> TokenStream {
        match self {
            &Self::AtOffset(i, j) => {
                quote! {self.mesh.get_at((i as isize + (#i)) as usize, (j as isize + (#j)) as usize)}
            }
            &Self::Constant(c) => quote! {#c},
            Self::FunctionVal(_f) => todo!(),
            Self::Negate(expr) => {
                let expr = expr.render();
                quote! {(-#expr)}
            }
            Self::Reciprocal(expr) => {
                let expr = expr.render();
                quote! {(1. / #expr)}
            }
            Self::SymbolicConst(c) => quote! {self.consts.#c},
            Self::Sum(items) => {
                let mut iter = items.iter();

                let first = iter.next().unwrap().render();
                let mut stream = quote! {#first};

                for item in iter {
                    let rendered = item.render();
                    stream = quote! {#stream + #rendered}
                }

                quote! {(#stream)}
            }
            Self::Prod(items) => {
                let mut iter = items.iter();

                let first = iter.next().unwrap().render();
                let mut stream = quote! {#first};

                for item in iter {
                    let rendered = item.render();
                    stream = quote! {#stream * #rendered}
                }

                quote! {(#stream)}
            }
        }
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

    use super::{Expression, MeshExpr, SquareMat};

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

    #[test]
    fn product_rule() {
        let expr = MeshExpr::Prod(vec![MeshExpr::AtOffset(0, 0), MeshExpr::AtOffset(0, 0)]);

        assert_eq!(
            expr.differentiate(&MeshExpr::AtOffset(0, 0)).simplify(),
            MeshExpr::Sum(vec![MeshExpr::AtOffset(0, 0), MeshExpr::AtOffset(0, 0),]),
        );
    }
}
