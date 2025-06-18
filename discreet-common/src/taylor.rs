use crate::algebra::Expression;

pub struct DerivativeApproximation2D {
    expression: Expression,

    /// Exponents of the steps in each direction in the leading order error term.
    /// E.g. If the error is `O(Δx^2 * Δy^3)`, this will be (2, 3).
    leading_error: (usize, usize),
}

pub struct TaylorTable2D {
    cols: Vec<Vec<f64>>,
}

impl TaylorTable2D {
    pub fn new(stencil: &[(isize, isize)]) -> Self {
        let size = stencil.len();
        let mut cols = Vec::with_capacity(size);
        cols.resize_with(size, || {
            let mut row = Vec::with_capacity(size);
            row.resize_with(size, || 0.0);
            row
        });

        Self { cols }
    }

    pub fn get_scheme(derivative_order: (usize, usize)) -> Expression {
        todo!()
    }
}
