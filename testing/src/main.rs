use discreet_common::Variable;
use discreet_macros::variable;

fn main() {
    println!("The variable is called {}", X::get_name());
}

variable!(X);

// #[discreet::finite_diff_2d::stencil]
// struct SimpleUpwind {
//     target: Node2D<1, 1>,
// }
