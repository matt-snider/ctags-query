use getopts::Options;
use std::env;

const DEFAULT_TAGS_LOCATION: &str = "./.tags";


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Define options and parse invocation
    let mut opts = Options::new();
    opts.optopt(
        "f",
        "file",
        &format!("the location of the tag file (defaults to {})", DEFAULT_TAGS_LOCATION),
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
        print_usage(&program, opts);
        return;
    };

    // Use --file option, otherwise default
    let tags_file = match matches.opt_str("f") {
        Some(f) => f,
        None => String::from(DEFAULT_TAGS_LOCATION),
    };

    println!("Running query '{}' on file {}", query, tags_file);
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] QUERY", program);
    print!("{}", opts.usage(&brief));
}

