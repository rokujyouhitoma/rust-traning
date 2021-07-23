use std::collections::HashMap;
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
    data: HashMap<String, Exp>,
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
    let data: HashMap<String, Exp> = HashMap::new();
    Env { data, outer: None }
}

fn env_get(k: &str, env: &Env) -> Option<Exp> {
    match env.data.get(k) {
        Some(exp) => Some(exp.clone()),
        None => match &env.outer {
            Some(outer_env) => env_get(k, &outer_env),
            None => None,
        },
    }
}

fn eval(exp: &Exp, env: &Env) -> Result<Exp, Err> {
    match exp {
        Exp::Symbol(_s) => Ok(exp.clone()),
        Exp::Symbol(k) => env_get(k, env).ok_or(Err::Reason(format!("unexpected symbol: '{}'", k))),
    }
}

fn tokenize(expr: String) -> Vec<String> {
    expr.split_whitespace().map(|x| x.to_string()).collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), Err> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Err::Reason("could not get token".to_string()))?;
    match &token[..] {
        _ => Ok((Exp::Symbol(token.clone()), rest)),
    }
}

fn parse_eval(expr: String, env: &mut Env) -> Result<Exp, Err> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
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
