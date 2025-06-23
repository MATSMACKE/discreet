// We allow non-snake case here because of the Δ that occasionally occurs and is not lowercase. However,
// using this keeps it much more readable because of consistency with mathematical notation.
#![allow(non_snake_case)]
use std::collections::BTreeMap;

use syn::Ident;

use crate::algebra::Expression;

pub struct FiniteDiff {
    explicitness: Explicitness,
    main_finite_diff: FiniteDiffOperator,
    boundary_finite_diffs: FiniteDiffOperator,
}

#[derive(Default)]
pub struct FiniteDiffBuilder {
    dimensions: Option<Vec<Ident>>,
    constants: Option<Vec<Ident>>,
    equation: Option<Expression>,
    stencil: Option<Stencil>,
    number_format: Option<Ident>,
}

impl FiniteDiffBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dimensions(mut self, dimensions: Vec<Ident>) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn constants(mut self, constants: Vec<Ident>) -> Self {
        self.constants = Some(constants);
        self
    }

    pub fn equation(mut self, equation: Expression) -> Self {
        self.equation = Some(equation);
        self
    }

    pub fn stencil(mut self, stencil: Stencil) -> Self {
        self.stencil = Some(stencil);
        self
    }

    pub fn construct(self) -> FiniteDiff {
        todo!()
    }
}

pub struct Stencil {
    pub points: Vec<(isize, isize)>,
}

pub struct Discretisation {
    /// an LHS containing unknowns and an RHS containing knowns
    expression: (DiscretisedExpression, DiscretisedExpression),
}

impl Discretisation {
    /// Constructs a discretisation of the provided PDE based on the finite difference approximations provided.
    pub fn new(equation: Expression, approximations: DerivativeApproximations) -> Self {
        todo!()
    }
}

pub enum DiscretisedExpression {
    /// Corresponds to `coefficient * u[location.0, location.1]`.
    SolutionValue {
        location: (isize, isize),
        coefficient: f64,
    },
    Sum(Vec<DiscretisedExpression>),

    /// Divides the expression in the numerator by computational domain spacings, which is commonly necessary for derivatives.
    Div {
        numerator: Box<DiscretisedExpression>,
        denom_Δx_power: usize,
        denom_Δy_power: usize,
    },

    /// Should always be included to allow for more flexibility, and the method of manufactured solutions in particular.
    SourceTerm,
}

/// Stores finite difference operators for
pub struct DerivativeApproximations {
    operators: BTreeMap<(usize, usize), FiniteDiffOperator>,
}

/// Represents a finite difference approximation of a derivative.
pub struct FiniteDiffOperator {
    validity: NodeRange,
    terms: Vec<FiniteDiffOperatorTerm>,

    /// Power of Δx. As an example, this will usually be -1 for a first derivative approximation, as in `du/dx ≈ Δx⁻¹ * (u[i+1] - u[i])`.
    Δx_power: isize,
}

/// A term in a finite difference operator, e.g. `0.5 * u[i-1, j+1]`
pub struct FiniteDiffOperatorTerm {
    /// Relative position relative to the node where the expansion is centred
    index: (isize, isize),

    coefficient: f64,
}

/// Indicates a range of nodes in a 2D mesh
pub struct NodeRange {
    /// (i1, i2) means i1 <= i < i2. Negative indices go from the last element (so (0, -1) would correspond to all indices, (0, -2) to all but the last, etc. )
    between_i: (isize, isize),
    between_j: (isize, isize),
}

pub enum Explicitness {
    Explicit,
    Implicit,
}

pub enum Dimension2D {
    First,
    Second,
}
