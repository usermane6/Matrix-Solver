use solver::row_reduce;
use std::env;

mod matrix;
mod solver;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    let m = matrix::Matrix::new( 
        (5, 3),
        vec![
            1.0, 3.0, 1.0, 1.0, 3.0,
            2.0,-2.0, 1.0, 2.0, 8.0,
            1.0,-5.0, 0.0, 1.0, 5.0,
        ]
    );

    println!("{}", m);

    println!("{}", row_reduce(m))
}