mod lexer;
mod token;
mod query;

use getopts::Options;
use std::env;
use std::path::{PathBuf};
use crate::query::Query;

const DEFAULT_TAGS_FILE: &str = ".tags";


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Define options and parse invocation
    let mut opts = Options::new();
    opts.optopt(
        "f",
        "file",
        &format!("the location of the tag file (defaults to {})", DEFAULT_TAGS_FILE),
        "FILEPATH"
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(f) => panic!(f.to_string()),
    };

    // Check if help was requested
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Ensure a query was passed
    let query = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        eprintln!("Error: QUERY not specified");
        print_usage(&program, opts);
        return;
    };

    // Use --file option, otherwise default to DEFAULT_TAGS_FILE
    // Turn it into an absolute path
    let tags_file = PathBuf::from(
        matches
        .opt_str("f")
        .unwrap_or(String::from(DEFAULT_TAGS_FILE))
    );

    let tags_file_path = match tags_file.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error with file '{}': {}", tags_file.display(), e);
            print_usage(&program, opts);
            return
        },
    };

    // Parse and run query
    println!("Running query '{}' on file {}", query, tags_file_path.display());
    Query::from(&query);
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] QUERY", program);
    print!("{}", opts.usage(&brief));
}
