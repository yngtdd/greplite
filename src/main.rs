use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_args() -> clap::ArgMatches<'static> {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("Text to search within")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    args
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (line_num, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        if re.find(&line).is_some() {
            println!("{}: {}", line_num, line);
        }
    }

}


fn main() {
    let args = parse_args();

    let input = args.value_of("input").unwrap_or("-");
    let pattern = args.value_of("pattern").expect("No pattern given");
    let re = Regex::new(pattern).expect("Cannot build regex");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re)
    } else {
        let filename = args.value_of("input").expect("Need file to parse");
        let f = File::open(filename).expect("Oops, where is the file?");
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
