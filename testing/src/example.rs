#![allow(unused)]

// ==================
// CODE THAT SHOULD BE GENERATED
// finite_diff_2d! {
//     dimensions: (x, t),
//     constants: [c],
//     equation: du/dt + c * du/dx = 0,
//     stencil: [(-1, 0), (0, 0), (0, -1)],
//     unknown: (0, 1),
//     number_format: f64
// }

use discreet_common::mesh2d::{FiniteDiffMesh, MeshScaling};

struct FiniteDiff {
    consts: Constants,
    mesh: FiniteDiffMesh,
    fns: FunctionValueMesh,
}

impl FiniteDiff {
    fn new(consts: Constants, mesh: FiniteDiffMesh, fns: FunctionValueMesh) -> Self {
        Self { consts, mesh, fns }
    }

    fn run_iteration(&mut self) {
        let indices = self.mesh.index_iter().filter(|(i, j)| *i > 0 && *j > 0);

        match self.mesh.get_scaling() {
            MeshScaling::SimpleGrid(dx, dy) => {
                let scale_const_1 = (1. / (1. / dy + self.consts.c / dx)) * (self.consts.c / dx);
                let scale_const_2 = (1. / (1. / dy + self.consts.c / dx)) * (1. / dy);

                for (i, j) in indices {
                    self.iterate_point_simple_domain(i, j, scale_const_1, scale_const_2);
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
                let c1 = self.consts.c / dx;
                let c2 = 1. / dy;

                for (i, j) in indices {
                    let v0 = self.mesh.get_at(i - 1, j);
                    let v1 = self.mesh.get_at(i, j - 1);
                    let v2 = self.mesh.get_at(i, j);

                    let error = (c1 * (v2 - v0) + c2 * (v2 - v1)).abs();

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
        scale_const_1: f64,
        scale_const_2: f64,
    ) {
        let v0 = self.mesh.get_at(i - 1, j);
        let v1 = self.mesh.get_at(i, j - 1);

        let v = scale_const_1 * v0 + scale_const_2 * v1;

        self.mesh.set_at(i, j, v);
    }
}

struct Constants {
    c: f64,
}

struct FunctionValueMesh {
    values: Vec<()>,
}

impl FunctionValueMesh {
    #[allow(clippy::extra_unused_type_parameters)]
    fn new<F: Fn(f64, f64) -> f64>(mesh: &FiniteDiffMesh) -> Self {
        todo!()
    }
}
