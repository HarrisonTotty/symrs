//! An introductory example to `symrs`.

use symrs::prelude::*;

fn main() {
    // Let's create a symbol.
    let x = Symbol::new("x");
    println!("x = {}", x.to_string());

    // Let's create another symbol.
    let y = Symbol::new("y");
    println!("y = {}", y.to_string());

    // Let's add them together.
    println!("(x + y) = {}", (x.clone() + y.clone()).to_string());

    // Let's subtract them.
    println!("(x - y) = {}", (x.clone() - y.clone()).to_string());
    
}
