mod river;
mod cli;

use std::env;
use river::*;

fn main() {
    // Parse the args to get the file data
    let args: Vec<String> = env::args().collect();
    let file_data = cli::parse_args(args);

    // println!("{}", file_data);

    // Tokenize the file data
    let tokens = tokenizer::tokenize(file_data);
    for token in tokens {
        println!("{}", token);
    }
}