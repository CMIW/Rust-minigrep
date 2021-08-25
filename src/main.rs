// This is a command line tool, taken from the book "The Rust Programming Language", to learn the
// language by trying and practicing common Rust concepts. The tool is a small version of the
// classic command tool grep, it takes as arguments a filename or the path of the file we want to
// search and the string we want to search for, then it reads the file, finds the lines in the file
// that contain the string argument, and prints those lines. We use an environment variable
// CASE_INSENSITIVE to set the case sensitivity, 0 for False and 1 for True.

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // read any command line arguments passed to it and then collect the values into a vector
    let args: Vec<String> = env::args().collect();

    // create a new configuration object
    // if an error occurs display it and end the program
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // read the file, search for the query, and print the lines that contain the query
    // if an error occurs display it and end the program
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
