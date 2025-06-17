pub mod diff_eq;

pub use diff_eq::{Derivative, Variable};

/// A node in a 2-dimensional finite difference method. This corresponds to the term u[i+I, j+J].
pub struct Node2D<const I: isize, const J: isize> {
    /// The value of this node when evaluating the finite difference operator on the stencil
    value: f64,
}

/// A node in a 1-dimensional finite difference method. This corresponds to the term u[i+I].
pub struct Node1D<const I: isize> {
    /// The value of this node when evaluating the finite difference operator on the stencil
    value: f64,
}

/// A marker struct that indicates a Dirichlet boundary condition
pub struct Dirichlet;

/// A marker struct that indicates a Dirichlet boundary condition
pub struct Neumann;

/// A struct implementing this trait can be used as the stencil for a finite difference method. This
/// trait is not intended to be manually implemented but rather to be implemented through the `stencil`
/// macro.
pub trait FiniteDifferenceStencil2D {
    /// Constructs an instance of the stencil
    fn construct(grid: &[f64], x_nodes: usize, i: usize, j: usize) -> Self;

    /// Computes the value of the target of a filled in stencil.
    fn compute(&self) -> f64;
}
