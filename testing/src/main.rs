use discreet_common::mesh2d::{FiniteDiffMesh, MeshScaling};
use discreet_macros::finite_diff_2d;

fn main() {
    let mut mesh = FiniteDiffMesh::from_num_points(0., 6., 0., 3., 100, 100);
    mesh.fill_dirichlet_bc_vals(discreet_common::mesh2d::Boundary::Bottom, |x| {
        (-(x - 3.).powi(2)).exp()
    });

    let mut method = FiniteDiff::new(
        Constants { c: 0.5 },
        mesh,
        FunctionValueMesh { values: Vec::new() },
    );

    method.run_iteration();

    let (mean, max) = method.get_error_stats();

    println!("Mean: {mean}. Max: {max}");

    method.mesh.save_values("MyMesh");
}

finite_diff_2d! {
    equation: u_y + c * u_x = 0,
    stencil: [(-1, 0), (0, 0), (0, -1)],
    constants: [c],
    functions: [],
}
