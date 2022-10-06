use clap::{App, Arg};
use fajt_codegen::{generate_code, GeneratorContext};
use fajt_parser::error::emitter::ErrorEmitter;
use fajt_parser::parse_program;
use std::fs::read_to_string;

struct Arguments {
    check: bool,
    file_name: String,
    generator_context: Option<GeneratorContext>,
}

fn main() {
    let args = get_arguments();
    let source = read_to_string(&args.file_name).unwrap();
    let mut program = parse_program(&source);

    if let Err(error) = program {
        let mut stderr = std::io::stderr();
        let mut emitter = ErrorEmitter::new(&args.file_name, &source, &mut stderr);
        emitter.emit_error(&error).unwrap();
        std::process::exit(1);
    }

    if args.check {
        println!("Parsed successfully!");
        return;
    }

    if let Some(ctx) = args.generator_context {
        let output = generate_code(program.as_mut().unwrap(), ctx);
        println!("{output}");
    } else {
        println!("{program:#?}");
    }
}

fn get_arguments() -> Arguments {
    let matches = App::new("fajt")
        .arg(Arg::with_name("file").required(true))
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .value_name("format")
                .possible_values(&["pretty", "minified"]),
        )
        .arg(Arg::with_name("check").long("check").short("c"))
        .get_matches();

    let file_name = matches.value_of("file").expect("File argument required");
    let format = matches.value_of("format");
    let check = matches.is_present("check");

    let generator_context = format.map(|format| {
        let mut context = GeneratorContext::new();
        if format == "minified" {
            context.minified = true;
        }
        context
    });

    Arguments {
        check,
        file_name: file_name.to_owned(),
        generator_context,
    }
}
