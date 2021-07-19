use std::io;
use std::io::Write;


#[derive(Clone)]
enum Exp {
  Symbol(String),
} 

#[derive(Clone)]
struct Env {
}

fn eval(_exp: &Exp, _env: &Env) {
}

fn read() -> String {
  let mut line = String::new(); 
  io::stdin().read_line(&mut line)
  .expect("Error: read line");
  line
}

fn main() {
  let env = &mut Env{};
  loop {
    print!("repl> ");
    io::stdout().flush().unwrap();
    let line = read();
    println!("// line -> {}", line);
    eval(&Exp::Symbol(line), env);
  }
}
