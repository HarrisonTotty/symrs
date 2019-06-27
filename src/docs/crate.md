A symbolic mathematics library for the Rust programming language.

# Introduction

`symrs` is a library for symbolic mathematics with a high emphasis for data-oriented programming and ease of extension. Symbolic mathematics in `symrs` is composed the following core data structures:

* **_Structures_** - represent the static, actual structure of a mathematical expression.

* **_Expressions_** - reference-counted pointers to an underlying mathematical structure.

* **_Attributes_** - properties (inherent or derived) of mathematical structures.

* **_Relations_** - mappings from one mathematical structure to another.

* **_Computations_** - "actions" which may be performed on mathematical expressions.

* **_Kernels_** - "universes" (or "sessions") in which computations may occur.

Lets consider the following example to get an idea how each of the above components plays into the following mathematical expression:

$$ 2 * (3 + 4) $$

The above expression would correspond to an `Expression` pointing to a `Structure` with a string representation of:

```
Multiplication{2, Addition{3, 4}}
```

and would be manually constructed like so in Rust:

```rust
use symrs::prelude::*;

// Let's create our integer components
let two   = Integer::new(2);
let three = Integer::new(3);
let four  = Integer::new(4);

// Now let's assemble the final expression
let added      = Addition::new(three, four);
let expression = Multiplication::new(two, added);
```

However, we have only simply defined the structure of the above expression, and have yet to "solve the mathematics".
