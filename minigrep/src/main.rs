use std::env;
use std::fs;

fn main() {
    // std::env::args will panic if any argument contains invalid Unicode. we will always pass valid unicode chars for simplicity
    // but if we had the requirement to accept invalid we could have used std::env::args_os

    // can't rely on type inference for collect, need to explicitly say Vec<String>
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Config {
        // a non-idiomatic way to handle errors in Rust. this would be better to catch a mistake the dev made, like assert in other languages
        // not so great for catching a mistake the user made.
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        // not using references anymore, can't violate ownership rules providing slices to Config
        // most straightforward way to share these values with config is to clone them here. Copies of the data will be made.
        // clone is inefficient, but it is 2 strings and we are only doing it once.
        // remember from Ch2, variables are immutable by default, lack of mut means these are immutable
        let query = args[1].clone();
        let filename = args[2].clone();

        // expression, no semicolon required, will be returned
        Config { query, filename }
    }

}
