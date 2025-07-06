use discreet_common::mesh2d::FiniteDiffMesh;
use discreet_macros::finite_diff_2d;

mod example;

fn main() {
    // let mut mesh = FiniteDiffMesh::from_num_points(0., 6., 0., 3., 1000, 1000);
    // mesh.fill_dirichlet_bc_vals(discreet_common::mesh2d::Boundary::Bottom, |x| {
    //     (-(x - 3.).powi(2)).exp()
    // });

    // let mut method = FiniteDiff::new(
    //     Constants { c: 0.05 },
    //     mesh,
    //     FunctionValueMesh { values: Vec::new() },
    // );

    // method.run_iteration();

    // let (mean, max) = method.get_error_stats();

    // println!("Mean: {mean}. Max: {max}");

    // method.mesh.save_values("MyMesh");
}

finite_diff_2d! {
    // dimensions: (x, t),
    // constants: [nu],
    equation: u_x + c * u_x = 0,
    // equation: 1. * u_t / 0.5
    // stencil: [(-1, 0), (0, 0), (1, 0)],
    // unknown: (0, 1),
    // number_format: f64
}
