use syn::Ident;

use crate::algebra::Expression;

pub struct FiniteDiff2D {
    explicitness: Explicitness,
    main_finite_diff: FiniteDiffOperator,
    boundary_finite_diffs: FiniteDiffOperator,
}

#[derive(Default)]
pub struct FiniteDiff2DBuilder {
    dimensions: Option<Vec<Ident>>,
    constants: Option<Vec<Ident>>,
    equation: Option<Expression>,
    stencil: Option<Stencil>,
    number_format: Option<Ident>,
}

impl FiniteDiff2DBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dimensions(&mut self, dimensions: Vec<Ident>) -> &mut Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn constants(&mut self, constants: Vec<Ident>) -> &mut Self {
        self.constants = Some(constants);
        self
    }

    pub fn equation(&mut self, equation: Expression) -> &mut Self {
        self.equation = Some(equation);
        self
    }

    pub fn construct(self) -> FiniteDiff2D {
        todo!()
    }
}

pub struct Stencil {
    pub points: Vec<(isize, isize)>,
}

pub enum Explicitness {
    Explicit,
    Implicit,
}

/// Represents a finite difference approximation of a derivative.
pub struct FiniteDiffOperator {
    validity: NodeRange2D,
    terms: Vec<FiniteDiffOperatorTerm>,

    /// Power of Δx. As an example, this will usually be -1 for a first derivative approximation, as in `du/dx ≈ Δx⁻¹ * (u[i+1] - u[i])`.
    #[allow(non_snake_case)]
    Δx_power: isize,
}

/// A term in a finite difference operator, e.g. `0.5 * u[i-1, j+1]`
pub struct FiniteDiffOperatorTerm {
    /// Relative position relative to the node where the expansion is centred
    index: (isize, isize),

    coefficient: f64,
}

/// Indicates a range of nodes in a 2D mesh
pub struct NodeRange2D {
    /// (i1, i2) means i1 <= i < i2. Negative indices go from the last element (so (0, -1) would correspond to all indices, (0, -2) to all but the last, etc. )
    between_i: (isize, isize),
    between_j: (isize, isize),
}
