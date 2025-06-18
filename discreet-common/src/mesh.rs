/// Represents a mesh in the computational domain for a finite difference method.
/// This mesh contains a grid of values that is used for performing the computations.
pub struct FiniteDiffMesh1D {
    grid: Vec<f64>,
    scalings: Vec<f64>,
    points: Vec<f64>,

    /// Width of the grid. This is used to allow attributes of nodes to be stored in 1D,
    /// with indexing in 2D being handled by converting using this width.
    width: usize,
}

impl FiniteDiffMesh1D {
    /// Transforms the physical domain into computational domain to allow
    pub fn from_physical_domain(points: &[&[f64]]) -> Self {
        todo!()
    }
}
