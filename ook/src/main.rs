use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod parser {
    use std::collections::HashMap;

    pub struct Parsed {
        pub tokens: Vec<String>,
        pub bracket_map: HashMap<u32, u32>,
    }
}

fn mainloop(parsed: parser::Parsed) {
    // TODO
    for x in parsed.tokens.iter() {
        println!("> {}", x);
    }
}

fn split(program: String) -> Vec<String> {
    // TODO
    let tokens: Vec<String> = vec![];
    return tokens;
}

fn parse(program: String) -> parser::Parsed {
    // TODO
    let tokens = split(program);

    let parsed: Vec<String> = vec![];
    let bracket_map: HashMap<u32, u32> = HashMap::new();
    let leftstack: Vec<u32> = vec![];

    let mut pc: u32 = 0;

    let instructions: HashSet<&'static str> = [
        "Ook. Ook?",
        "Ook? Ook.",
        "Ook. Ook.",
        "Ook! Ook!",
        "Ook. Ook!",
        "Ook! Ook.",
        "Ook! Ook?",
        "Ook? Ook!",
    ]
    .iter()
    .cloned()
    .collect();

    for token in tokens.iter() {
        if instructions.contains(token.as_str()) {
            println!("> {}", token);
            pc += 1;
        }
    }

    return parser::Parsed {
        tokens: parsed,
        bracket_map: HashMap::new(),
    };
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
