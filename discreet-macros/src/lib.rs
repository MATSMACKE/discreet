// /// This macro implements the FiniteDifferenceStencil2D trait for a struct. This struct should
// /// contain only instances of Node2D, the differential equation, boundary conditions, and any
// /// other strategy markers.
// #[proc_macro_attribute]
// pub fn stencil2d() {}

#![allow(unused)]
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod args;
mod diff_eq;

use args::CommaSeparatedArgs;

use crate::diff_eq::parse_pde;

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
/// `number_format`: The number type that will be used for calculations. Example: `number_format: f64`.
#[proc_macro]
pub fn finite_diff_2d(args: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(args as CommaSeparatedArgs);

    // parsed.print_all();
    let expr = parsed.find_arg("equation".to_string()).unwrap();

    eprintln!("{expr:#?}");

    let eqn = match parse_pde(expr) {
        Ok(e) => e,
        Err(e) => return e.to_compile_error().into(),
    };

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
