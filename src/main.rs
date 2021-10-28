use clap::{App, Arg};

mod ast;
mod json;
mod parse;

fn main() {
    let matches = App::new("strange-config-format")
        .author("Charlotte Som <half-kh-hacker@hackery.site>")
        .about("Parses a strange configuration format")
        .arg(
            Arg::with_name("FILE")
                .help("Specifies the input file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_content =
        std::fs::read_to_string(matches.value_of("FILE").unwrap()).expect("Failed to read file");

    let doc_ast = parse::parse_document(&input_content)
        .expect("Failed to parse document")
        .1;

    println!("{}", json::convert_to_json(doc_ast));
}
