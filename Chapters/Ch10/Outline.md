# Chapter 10: Generic Types, Traits, and Lifetimes

Generics in Rust are one way to deal with having many ways to express the same concept.

The analogy for generics is that just like how functions have parameters whose values
aren't known until the function is called, they can have generic types that aren't
known until the function is invoked. There have already been Generics covered in the 
text, Option<T>, Vec<T>, HashMap<K, V>, and Result<T, E>.

Examples in this chapter: 
* Show how generics can extract a single common function out of
two functions that differ only in their types. 
* how to use generics in structs and enums
* how to combine generics with traits so that allowed generics are restricted to a given behavior
* talk about lifetimes: these are generics too, they enable the compiler to keep references valid across
  more portions of the code than would be possible without them.

## Generics To Extract A Common Function

Starting with an example of abstraction without generics, just parameterizing a function: 

going from this for finding the largest number in a specific list:

```Rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    assert_eq!(largest, 100);
}
```

to this: 

```Rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);
}
```

So the function abstraction largest allows us to pass in any list of i32 values and  returns the largest. 
However, what if we wanted to find the largest item in lists of other types? that is where a generic will come in.

So to demonstrate generics, we go from two functions, one that finds largest in a list of i32s and one that finds it in a list of chars
looks like this before:

```Rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');
}
```

... and this after applying generics:

```Rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

Note, however, that this generics code will not compile yet. you will get this error: 

```sh
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` due to previous error
```

Because the generic is not restricted to types that have implemented the std::cmp::PartialOrd trait.

However, there are things we can talk about now before fixing this. Some observations about how Rust does Generics: 
* You name the generic type yourself, but the convention is using one letter, and 'T' for 'type' is a common one when
  you only have one and there isn't a better letter (like 'K' for key and 'V' for value in HashMap's case)
* the T was added in the signature block in 3 places in this case, <T> after the funciton name, and then to replace the type of the 
  arguments and return value that used to have the specific type.
* The way you talk about this is "making the largest function generic _over_ T"

We will cover how to make the extracted fucntion compile in the traits section.

## Other Places Generics Can Be Applied

### Structs

You can use generics in structs: 

```Rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

Note how there are two generics types declared here. if x and y used the same type we could have used just one generic, 
however in this case the values of x are ints and the values of y are floats, and the compiler would catch that these cannot be
the same, so we have to make separate generics for each. Having too many generics is a code smell that means your struct is probably 
tracking too much state and needs to be broken up.

### Enums

You can apply generics to enums as well, in fact here are a few we've already seen: 

```Rust
enum Option<T> {
    Some(T),
    None,
}
```

```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Method definitions

You can also use Generics in the method definitions of a struct or enum. 

```Rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

This has a few more nuances: 
* the <T> is placed after the impl to signify that this method is using a generic. 
* it doesn't have to be T just becaus the Point struct uses T, that is just convention
* because this method uses T, it is defined for all concrete versions of Point. However, 
  you can restrict methods to only work on some types instead by not adding the <T> after impl
  and using the specific type, e.g.

```Rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

so in this example, distance_from_origin is only defined when you declare a Point<f32>.

You can also have generics that correspond to the struct and then separate generics that are 
specific to the method definition, and all are in scope for the method definition. 

## Performance Of Generics

Generics don't make your code run any slower than it would with concrete types. _Monomorphization_ 
is the fun word that describes what Rust does to accomplish this. 
So basically the Compiler reverse engineers what the types should be by working backward from the 
concrete types used and filling the generic templates. 

## Using Traits And Generics

Traits provide a way to define the behavior a particular type has. Trait bounds are a way to restrict
generic types to only those types that have the behavior defined by some trait.

Traits group method behaviors together to be used under one type.

example given is a Summary trait that defined the ability of a type to summarize its contents: 

```Rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Some things to note: 
* The privacy modifier in this case is `pub` so other crates can use this trait. 
* trait functions aren't given bodies, just semicolons. Defining the body will be the responsibility of types that have these traits.
* you can have multiple methods in the trait body.

### How To Implement Traits On a Type

Implement a trait on a type similar to the syntax for methods, but you follow `impl <trait name> for <Type Name>` syntax

```Rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### Using The Defined Trait With a Type

You can call trait methods like any other method on the type but big thing here is you need to bring the trait
into scope as well as the type that uses it.

```Rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

Important note here, known as the `coherence` property or the `orphan rule`: If you want a type to use a trait within your crate, 
either that trait or that type must be local to your crate. You cannot, in your crate, set up a third party type to use a 
third party type. This is because the rust compiler wouldn't have a way to know which trait method implementation to use if
two crates were both implementing the same trait for the same type.

### Default Implementations

In the prior section where it was described that trait methods didn't have bodies, that isn't entirely true. They can have bodies, 
these are default implementations that can be provided. All you would need to do is just provide a body and that is the default implementation, 
no additional decoration required (e.g. like in java when you specify default interfaces).

```Rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

When your trait method is using default implementation, you use 

```Rust
impl Summary for NewsArticle {}
```

and that is enough to state that you are using the trait. If you want to override, just provide a body for your trait method as you would if there were no default.

default methods in the trait can leverage other trait methods, which allows you to build a lot of the functionality for the trait and limit what a user
of that trait needs to provide.

Pretty much all you can't do is have an overriding method call a default method, because of course the default behavior will be overriden.

### Traits As Parameters

So traits can be used like types with the `impl` keyword, and similar to generics, when you use traits as a type you are bounding the type that is passed in to something
that uses the provided trait name.

```Rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
### Trait Bound Syntax

Above was the shorthand that works in most cases, but the longform way of referring to a trait as a type (a trait bound for the type) looks like this:

```Rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

notice the extra `<T: Summary>` portion and then the generic T being used in the argument's type position. So this goes to show that this really is 
a generic that is bounded by a trait. 

There are times when one of these forms of expressing trait bounds is more appropriate than the other. The main one is something that was covered earlier about
generics. The compiler will force a single generic that is used for multiple parameters to be the same type. With traits, we might have two different types that 
both use the trait. So we'd use the `impl <trait>` parameter type for cases when the types could be different and the longform generic type declaration with one
type (let's say `T`) to have the compiler force both parameters to have the same type.

### Multipole Trait Bounds With +

In a nice to read syntax that looks like just adding things together, you can have an infix `+` operator between your traits to represent that you want the compiler
to check that the incoming parameter types have more than one trait, e.g.

```Rust
pub fn notify(item: &(impl Summary + Display)) {
```

### Alternate Syntax When There Are Many Trait Bounds

For readability, Rust has another syntax to declare the trait bounds of generics so they don't clutter up the signature block inline. It is with a `where` clause and looks like 
this:

```Rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

### Returning Types That Implement Traits

So instead of returning a specific type, you can have your functions return some type that implements a trait. In the way you might expect, you would specify this by 
replacing the type parameter of the return value in your function signature with `impl <trait name>`. So you can have the compiler enforce that a type with a certain behavior 
is returned from the function as part of the contract to the calling code. However, note that you cannot use a trait bound as a return type in this manner if
the function itself has multiple paths that each return different concrete types. This is a limitation in the way that they Rust compiler implements the trait syntax. 

### Fixing the Generics Example, Longest Function With Trait Bounds

Going to spare you the multiple step with intentional error walkthrough that was done in the text, just know that it turns out that the arguments to largest required both a 
`PartialOrd` trait and a `Copy` trait because specifying just the generic `T` allowed all types and the compiler knew to enforce that the body of this function would try and 
compare two values and would also have the trait that both i32 and char already had which was they had a size that was finite and could be stored on the stack and so they implemented `Copy`

```Rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

It is noted here that for the `Copy` solution we were forcing the types to be types that coudl be represented on the stack. There was also the `Clone` trait that we could
have used, but that would have allowed heap allocations which can be slow and if we aren't working with heap types maybe not desirable. Then tehre was a third option, switching
`T` return value in the signature to `&T` to use a reference instead and working within the slice. Trying these all out and seeing how they differ is left as an exercise to the reader.

### Conditionally Adding Methods To Types With Trait Bounds

when you use the impl block of a type's method, you can specify the trait bounds for which that method will exist geared off of what form the generic type `T` ultimately takes
when the type is defined. So imaagine you have a struct `Juice<T>` that is parameterized by `T`, then if you have traits `Potable` and `StartEngine`, you can have methods like quaff that are 
only implemented in cases where `T` uses the trait Potable

### Traits Conditionally Provided Based On Other Traits

This is called a Blanket Implementation and it allows you to rely on some traits existing if one already does. The standard library in Rust does this a lot and an example is the Display trait which has a ToString trait conditionally added to it: 

```Rust
impl<T: Display> ToString for T {
    // --snip--
}
```

Traits and Generics allow us to check for a great deal of errors that might occur in our code at compile time instead of runtime because we set up the expectation for what allowed types are and what they should be expected to do. 

## Using Lifetimes

Lifetimes are mostly inferred by the compiler, so if we follow the rules for references and borrowing we often don't encounter them. 

This is not a full overview of lifetimes but it is an introduction to ways you may encounter them. 

### Preventing Dangling References With Lifetimes

Preventing dangling references is the main goal of lifetimes. Think buffer overflow in C. 

```Rust
fn main() {
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
}
```

in the above code, r is assigned to a reference to x and then x goes out of scope and r is accessed. `the borrowed value does not live long enough` is the compiler error message we'll get.

How does Rust know?

### The Borrow Checker

Rust compiler keeps tabs on the lifetimes of all declared variables and it throws a compile time error if a reference to a subject outlives the lifetime of that subject itself.

That's it, pretty rad.

### Generic Lifetimes In Functions

So in an example provided, there is a compiler error on a longest function we try to define using slices. 

```Rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

compile error: 

```sh
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` due to previous error
```

So this is because we are trying to return a borrowed value but there is know way for the Borrow Checker in the Rust compiler to know which 
of the two branches will execute and which of the two variables will end up being returned in that return value. Since It doesn't know
which lifetime to trace, it requires us to add a generic lifetime parameter. This will set up the relationships between the parameters and the
return type so the compiler can properly protect against dangling pointers.

### Lifetime Annotation Syntax

```Rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

So the convention is a lowercase letter after an apostrophe, and it comes in right after the & reference. 
These are descriptors demarcating what lifetimes exist and how they should relate to each other. using the same letter
means both references have the same intended lifetime.

### Lifetime Annotation Function Signatures

Here is what usage of the generic lifetime parameter looks like for the longest function: 

```Rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

So that is like saying to the compiler "this function will get two slices, that should both live the same amount, and returns a slice which should live the 
same duration as the two arguments slices. Reiterate, this doesn't change the lifetimes, this just specifies what the constraints are for the borrow checker.

Quick aside, having these annotations in the signature and not the body of the function was intentional language design choice, it simplifies analysis and 
allows Rust to give more direct feedback to developers.

In the two examples that were covered, in the first one both variables have a lifetime at least as long as result and so the compiler allows it. 

in the second example, the lifetime of one of the arguments is shorter than result, so the compiler does not allow it.

### Thinking In Terms Of Lifetimes

If one of your parameters has no bearing on the return value in terms of lifetimes, you don't need to add a lifetime parameter to it.

You can't have the return value of the function depend on a value declared within the function instead of the parameters, because that is like an automatic 
dangling reference.

So it all boils down to connecting the lifetimes of parameters to return values of functions.

### Lifetime Annoations In Struct Definitions

If instead of owned types we wanted structs to contain references, we would want to have lifetme annotations for all references in the definition.

```Rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

### Lifetime Elision

As if things weren't getting advanced enough, the Rust core team implemented some inference capabilities into the Rust compiler to allow lifetime annotations to be omitted, but only
when certain rules are obeyed by the function.

These are called `lifetime elision rules` and they may change over time, and the compiler won't always allow you to get by with them in the way that you might think you can, just based on ambiguity. 

`input lifetimes` are elision rules on function or method input parameters
`output lifetimes` are elision rules on return values

3 current rules are tried and if there is still ambiguity, the compiler will throw up: 

1. input rule: compiler tries assigning a parameter to every input that is a reference, and make them separate from each other (e.g. param1 gets 'a, param2 gets 'b, etc.)
2. output rule: if there is one input parameter then it will be assigned to all return value parameters (e.g. param1 = 'a, return val = 'a)
3. output rule: if there are more than one input params but one of them is self because this is a method, then use the lifetime of self for all output params.

Seems like this only allows you to omit in most cases where you are only using one parameter or you are in a method and you know the return value won't outlive the instance.

### Lifetime Annotations In Method Definitions

Here is an example of rule 3 above in action

```Rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

So we don't need the lifetime annotations in the signature of announce_and_return_part because the compiler will label the lifetimes of all of the params and return with `'a`

### The Static Lifetime

Static in Rust is done through lifetimes, and it is annotated on types to indicate that they may live for the entire duration of the program. It looks like this:

```Rust

#![allow(unused)]
fn main() {
let s: &'static str = "I have a static lifetime.";
}
```

The text cautions against following the compiler's advice when it suggests using this, because usually you don't really want static you have just tried to create a dangling 
reference or somehow misstepped with the available lifetimes specified. 

## Generic Type Parameters, Trait Bounds , and Lifetimes Together

```Rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

See it? beautiful, right? We've got a generic that is trait bound to use Display, we've got two specific str types for the arguments as slices and a return slice
which is also a string, all of which have been given the same generic lifetime parameter.

## Summary

So the thing that ties all of these ideas together are features of Rust that enable compile time analysis of code that is flexible but conforms to 
expectations and doesn't have any dangling references. 







