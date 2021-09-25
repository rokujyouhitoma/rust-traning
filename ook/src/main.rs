use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod parser {
    use std::collections::HashMap;

    pub struct Parsed {
        pub tokens: Vec<String>,
        pub bracket_map: HashMap<i32, i32>,
    }
}

fn mainloop(parsed: parser::Parsed) {
    // TODO
    for x in parsed.tokens.iter() {
        println!("> {}", x);
    }
}

fn parse(program: String) -> parser::Parsed {
    // TODO
    println!("{}", program);
    let parsed = parser::Parsed { tokens: vec![String::from("Ook! Ook?"), String::from("Ook! Ook.")], bracket_map: HashMap::new()};
    return parsed;
}

fn run(mut file: &File) {
    let mut contents = String::new();
    let res = file.read_to_string(&mut contents);
    match res {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
    let parsed = parse(contents);
    mainloop(parsed);
}

fn entry_point(args: Vec<String>) -> std::io::Result<()> {
    if args.len() < 2 {
        println!("You must supply a filename");
        std::process::exit(1);
    }
    let filename: &String = &args[1];
    println!("{}", filename);
    let file = File::open(filename)?;
    run(&file);
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let res = entry_point(args);
    match res {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}
