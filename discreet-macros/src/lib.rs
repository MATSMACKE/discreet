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
/// `constants`: Any constants used in the differential equation. This will be turned into `struct Constants`,
/// which will be a parameter to `FiniteDiff::new`. Example (linear diffusion equation): `constants: [nu]`.
///
/// `functions`: Functions used in the PDE that aren't the PDE being solved for. For example, if you want to
/// solve `u_x + u_y - f(x, y) = 0`, you would use `functions: [f]`, and `equation: u_x + u_y - f = 0`.
/// The given functions will be used to generate a `struct FunctionValueMesh`, which represents the function
/// values at each point in the computational domain. This can be filled in at runtime before running the
/// finite difference scheme by using the new function of `FunctionValueMesh` with closures corresponding
/// to the functions (in the physical domain).
///
/// `equation`: The equation to solve. This should be in the form `L(u) = 0`, where L is a finite difference
/// operator. Example (linear diffusion): `equation: u_y - nu * u_xx = 0`.
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

    // eprintln!("{expr:#?}");

    let eqn = match parse_pde(expr) {
        Ok(e) => e,
        Err(e) => return e.to_compile_error().into(),
    };

    println!("{eqn:#?}");

    // ===========
    // THIS IS JUST A PLACEHOLDER UNTIL PROPERLY IMPLEMENTED
    let mut vars = quote!(pub nu: f64,);
    vars = quote!(#vars pub bla: f64,);

    // for one function
    let values_type = quote!((f64));
    let functions = quote!(sinxcosy: F);

    // ==========

    quote!(
        struct FiniteDiff {
            consts: Constants
        }

        impl FiniteDiff {
            fn new(consts: Constants, mesh: FiniteDiffMesh, fns: FunctionValueMesh) -> Self {
                Self {
                    consts
                }
            }
        }

        struct Constants {
            #vars
        }

        struct FunctionValueMesh {
            values: Vec<#values_type>
        }

        impl FunctionValueMesh {
            fn new<F: Fn(f64, f64) -> f64>(mesh: &FiniteDiffMesh, #functions) -> Self {
                todo!()
            }
        }
    )
    .into()
}
