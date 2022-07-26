# Chapter 11: Writing Automated Tests

Rust compiler can check the code for a wide range of correctness, but it can't check that your program is doing what you intended your program to do.  As a result, it's important to use Rust's built in assert suite to write unit and integration tests for the program.

## How to Write Tests

1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

Tests in rust are annotated by the `test` attribute, much like `derive` from structs.  Add `#[test]` in front of a `fn` in order to have cargo use it as a test when you run `cargo test`.  When a library is created with Cargo (`cargo new myproject --lib`), cargo also creates a template test.

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

`it_works` is marked as a test with the attribute but it is possible to have other functions that aren't tests for setting up data or state for instance.  `assert_eq!` is a macro that is able to assert that the result will equal the second argument.  Running `cargo test` will give you a good amount of information including the test name, whether it passed or not, as well as total passed, failed, ignored, etc.

There is also output regarding doc tests which allow for api documentation tests to make sure that any refactoring of the code ensures that the documentation has also been changed.

A test will fail when something within the test panics, including any assert macros.  When a failed test occurs, the output changes to include why that test failed.  If one test fails, the entire test command is marked as a fail.

`assert!` is available if we want to assert that something evaluates to `true`.  When you are testing your code in the test module but you are using code from your program or the outer module, you'll need to add `use super::*;` to include all your code to be available for use in the test.

`assert_eq!` and `assert_ne!` are available to test equals (already mentioned above) and not equals.  While you could use `assert!` and just use `==` operator, these are more convenient.  In these assert cases for instance, when you have a failure, it will be able to tell you what the two results were, the expected and the actual, whereas with `assert!`, it can only mention that it didn't evaluate to `true`.  Since these use `PartialEq` and `Debug` traits, any types you create will need these traits to correctly output information when there is a failure.

You can also add custom messages as an optional argument to the assert macros, that way you can output even more information that might not be available just in the comparison the assert does.

There is also a `#[should_panic]` attribute that you can add after `#[test]` to inform the cargo test runner that the test's intended function is to panic.  You can even state what you expect the panic to be raised with by specifying it within the parenthesis `#[should_panic("Error message here")]`.

You can use Result<T,E> in tests but you can't use the `#[should_panic]` in that situation:

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Controlling How Tests Are Run

There are a few command line arguments you can use, but it's important to note the difference between changing the `cargo test` run, or the test binary, which can be accessed by putting `--` after `cargo test`.

Tests in cargo run in parallel by default but if you don't want tests to run in parallel, you can change the test binary to use just one thread: `cargo test -- --test-threads=1`.

By default, the test binary will not output any `println!` macros in the output.  These can be shown if you use `cargo test -- --show-output`.

It's also possible to run a subset of tests or just a single test.  You can pass the name of the test to `cargo test` to just run one test (`cargo test test_name`).  If you want to run multiple, you can filter based on a part of the test name.

It's possible to ignore tests by adding an `#[ignore]` attribute after `#[test]`.  If you still want to run ignored tests, you can do `cargo test -- --ignored` to run just the ignored ones, or `cargo test -- --include-ignored` to run all.

## Test Organization

The Rust community separates tests by either unit or integration tests.  Unit tests are focused, integration tests are entirely external in rust testing, essentially how someone would use the program or library after it's compiled.

You'll want to put unit tests in the `src` folder and create a module named `tests` with the `cfg(test)` attribute.  That attribute will tell cargo to not build and to only use it when testing.  Saves compile time and doesn't include useless files into the compiled library/program.

Although some programming languages don't test private functions (and there's debate whether they should be tested), Rust's privacy rules do allow for private functions to be tested due to the `use super::*` line that you can add after the module declaration.

Integration tests are entirely external and should be in their own `tests` directory outside of `src`.  Cargo will look for tests in that folder specifically when you want to run integration tests.  Its possible to make as many files as you want and cargo will compile each separately before running.

```Rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

Note that on the top, it uses the library.  If you want to test a specific integration test file, you'd want to use `cargo test --test integration_test`.
