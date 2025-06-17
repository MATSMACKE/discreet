use discreet_common::{Node1D, Variable};
use discreet_macros::{finite_diff_1d, variable};

fn main() {
    println!("The variable is called {}", X::get_name());
}

variable!(X);

// #[finite_diff_1d]
// enum Test {}

#[finite_diff_1d{
    boundary_left: "Dirichlet",
}]
struct test {
    target: Node1D<1>,
    center: Node1D<0>,
    upwind: Node1D<-1>,
}

#[finite_diff_1d]
struct Fails();
