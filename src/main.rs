use clap::{App, Arg};
use std::fs::read_to_string;

struct Arguments {
    file_name: String,
}

fn main() {
    let args = get_arguments();
    let source = read_to_string(args.file_name).unwrap();

    let lexer = fajt_lexer::Lexer::new(&source).unwrap();
    let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
    let mut parser = fajt_parser::Parser::new(&mut reader).unwrap();

    let program = parser.parse();
    println!("{:#?}", program);
}

fn get_arguments() -> Arguments {
    let matches = App::new("fajt").arg(Arg::with_name("file")).get_matches();

    let file_name = matches.value_of("file").expect("File argument required");

    Arguments {
        file_name: file_name.to_owned(),
    }
}
