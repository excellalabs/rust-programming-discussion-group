use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // std::env::args will panic if any argument contains invalid Unicode. we will always pass valid unicode chars for simplicity
    // but if we had the requirement to accept invalid we could have used std::env::args_os

    // can't rely on type inference for collect, need to explicitly say Vec<String>
    let args: Vec<String> = env::args().collect();

    // we can call unwrap_or_else here because new now returns a Result<T, E> instance
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        // manually exit with non-zero code. Less noisy than panic! macro
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // just a heads up for this commit only, since we didn't handle the Result object and the possibility
    // of an error, the compiler will generate an error here. 
    run(config);
}

//  dyn Error allows different subtypes of Error to be returned for different reasons
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);

    // standard way to express "this side-effecting function completed without error"
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // switching to returning an Err Result object
        if args.len() < 3 {
            // error must always be a static lifetimed string
            return Err("not enough arguments");
        }
        // not using references anymore, can't violate ownership rules providing slices to Config
        // most straightforward way to share these values with config is to clone them here. Copies of the data will be made.
        // clone is inefficient, but it is 2 strings and we are only doing it once.
        // remember from Ch2, variables are immutable by default, lack of mut means these are immutable
        let query = args[1].clone();
        let filename = args[2].clone();

        // expression, no semicolon required, will be returned
        Ok(Config { query, filename }) 
    }

}
