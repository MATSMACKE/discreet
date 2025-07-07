/// Represents a mesh in the computational domain for a finite difference method.
/// This mesh contains a grid of values that is used for performing the computations.
pub struct FiniteDiffMesh {
    solution_vals: Vec<f64>,
    scalings: MeshScaling,
    points: Vec<PhysicalCoordinate>,

    /// Width of the grid. This is used to allow attributes of nodes to be stored in 1D,
    /// with indexing in 2D being handled by converting using this width.
    width: usize,
}

impl FiniteDiffMesh {
    /// Transforms the physical domain into computational domain to allow
    pub fn from_physical_domain(_points: &[&[PhysicalCoordinate]]) -> Self {
        todo!()
    }

    pub fn from_num_points(
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
        numx: usize,
        numy: usize,
    ) -> Self {
        let dx = (xmax - xmin) / ((numx - 1) as f64);
        let dy = (ymax - ymin) / ((numy - 1) as f64);

        let num_points = numx * numy;
        let scalings = MeshScaling::SimpleGrid(dx, dy);

        let mut points = Vec::with_capacity(num_points);
        let solution_vals = [0f64].repeat(num_points);

        for j in 0..numy {
            let y = ymin + (j as f64) * dy;

            for i in 0..numx {
                let x = xmin + (i as f64) * dx;

                points.push(PhysicalCoordinate { x, y });
            }
        }

        let width = numx;

        Self {
            solution_vals,
            scalings,
            points,
            width,
        }
    }

    pub fn fill_dirichlet_bc_vals<F: Fn(f64) -> f64>(&mut self, bound: Boundary, func: F) {
        match bound {
            Boundary::Bottom => {
                for i in 0..self.width {
                    let index = self.get_index(i, 0);
                    let PhysicalCoordinate { x, .. } = self.points[index];
                    self.set_at(i, 0, func(x));
                }
            }

            Boundary::Top => {
                let num_rows = self.solution_vals.len() / self.width;
                for i in 0..self.width {
                    let index = self.get_index(i, num_rows - 1);
                    let PhysicalCoordinate { x, .. } = self.points[index];
                    self.set_at(i, num_rows - 1, func(x));
                }
            }

            Boundary::Left => {
                let num_rows = self.solution_vals.len() / self.width;
                for j in 0..num_rows {
                    let index = self.get_index(0, j);
                    let PhysicalCoordinate { y, .. } = self.points[index];
                    self.set_at(0, j, func(y));
                }
            }

            Boundary::Right => {
                let num_rows = self.solution_vals.len() / self.width;
                for j in 0..num_rows {
                    let index = self.get_index(num_rows - 1, j);
                    let PhysicalCoordinate { y, .. } = self.points[index];
                    self.set_at(num_rows - 1, j, func(y));
                }
            }
        }
    }

    pub fn get_at(&self, i: usize, j: usize) -> f64 {
        let idx = self.get_index(i, j);
        self.solution_vals[idx]
    }

    pub fn set_at(&mut self, i: usize, j: usize, value: f64) {
        let idx = self.get_index(i, j);
        self.solution_vals[idx] = value;
    }

    pub fn index_iter(&self) -> impl Iterator<Item = (usize, usize)> + use<> {
        let width = self.width;
        (0..self.solution_vals.len()).map(move |i| Self::make_indices(width, i))
    }

    pub fn get_scaling(&self) -> &MeshScaling {
        &self.scalings
    }

    pub fn save_coords(&self, file: &str) {
        let mut string = String::new();
        for i in 0..self.solution_vals.len() {
            let PhysicalCoordinate { x, y } = self.points[i];
            let val = self.solution_vals[i];
            string = format!("{x} {y} {val}\n");
        }

        std::fs::write(file, string).expect("Writing failed");
    }

    pub fn save_values(&self, file: &str) {
        let bytes: Vec<u8> = self
            .solution_vals
            .iter()
            .flat_map(|v| v.to_be_bytes())
            .collect();
        std::fs::write(file, bytes).expect("Writing failed");
    }

    fn get_index(&self, i: usize, j: usize) -> usize {
        i + j * self.width
    }

    fn make_indices(width: usize, index: usize) -> (usize, usize) {
        let i = index % width;
        let j = index / width;
        (i, j)
    }
}

#[derive(Debug)]
pub struct PhysicalCoordinate {
    x: f64,
    y: f64,
}

/// Identifies a boundary of the computational domain. Bottom is the line where the first coordinate is zero, top is where
/// the first coordinate is highest. Left and right are similarly defined but w.r.t. the second coordinate.
pub enum Boundary {
    Top,
    Bottom,
    Left,
    Right,
}

pub enum MeshScaling {
    /// Values are dx and dy
    SimpleGrid(f64, f64),
    ComplexPhysDomain(Vec<(f64, f64, f64, f64)>),
}
