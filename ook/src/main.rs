use std::env;

fn entry_point(args: Vec<String>) {
    if args.len() < 2 {
        println!("You must supply a filename");
        std::process::exit(1);
    }
    let filename: &String = &args[1];
    println!("{}", filename);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    entry_point(args);
}
