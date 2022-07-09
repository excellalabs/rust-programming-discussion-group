use std::env;
use std::fs;

fn main() {
    // std::env::args will panic if any argument contains invalid Unicode. we will always pass valid unicode chars for simplicity
    // but if we had the requirement to accept invalid we could have used std::env::args_os

    // can't rely on type inference for collect, need to explicitly say Vec<String>
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    // using references to the indexed values so the arg values do not get consumed.
    // remember from Ch2, variables are immutable by default, lack of mut means these are immutable
    let query = &args[1];
    let filename = &args[2];

    // expression, no semicolon required, will be returned
    (query, filename)
}
