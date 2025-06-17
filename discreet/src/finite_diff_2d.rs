use discreet_common::FiniteDifferenceStencil2D;

pub struct FiniteDiffMethod2D<Stencil: FiniteDifferenceStencil2D> {
    progress: usize,
    grid: Vec<f64>,

    _stencil: std::marker::PhantomData<Stencil>,
}

impl<Stencil: FiniteDifferenceStencil2D> FiniteDiffMethod2D<Stencil> {}
