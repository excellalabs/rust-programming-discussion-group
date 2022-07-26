# Chapter 13: Functional Language Features: Iterators and Closures

Functional programming is a significant influence in Rust.

- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements
- How to use these two features to improve the I/O project in Chapter 12
- The performance of these two features (Spoiler alert: they’re faster than you might think!)

Enums and pattern matching are also influenced by functional programming, but those were detailed in earlier chapters.

## Closures: Anonymous Functions that Can Capture Their Environment

Rust's closures are anonymous functions you can save to a variable or send as an argument.  You can send to one context and close in a different context.

### Capturing the Environment with Closures

First aspect is closures that can capture values from the environment they are defined in.

```Rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

`ShirtColor` enum has variants `Red` and `Blue`.  The inventory is represented by `Inventory` struct that has a field `shirts` with `Vec<ShirtColor>`.  `shirt_giveaway` will get an optional shirt preference and return the color shirt the person will get.

The `giveaway` method is the one with the closure where it takes the user preference `Option<ShirtColor>` and calls `unwrap_or_else` on it.  If `Option<T>` is the `Some` variant, `unwrap_or_else` will return the value within `Some`, whereas if it's `None` it would call the closure and returns the value returned by the closure.

### Closure Type Inference and Annotation

Closures don't require annotated types of parameters or return value like functions do.  Type declarations are required on functions because they're part of an exposed explicit interface.  But closures aren't exposed, they're stored in variables and used without naming or exposing.

They are typically short and relevant to a narrow context so the compiler can infer the types.  We can add type annotations if we want to be clear and explicit but it's not necessary.

Here's an example:
```Rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The function needs the types, but the second one, since its a closure, doesn't need them but can still provide them.  The third is with the types removed and fourth has the brackets removed, which are optional because the body only has one expression.

The compiler will find issues if you misuse your closures depending on what the compiler expects it to be:

```Rust
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
```

Since the first time the compiler used the closure, it expected it to be `String` struct.  When it hits the number usage, it will state that it expected `String` struct.

### Capturing References or Moving Ownership

There's three ways a closure can capture values:

- Borrowing immutably
- Borrowing mutably
- Taking ownership

The closure will decide which of the three is used depending on how the function is written.

```Rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
```

In the above example, it only borrows immutably because it just needs to print.

```Rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}
```

This example borrows mutably because it adds an element to the `list` vector.

If you want to take ownership, you'd want to use the `move` keyword before the parameter list.  This is most used when passing a closure to a new thread to move the data to that thread.

### Moving Captured Values Out of the Closure and the `Fn` Traits

1. `FnOnce` applies to closures that can be called at least once. All closures implement this trait, because all closures can be called. If a closure moves captured values out of its body, then that closure only implements `FnOnce` and not any of the other Fn traits, because it can only be called once.
2. `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
3. `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently. Closures that don’t capture anything from their environment implement `Fn`.

```Rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

`T` is the generic that is not only for `Option` but also the return type of `unwrap_or_else`.  `F` is the generic type that specifies the closure.  The trait bound is `FnOnce -> T` since `unwrap_or_else` is only going to call the closure once.

```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

Output:
```
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]
```

In the above example, the closure sent to `sort_by_key` is actually an `FnMut` trait. The closure gets one argument and returns a type of `K` that can be ordered.  The reason why it's a `FnMut` is because it calls the closure multiple times, once for each item in the slice.  The closure itself doesn't capture, mutate, or move out anything from the environment so it meets the requirements.

Changing the closure to something like this changes the closure so that it does move something out of its environment.  The closure captures `value` and then moves `value` out of the closure by transferring ownership of `value` to the `sort_operations` function.  This makes the closure change into a `FnOnce` since that can only be done once, which makes the compiler error because `sort_by_key` cannot be used with a `FnOnce`:

```Rust
let mut sort_operations = vec![];
let value = String::from("by key called");

list.sort_by_key(|r| {
    sort_operations.push(value);
    r.width
});
```

However, if you wanted to add a counter to your operation like this, it wouldn't change it from a `FnMut` because nothing is getting moved out of the environment:

```Rust
let mut num_sort_operations = 0;
list.sort_by_key(|r| {
    num_sort_operations += 1;
    r.width
});
```

## Processing a Series of Items with Iterators

Iterators allow you to perform a task on a sequence of items in turn.  The iterator is responsible for the logic of iterating over each item and then determining when finished.  Rust iterators are lazy.  They have no effect until you call methods that consume the iterator to use it.

```Rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
```

In chapter 3, we used a `for` loop to iterate over an array but under the hood, it implicitly created and then consumed an iterator.

```Rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
```

This example uses the iterator for the `for` loop.  The iterator is just stored first and then the `for` loop uses the iterator to iterate over the items in the loop.  With iterators, it's not necessary to create your own loop starting at index 0 and finishing at the vectors length, less code/hassle/bugs.

### The `Iterator` Trait and the `next` Method

All iterators are `Iterator` trait that has an `Item` type and a `next` method that returns that same `Item` type.  When the `next` method is called, it returns either a `Some` wrapped `Item` or `None`.

```Rust
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

Calling next on an iterator changes the internal state so the iterator needed to be mutable for this test.  The values we get from calls to `next` are immutable references to the values in the vector.  If we want an iterator that takes ownership, we'd want to use `into_iter` instead or if we wanted mutable references, we'd use `iter_mut`.

### Methods that Consume the Iterator

Methods that call `next` are called consuming adaptors because calling them uses up the iterator.  `sum` repeatedly calls `next` on the iterator until fully consumed for instance.

```Rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

### Methods that Produce Other Iterators

Iterator adaptors allow you to change iterators into different kinds of iterators.  These can be chained to perform readable complex actions.  Although all iterators are lazy so you'll need to call one of the consuming adaptor methods to get results from calls to the iterator adaptors.

```Rust
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```

Without the `collect` method, we wouldn't be consuming the iterator created by map.  Calling `collect` method will collect the resulting values into a collection data type.

### Using Closures that Capture Their Environment

The `filter` iterator adaptor takes a closure that takes an item and returns a `Boolean`.  If the closure returns `true`, the value will be included in the iterator produced by `filter` but if it's `false`, it won't be included.

```Rust
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

## Comparing Performance: Loops vs Iterators

```
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

Iterators were slightly faster in this test using `search` and _The Adventures of Sherlock Holmes_ and looking for `the`.  Iterators (high-level abstraction) get compiled down to roughly the same code as if you'd written the low level code yourself.  Iterators are a zero-cost abstraction.  Another example would be a high level abstraction of an audio decoder that would be then abstracted down to assembly with zero cost, which is highly optimal for audio decoders.

```Rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```
