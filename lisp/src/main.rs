use std::fmt;
use std::io;
use std::io::Write;

#[derive(Clone)]
enum Exp {
    Symbol(String),
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Exp::Symbol(s) => s.clone(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone)]
struct Env<'a> {
    outer: Option<&'a Env<'a>>,
}

#[derive(Debug)]
enum Err {
    Reason(String),
}

fn read() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Error: read line");
    line
}

fn default_env<'a>() -> Env<'a> {
    Env { outer: None }
}

fn eval(exp: &Exp, _env: &Env) -> Result<Exp, Err> {
    Ok(exp.clone())
}

fn parse_eval(expr: String, env: &mut Env) -> Result<Exp, Err> {
    let evaled_exp = eval(&Exp::Symbol(expr), env)?;
    Ok(evaled_exp)
}

fn main() {
    let env = &mut default_env();
    loop {
        print!("repl> ");
        io::stdout().flush().unwrap();
        let expr = read();
        match parse_eval(expr, env) {
            Ok(res) => println!("{}", res),
            Err(e) => match e {
                Err::Reason(msg) => println!("{}", msg),
            },
        }
    }
}
