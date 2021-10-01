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

struct Tape {
    position: u64,
    thetape: Vec<u64>,
}

impl Tape {
    fn new() -> Self {
        Tape {
            position: 0,
            thetape: vec![0],
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.thetape.len() as u64 <= self.position {
            self.thetape.push(0);
        }
    }

    fn devance(&mut self) {
        self.position -= 1;
    }
}

fn mainloop(parsed: parser::Parsed) {
    // TODO
    for x in parsed.tokens.iter() {
        println!("{}", x);
    }
}

fn split(program: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let fragments: Vec<String> = program.split(" ").map(|s| s.to_string()).collect();
    let length = fragments.len() / 2;
    for n in 0..length {
        let mut s = String::new();
        s.push_str(&fragments[n * 2]);
        s.push(' ');
        s.push_str(&fragments[n * 2 + 1]);
        tokens.push(s);
    }
    return tokens;
}

fn parse(program: String) -> parser::Parsed {
    let tokens = split(program);

    let mut parsed: Vec<String> = vec![];
    let mut bracket_map: HashMap<u32, u32> = HashMap::new();
    let mut leftstack: Vec<u32> = vec![];

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

    let mut left: u32 = 0;
    let mut right: u32 = 0;
    for token in tokens.iter() {
        if instructions.contains(token.as_str()) {
            parsed.push(token.to_string());

            if token.eq("Ook! Ook?") {
                leftstack.push(pc);
            } else if token.eq("Ook? Ook!") {
                left = match leftstack.pop() {
                    Some(number) => number,
                    None => 0,
                };
                right = pc;
                bracket_map.insert(left, right);
                bracket_map.insert(right, left);
            }

            pc += 1;
        }
    }

    return parser::Parsed {
        tokens: parsed,
        bracket_map: bracket_map,
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
