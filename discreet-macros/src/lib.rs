#![allow(unused)]
use std::collections::HashMap;

use discreet_common::{
    algebra::{MeshExpr, Variable},
    taylor::TaylorTable,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

mod args;
mod diff_eq;

use args::{CommaSeparatedArgs, ident_list, parse_stencil};

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
#[proc_macro]
pub fn finite_diff_2d(args: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(args as CommaSeparatedArgs);

    // parsed.print_all();
    let expr = parsed.find_arg("equation".to_string()).unwrap();
    let eqn_span = expr.span();

    // eprintln!("{expr:#?}");

    let eqn = match parse_pde(expr) {
        Ok(e) => e,
        Err(e) => return e.to_compile_error().into(),
    };
    // println!("{eqn:#?}");

    let stencil = parsed.find_arg("stencil".to_string()).unwrap();
    let stencil_span = stencil.span();
    let stencil = match parse_stencil(stencil) {
        Ok(e) => e,
        Err(e) => return e.to_compile_error().into(),
    };

    let required_derivatives = eqn.list_required_derivatives();

    let x_taylor_table = TaylorTable::new(stencil.as_slice(), Variable::X);
    let y_taylor_table = TaylorTable::new(stencil.as_slice(), Variable::Y);

    let mut derivatives = HashMap::new();

    for (v, o) in required_derivatives {
        let derivative = match v {
            Variable::X => x_taylor_table.get_scheme(o),
            Variable::Y => y_taylor_table.get_scheme(o),
        };

        let derivative = match derivative {
            Some(d) => d,
            None => {
                return syn::Error::new(
                    stencil_span,
                    "Could not construct discretisation of derivative with this stencil.",
                )
                .to_compile_error()
                .into();
            }
        };

        derivatives.insert((v, o), derivative);
    }

    // println!("{derivatives:#?}");

    let constants = match parsed.find_arg("constants".to_string()) {
        Some(constants) => match ident_list(constants) {
            Ok(c) => c,
            Err(e) => return e.to_compile_error().into(),
        },
        None => vec![],
    };
    let functions = match parsed.find_arg("functions".to_string()) {
        Some(functions) => match ident_list(functions) {
            Ok(c) => c,
            Err(e) => return e.to_compile_error().into(),
        },
        None => vec![],
    };

    // println!("{constants:?}");
    // println!("{functions:?}");

    let fn_strings: Vec<String> = functions.iter().map(|f| format!("{f}")).collect();
    let discretised_de = match MeshExpr::from_diff_eq(eqn, fn_strings.as_slice(), &derivatives) {
        Ok(e) => e,
        Err(e) => {
            return syn::Error::new(eqn_span, e.as_str())
                .to_compile_error()
                .into();
        }
    };

    let error_expr = discretised_de.render();

    let rhs_expr = discretised_de.find_root_linear(&MeshExpr::AtOffset(0, 0));

    todo!("Insert mesh scaling factors");

    let rhs_expr = rhs_expr.render();

    let mut consts = quote!();
    for c in constants {
        consts = quote!(#consts pub c: f64,);
    }

    // PLACEHOLDER
    let functions = quote!();

    quote!(
        struct FiniteDiff {
            consts: Constants,
            fns: FunctionValueMesh,
            mesh: FiniteDiffMesh
        }

        impl FiniteDiff {
            fn new(consts: Constants, mesh: FiniteDiffMesh, fns: FunctionValueMesh) -> Self {
                Self {
                    consts,
                    mesh,
                    fns
                }
            }

            fn run_iteration(&mut self) {
                let indices = self.mesh.index_iter().filter(|(i, j)| *i > 0 && *j > 0);

                match self.mesh.get_scaling() {
                    MeshScaling::SimpleGrid(dx, dy) => {
                        for (i, j) in indices {
                            self.iterate_point_simple_domain(i, j);
                        }
                    }
                    MeshScaling::ComplexPhysDomain(factors) => {
                        todo!()
                    }
                }
            }

            fn get_error_stats(&self) -> (f64, f64) {
                let mut prev_elements = 0.;
                let mut mean = 0.;
                let mut max = 0.;

                let indices = self.mesh.index_iter().filter(|(i, j)| *i > 0 && *j > 0);

                match self.mesh.get_scaling() {
                    MeshScaling::SimpleGrid(dx, dy) => {

                        for (i, j) in indices {
                            let error = (#error_expr).abs();

                            let total = mean * prev_elements + error;
                            prev_elements += 1.;
                            mean = total / prev_elements;

                            if error > max {
                                max = error;
                            }
                        }
                    }
                    MeshScaling::ComplexPhysDomain(factors) => {
                        todo!()
                    }
                }

                (mean, max)
            }

            fn iterate_point_simple_domain(
                &mut self,
                i: usize,
                j: usize,
            ) {
                let v = #rhs_expr;

                self.mesh.set_at(i, j, v);
            }
        }

        struct Constants {
            #consts
        }

        struct FunctionValueMesh {
            values: Vec<f64>
        }

        impl FunctionValueMesh {
            fn new<F: Fn(f64, f64) -> f64>(mesh: &FiniteDiffMesh, #functions) -> Self {
                todo!()
            }
        }
    )
    .into()
}
