use crate::cas::Expression;

pub struct TaylorTable2D {}

impl TaylorTable2D {
    pub fn new(stencil: &[(isize, isize)]) -> Self {
        todo!()
    }

    pub fn get_scheme(derivative_order: (usize, usize)) -> Expression {
        todo!()
    }
}
