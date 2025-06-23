use discreet_common::mesh::FiniteDiffMesh1D;
use discreet_macros::finite_diff_2d;

fn main() {
    println!("Hello World");
}

finite_diff_2d! {
    // dimensions: (x, t),
    // constants: [nu],
    equation: du/dt - nu * d2u/dx2 - 2 * u = 0,
    // stencil: [(-1, 0), (0, 0), (1, 0)],
    // unknown: (0, 1),
    // number_format: f64
}
