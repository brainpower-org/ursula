use clap::{App, Arg};
use std::fs::File;

mod parser;
use parser::parse_from_file;

fn main() {
    let matches = App::new("ursula")
        .version("1.0")
        .about("Render text to svg diagrams")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .required(true)
            .takes_value(true)
            .help("Sets input file")
            )
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILE")
            .required(true)
            .takes_value(true)
            .help("Sets the output file")
            )
        .get_matches();

    let input_file = File::open(matches.value_of("input").unwrap()).unwrap();
    let output_file = File::create(matches.value_of("output").unwrap()).unwrap();

    dbg!(output_file);

    parse_from_file(&input_file);
}
