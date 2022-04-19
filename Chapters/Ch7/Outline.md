# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

As the program grows larger, it's important to use ways to organize the codebase.  You can split it by multiple modules and then multiple files for easier development.  

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules and use**: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

## Packages and Crates
A crate is a binary or library.  Crate root is a source file that the Rust compiler uses to make the root module of the crate. A package is one or more crates that provide specific functionality and a package contains a Cargo.toml file on how to build those crates.

A package can contain at most just one library crate but it can contain as many binary crates as necessary, but it must contain at least one crate.

You can create a new package by using `cargo new my-package-name`, which will create a Cargo.toml and a src/main.rs file.  The main.rs file would be the binary crate.  If there were a lib.rs file, it would be the library crate.  Both files would share the same name as the package.  A package can also have multiple binary crates in the src/bin folder.

A crate gives specific functionality in its own scope, such as the `rand` crate that gives random number generator functionality to a program after it has been added as a crate.  Keeping the functionality in the crate's scope clarifies whether that functionality comes from the crate or our own code and prevents conflicts.  For instance, `rand` package has `Rng` trait but we can also make a `Rng` in our own crate.  Even after adding `rand` into the program, the compiler knows which `Rng` is referenced in those scopes since ours is `Rng` but `rand` package would be `rand::Rng`.

## Defining Modules to Control Scope and Privacy

Restaurant package src/lib.rs:
```Rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
`mod` keyword defines a module, followed by the name and then curly brackets.  It is possible to have modules inside other modules and modules can also define structs, functions, enums, constants, etc.  Using modules allows for the grouping of functionality to make it easier for developers to locate the code necessary.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

`lib.rs` is noted as a the crate root (same as `main.rs`).  Above shows how the modules are related in the _module tree_.

## Paths for Referring to an Item in the Module Tree
To find an item in a module tree, it works the same as finding a file in a filesystem. 

You can have either the relative path or the absolute path.  The absolute path starts from the crate root and the relative path starts from the module currently in, referenced as self, super, or a different identifier.

```Rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
First time uses the absolute path, using crate as the root.  Second time uses the relative path using `front_of_house` as the starting point which is at the same level as `eat_at_restaurant`.  Entirely up to the developers as to whether they want to use absolute or relative, absolute would be more resilient to refactoring.

The above code will still not compile however because some parts are not public and the compiler can't access them.  Even putting `pub` in front of hosting would still result in an error because the function `add_to_waitlist` is private so we need to add `pub` in front of that as well.  What's important is since `front_of_house` and `eat_at_restaurant` are at the same level, `front_of_house` does not need to be public.

```Rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```
Using `super` allows you to essentially do something like `..` in a filesystem.

### Making Structs and Enums Public
If we put  `pub` in front of a struct, the struct will be public but each field won't.  We can keep some fields public and some private as a result.

```Rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

Because `Breakfast` has a private field, we need to create a public `impl` and public `fn` to create the `Breakfast` instance, we can't do it directly due to the private field. In contrast, here's an example with enums:

```Rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

Because enums don't allow specific fields to be public and the entire `Appetizer` enum is public, it's possible to create one and access directly without a function.

## Bringing Paths into Scope with the use Keyword
Paths can be massive and unwieldy eventually, either by using super too much or digging too far in to a module.  Instead, we can bring a path into scope by using `use` and access things easier.

```Rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

Adding `use` and a path is similar to making a symbolic link in a filesystem.  You can also use relative path to bring a path in.

In the above example, we didn't make a path directly to `add_to_waitlist` but instead to `hosting` and used `hosting::add_to_waitlist`.  This is the idiomatic way of bringing a function into scope.  This is a better way of doing `use` because you imply that the function is not a part of the current scope by needing to specify `hosting::`, otherwise it is unclear.  However, with structs and enums, it's actually recommended to do the full path as the idiomatic way.  There's no reason for this language idiom, it's just what emerged.  The exception is when you bring two structs/enums with the same name.

However, there is a different way to bring in two structs/enums with the same name by changing the name of it in the scope with `as`:

```Rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

When a name is brought into scope, it is private but can be made public.  This is called re-exporting.

```Rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

`pub use` allows external code to also call `hosting::add_to_waitlist`

### External Packages
In chapter 2, we used `rand` and imported it into the `Cargo.toml` file by using `rand = "0.8.3"`.  Then we used it by doing `use rand::Rng`.

There are several other external packages available at crates.io.

We can also use the standard library using `use` but we don't need to add it to the `Cargo.toml` file since it's internal to the compiler.

### Using Nested Paths to Clean Up Large use Lists

```Rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

or

// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

```Rust
use std::io;
use std::io::Write;

or

use std::io::{self, Write};
```

You can also bring everything in by using `*` as glob operator:

```Rust
use std::collections::*;
```

## Separating Modules into Different Files
When modules get very large, it might be better to move definitions to a different file.

src/lib.rs
```Rust 
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

src/front_of_house.rs
```Rust
pub mod hosting;
```

src/front_of_house/hosting.rs
```Rust
pub fn add_to_waitlist() {
    // ...
}
```
