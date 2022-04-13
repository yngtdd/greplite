use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_args() -> clap::ArgMatches<'static> {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searches for patterns")
        .arg(
            Arg::with_name("file")
                .help("The file to search within")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    args
}

fn main() {
    let args = parse_args();
    let filename = args.value_of("file").expect("Need file to parse");
    let f = File::open(filename).expect("Oops, where is the file?");
    let reader = BufReader::new(f);
    let pattern = args.value_of("pattern").expect("No pattern given");
    let re = Regex::new(pattern).expect("Cannot build regex");

    for (line_num, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        if re.find(&line).is_some() {
            println!("{}: {}", line_num, line);
        }
    }
}
