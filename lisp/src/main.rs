use std::io;
use std::io::Write;


#[derive(Clone)]
enum Exp {
  Symbol(String),
} 

#[derive(Clone)]
struct Env<'a> {
  outer: Option<&'a Env<'a>>,
}

fn eval(_exp: &Exp, _env: &Env) {
}

fn read() -> String {
  let mut line = String::new(); 
  io::stdin().read_line(&mut line)
  .expect("Error: read line");
  line
}

fn default_env<'a>() -> Env<'a> {
  Env {outer: None}
}

fn main() {
  let env = &mut default_env();
  loop {
    print!("repl> ");
    io::stdout().flush().unwrap();
    let line = read();
    println!("// line -> {}", line);
    eval(&Exp::Symbol(line), env);
  }
}
