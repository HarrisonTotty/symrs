//! An introductory example to `symrs`.

use symrs::*;
use std::rc::Rc;

fn main() {
    // Let's create a symbol.
    println!("----- x -----");
    let x = Symbol::new("x");
    println!("x.kind()      = {}", x.kind());
    println!("x.namespace() = {}", x.namespace());
    println!("x.skind()     = {}", x.skind());
    println!("x.to_string() = {}", x.to_string());

    // Let's create another symbol.
    let y = Symbol::new("y");
    println!("----- y -----");
    println!("y.kind()      = {}", y.kind());
    println!("y.namespace() = {}", y.namespace());
    println!("y.skind()     = {}", y.skind());
    println!("y.to_string() = {}", y.to_string());

    // Let's construct the addition between x and y.
    let added = x.clone() + "y";
    println!("----- (x + y) -----");
    println!("added.kind()      = {}", added.kind());
    println!("added.namespace() = {}", added.namespace());
    println!("added.skind()     = {}", added.skind());
    println!("added.to_string() = {}", added.to_string());

    // Let's create another symbol and build it up further.
    let z = Symbol::new("z");
    let added2 = added.clone() + z.clone();
    println!("----- ((x + y) + z) -----");
    println!("added2.to_string() = {}", added2.to_string());

    // Let's do something funky
    let added3 = added.clone() + added2.clone();
    println!("----- (added + added2) -----");
    println!("added3.to_string() = {}", added3.to_string());

    // Let's make some numbers
    let one   = Integer::new(1);
    let two   = Integer::new(2);
    let three = Integer::new(3);
    let four  = Integer::new(4);

    match (one.clone() + 1).apply() {
        Ok(None) => println!("Nothing!"),
        Ok(Some(a)) => println!("1 + 1 = {}", a.to_string()),
        _ => println!("Something happened...")
    };

    match (one.clone() * 1).apply() {
        Ok(None) => println!("Nothing!"),
        Ok(Some(a)) => println!("1 * 1 = {}", a.to_string()),
        _ => println!("Something happened...")
    };
}
