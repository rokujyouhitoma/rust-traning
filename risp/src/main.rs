#[derive(Clone)]
enum RispExp {
  Symbol(String),
  Number(f64),
  List(Vec<RispExp>),
} 

fn main() {
    println!("Hello, world!");
}
