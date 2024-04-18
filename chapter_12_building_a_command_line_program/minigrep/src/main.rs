use std::env;
use std::process;

use minigrep::Config;

fn main() {
    /* Chapter 12 implementation
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // NOTE: use eprintln! instead of println! to ensure
        // you cannot redirect via stdout (i.e.
        // you cannot use the '>' operator in bash to output
        // to a file like output.txt because this
        // instead sends the message to stderr)
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    */

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
