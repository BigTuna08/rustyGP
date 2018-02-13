extern crate rusty_gp;
use rusty_gp::gp_components::programs::SimpleProg;

fn main() {

    println!("Hello, world!");
    let genotype = vec![3,4,5];
    let fitness = 0.5;
    let k: SimpleProg = SimpleProg{genotype, fitness};


}
