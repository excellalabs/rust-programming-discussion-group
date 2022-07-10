use std::env;
use std::process;

use minigrep::Config;

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

    if let Err(e) = minigrep::run(config){
        println!("Application Error: {}", e);

        process::exit(1);
    }
}

