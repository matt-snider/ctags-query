mod lexer;
mod location;
mod matcher;
mod parser;
mod tags;
mod token;
mod query;

use getopts::Options;
use std::env;
use std::io;
use std::path::{PathBuf};

use crate::parser::Parser;
use crate::matcher::Matcher;

const DEFAULT_TAGS_FILE: &str = ".tags";


fn main() -> io::Result<()> {
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
        return Ok(());
    }

    // Ensure a query was passed
    let query = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        eprintln!("Error: QUERY not specified");
        print_usage(&program, opts);
        return Ok(());
    };

    // Use --file option, otherwise default to DEFAULT_TAGS_FILE
    // Turn it into an absolute path
    let tags_file = PathBuf::from(
        matches
        .opt_str("f")
        .unwrap_or(String::from(DEFAULT_TAGS_FILE))
    );
    let tags_file_path = tags_file.canonicalize()?; 

    // Parse query
    let mut p = Parser::new(&query);
    let query = p.parse()?;

    // Get the tags from the file and find matches
    let tagged_locations = tags::from_file(tags_file_path)?;
    let matcher = Matcher::new(tagged_locations);
    for m in  matcher.get_matches(query) {
        println!("{}\t{}\t{}", m.file, m.address, m.extra);
    }

    Ok(())
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] QUERY", program);
    print!("{}", opts.usage(&brief));
}
