use std::collections::{BTreeMap, HashMap};

use crate::algebra::{MeshExpr, Variable};

pub struct TaylorTable {
    cols: Vec<Vec<f64>>,
}

impl TaylorTable {
    pub fn new(stencil: &[(isize, isize)], variable: Variable) -> Self {
        let filter_fn: fn(&(isize, isize)) -> bool = match variable {
            Variable::X => |(_, y)| *y == 0,
            Variable::Y => |(x, _)| *x == 0,
        };

        let stencil: Vec<(isize, isize)> = stencil.iter().copied().filter(filter_fn).collect();

        let size = stencil.len();
        let mut cols = Vec::with_capacity(size);
        cols.resize_with(size, || {
            let mut row = Vec::with_capacity(size);
            row.resize_with(size, || 0.0);
            row
        });
        Self { cols }
    }

    pub fn get_scheme(derivative_order: usize) -> MeshExpr {
        todo!()
    }
}

pub type DerivativeApproximations = HashMap<(Variable, usize), MeshExpr>;
