// /// This macro implements the FiniteDifferenceStencil2D trait for a struct. This struct should
// /// contain only instances of Node2D, the differential equation, boundary conditions, and any
// /// other strategy markers.
// #[proc_macro_attribute]
// pub fn stencil2d() {}
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod args;
mod cas;
mod finite_diff;
mod taylor;

use args::CommaSeparatedArgs;

/// Generates a struct implementing the finite difference method in 2D.
/// The name of this struct is `FiniteDiff`.
/// The code can be used by calling `FiniteDiff::compute(&mut mesh)`, where `mesh` is a `FiniteDiffMesh1D`.
///
/// # Arguments:
/// `dimensions`: The variables assigned to each dimension. For example, if solving a problem in 1 spatial
/// dimension and time, this might be `dimensions: (x, t)`.
///
/// `constants`: Any constants used in the differential equation. This will be turned into `struct Constants`,
/// which will be a parameter to `FiniteDiff::new`. Example (linear diffusion equation): `constants: [nu]`.
///
/// `equation`: The equation to solve. This should be in the form `L(u) = 0`, where L is a finite difference
/// operator. Example (linear diffusion): `equation: du/dt - nu * d2u/dx2 = 0`.
///
/// `stencil`: The nodes to be used for calculating the next unknown. Coordinates are relative to the center
/// of the Taylor expansions. Example (explicit in time, central difference in space):
/// `stencil: [(-1, 0), (0, 0), (1, 0)]`
///
/// `unknown`: The node for which the value is calculated. This is also relative to the node used as the center
/// of the Taylor expansions. Example (same example as for stencil): `unknown: (0, 1)`.
#[proc_macro]
pub fn finite_diff_2d(args: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(args as CommaSeparatedArgs);

    parsed.print_all();

    let mut vars = quote!(pub nu: f64,);

    vars = quote!(#vars pub bla: f64,);

    quote!(
        struct FiniteDiff {
            consts: Constants
        }

        impl FiniteDiff {
            fn new(consts: Constants) -> Self {
                Self {
                    consts
                }
            }
        }

        struct Constants {
            #vars
        }
    )
    .into()
}
