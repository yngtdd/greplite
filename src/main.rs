use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use clap::{App, Arg};
use regex::Regex;

fn main() {
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

    let filename = args.value_of("file").unwrap();
    let f = File::open(filename).expect("Oops, where is the file?");
    let reader = BufReader::new(f);

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    for (line_num, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        if re.find(&line).is_some() {
            println!("{}: {}", line_num, line);
        }
    }
}
