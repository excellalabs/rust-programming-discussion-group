use std::error::Error;
use std::fs;

//  dyn Error allows different subtypes of Error to be returned for different reasons
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);

    // standard way to express "this side-effecting function completed without error"
    Ok(())
}

// once things get moved out we have to make everything public
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
