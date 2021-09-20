use std::env;

fn entry_point(_filename: String) {
    println!("{}", _filename);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You must supply a filename");
        std::process::exit(1);
    }
    entry_point(args[1].clone());
}
