use crate::finite_diff2d::{Dimension2D, FiniteDiffOperator};

pub struct TaylorTable {
    cols: Vec<Vec<f64>>,
}

impl TaylorTable {
    pub fn new(stencil: &[(isize, isize)], dimension: Dimension2D) -> Self {
        let filter_fn: fn(&(isize, isize)) -> bool = match dimension {
            Dimension2D::First => |(_, y)| *y == 0,
            Dimension2D::Second => |(x, _)| *x == 0,
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

    pub fn get_scheme(derivative_order: (usize, usize)) -> FiniteDiffOperator {
        todo!()
    }
}
