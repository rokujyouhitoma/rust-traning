/*
 SICP
 https://github.com/stopachka/risp/blob/master/src/main.rs
 https://blog.livewing.net/rust-bf
 http://norvig.com/lispy.html
 */

#[derive(Clone)]
enum Exp {
    Symbol(String),
} 

#[derive(Clone)]
struct Env {
}

fn eval(_exp: &Exp, _env: Env) {
}

fn main() {
   eval(&Exp::Symbol(String::from("Hello, world!")), Env{});
   println!("Hello, world!");
}
