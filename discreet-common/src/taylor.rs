use std::collections::HashMap;

use crate::algebra::{MeshExpr, SquareMat, Variable};

fn fact(n: usize) -> usize {
    (1..=n).product()
}

pub struct TaylorTable {
    /// Columns of the inverted Taylor table matrix. Its columns are simply
    /// the coefficients for each order derivative, zero-indexed.
    cols: Vec<Vec<f64>>,
    variable: Variable,
    stencil: Vec<isize>,
}

impl TaylorTable {
    pub fn new(stencil: &[(isize, isize)], variable: Variable) -> Self {
        let filter_fn: fn((isize, isize)) -> Option<isize> = match variable {
            Variable::X => |(x, y)| if y == 0 { Some(x) } else { None },
            Variable::Y => |(x, y)| if x == 0 { Some(y) } else { None },
        };

        let stencil: Vec<isize> = stencil.iter().copied().filter_map(filter_fn).collect();

        let size = stencil.len();
        let mut cols = Vec::with_capacity(size);

        for offset in &stencil {
            let mut col = Vec::with_capacity(size);

            for j in 0..size {
                let item = offset.pow(j as u32) as f64 / (fact(j) as f64);
                col.push(item);
            }

            cols.push(col);
        }

        let mat = SquareMat::new(cols);
        let inv = mat.invert();
        let cols = inv.get_cols();

        Self {
            cols,
            variable,
            stencil,
        }
    }

    pub fn get_scheme(&self, derivative_order: usize) -> Option<MeshExpr> {
        let col = self.cols.get(derivative_order)?;
        let size = self.cols.len();

        let mut terms = Vec::with_capacity(size);

        for (i, &coeff) in col.iter().enumerate() {
            if coeff == 0. {
                continue;
            }

            let stencil_value = self.stencil[i];

            let (i, j) = match self.variable {
                Variable::X => (stencil_value, 0),
                Variable::Y => (0, stencil_value),
            };

            terms.push(MeshExpr::Prod(vec![
                MeshExpr::Constant(coeff),
                MeshExpr::AtOffset(i, j),
            ]));
        }

        Some(MeshExpr::Sum(terms))
    }
}

pub type DerivativeApproximations = HashMap<(Variable, usize), MeshExpr>;

#[cfg(test)]
mod test {
    use crate::{
        algebra::{MeshExpr, Variable},
        taylor::fact,
    };

    use super::TaylorTable;

    #[test]
    fn fact_test() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(2), 2);
        assert_eq!(fact(3), 6);
        assert_eq!(fact(4), 24);
    }

    #[test]
    fn one_sided_diff_3_nodes() {
        let stencil = vec![(-2, 0), (0, 1), (-1, 0), (0, 0)];
        let table = TaylorTable::new(stencil.as_slice(), Variable::X);

        assert_eq!(
            table.cols,
            vec![vec![0., 0., 1.], vec![0.5, -2., 1.5], vec![1., -2., 1.]]
        );

        assert_eq!(table.stencil, vec![-2, -1, 0]);

        assert_eq!(table.variable, Variable::X)
    }

    #[test]
    fn forward_diff() {
        let stencil = vec![(0, 0), (1, 0)];
        let table = TaylorTable::new(stencil.as_slice(), Variable::X);
        let scheme = table.get_scheme(1);

        assert_eq!(
            scheme.unwrap(),
            MeshExpr::Sum(vec![
                MeshExpr::Prod(vec![MeshExpr::Constant(-1.0), MeshExpr::AtOffset(0, 0)]),
                MeshExpr::Prod(vec![MeshExpr::Constant(1.0), MeshExpr::AtOffset(1, 0)])
            ])
        )
    }
}
