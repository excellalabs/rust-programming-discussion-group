# Chapter 9: Error Handling

Rust requires the developer to acknowledge the possibility of errors before compiling so that there is an awareness before deploying to production.

There are two types of errors: recoverable and unrecoverable.  Recoverable would be something like "file not found" and would ask the user to try to input the file again, but unrecoverable would be an instance where that file path is hard-coded and can't be fixed.

`Result<T, E>` for recoverable errors and `!panic` macro for unrecoverable

## Unrecoverable errors with !panic

When there's nothing that can be done for the error, `!panic` is used.  Program will print an error, unwind and clean up the stack, and quit.

### Unwinding
Unwinding means walking back up the stack and cleaning up the data from each function it encounters.  Takes time so can be avoided with immediately aborting the program.  Memory would need to be cleaned up by OS though.

```Rust
fn main() {
    panic!("crash and burn");
}
```

`thread 'main' panicked at 'crash and burn', src/main.rs:2:5`

Informs where the program panicked so that it can be investigated.  Also states the information within the `!panic` macro.

```Rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

100th element does not exist so rust will panic.  In C, this would be an undefined exception and result in a buffer overread which could lead to security vulnerabilities if attacker can manipulate the index to read something they shouldn't.  Rust instead will panic and not continue.

`thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5`

Points to line four which is where we try to access index 99.

Running with `RUST_BACKTRACE=1` will give much more information by essentially giving a trace to the error.

```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
```

## Recoverable Errors with Result

Most errors are not `!panic` level serious and can be interpreted and fixed.  Recalling from Chapter 2, `Result` enum has the following format:

```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant.

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

If we aren't sure that `File::open` will return a `Result`, we can check documentation or give it a type we know it isn't so the compiler can tell us what it is.

If it succeeds, the `Ok` variant would return our file, but if it fails `Err` would return the error information.

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

Above would be an example of handling each variant differently using `match`, and in this scenario, we would `!panic` the program if we have an issue opening the file.

```Rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

In this scenario, if we can't find the file, we would create it.  However, we would still want to handle situations where it exists and somehow we can't open it (permissions) or we can't create the file.  We can use `ErrorKind` to specify a specific error, while using `other_error` for anything else.  Note that `std::io::ErrorKind` is imported.

```Rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

This would be an alternative to using `match` but have the same functionality.

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

`unwrap` allows us to easily `!panic` if there's an error or just return `f` if there isn't one with no need for `match`.

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

Alternative to `unwrap` is `expect` and it lets you specify the error message during `!panic`

```Rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

It's possible to propagate the error in Rust.  In this scenario, instead of having the program `!panic` when it fails to open the file, we instead return the error to the function that called `read_username_from_file` since the return type of `read_username_from_file` is a `Result`.

Since the first `Ok` is not returned, the code will continue to the `read_to_string` step and will either hit `Ok` or `Err` in those cases.  Since this is actually the last code of the function, both will inherently return the results, but either as an `Ok` or an `Err`.

```Rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

Using the `?` character is a shorthand for the previous method using `match` to propagate the error.

Could even shorten it further:

```Rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

`?` can only be used on methods where the return is compatible with the `?` operator.  For instance:

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```
Since main does not return a `Result` enum, it is not compatible and will error with `error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)`

Although it states you can use an `Option<T>`, you can only use it in a function that returns an `Option`.  If it is `None`, it will end early, but if it is `Some`, the code will continue.  Additionally you can't do an `Option` when it will return a `Result` or vice versa.

It is possible to use a `Result` with `main` even though it's not usual.  You can have `main` return a `Result<(), E>` type.  `main` will exit with a 0 if successful and non-zero if not.

### To !panic or not to !panic 

There are a few situations where you would want to `!panic` on purpose or you would want to not `!panic` and let your code recover.  Custom validation, comprehensive error handling, detailing concepts, and cases where you can give more information than the compiler has.
