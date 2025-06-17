use discreet_macros::{finite_diff_1d};

fn main() {
    println!("Hello World");
}

// #[finite_diff_1d]
// enum Test {}

finite_diff_1d!{
    id1: "Hello",
    id2: 1 + 2, 
    akcnl: 871023
}
