/*
MIT License

Copyright (c) Stepan Parunashvili

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Write;
use std::num::ParseFloatError;
use std::rc::Rc;

#[derive(Clone)]
enum Expression {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<Expression>),
    Function(fn(&[Expression]) -> Result<Expression, Error>),
    Lambda(Lambda),
}

#[derive(Clone)]
struct Lambda {
    params_exp: Rc<Expression>,
    body_exp: Rc<Expression>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Expression::Bool(a) => a.to_string(),
            Expression::Symbol(s) => s.clone(),
            Expression::Number(n) => n.to_string(),
            Expression::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            Expression::Function(_) => "Functiontion {}".to_string(),
            Expression::Lambda(_) => "Lambda {}".to_string(),
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug)]
enum Error {
    Reason(String),
}

#[derive(Clone)]
struct Environment<'a> {
    data: HashMap<String, Expression>,
    outer: Option<&'a Environment<'a>>,
}

fn tokenize(expr: String) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(Expression, &'a [String]), Error> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Error::Reason("could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(Error::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(Expression, &'a [String]), Error> {
    let mut res: Vec<Expression> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(Error::Reason("could not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((Expression::List(res), rest)); // skip `)`, head to the token after
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> Expression {
    match token.as_ref() {
        "true" => Expression::Bool(true),
        "false" => Expression::Bool(false),
        _ => {
            let potential_float: Result<f64, ParseFloatError> = token.parse();
            match potential_float {
                Ok(v) => Expression::Number(v),
                Err(_) => Expression::Symbol(token.to_string().clone()),
            }
        }
    }
}

macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: &[Expression]| -> Result<Expression, Error> {
            let floats = parse_list_of_floats(args)?;
            let first = floats
                .first()
                .ok_or(Error::Reason("expected at least one number".to_string()))?;
            let rest = &floats[1..];
            fn f(prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            }
            Ok(Expression::Bool(f(first, rest)))
        }
    }};
}

fn default_env<'a>() -> Environment<'a> {
    let mut data: HashMap<String, Expression> = HashMap::new();
    data.insert(
        "+".to_string(),
        Expression::Function(|args: &[Expression]| -> Result<Expression, Error> {
            let sum = parse_list_of_floats(args)?
                .iter()
                .fold(0.0, |sum, a| sum + a);

            Ok(Expression::Number(sum))
        }),
    );
    data.insert(
        "-".to_string(),
        Expression::Function(|args: &[Expression]| -> Result<Expression, Error> {
            let floats = parse_list_of_floats(args)?;
            let first = *floats
                .first()
                .ok_or(Error::Reason("expected at least one number".to_string()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

            Ok(Expression::Number(first - sum_of_rest))
        }),
    );
    data.insert(
        "=".to_string(),
        Expression::Function(ensure_tonicity!(|a, b| a == b)),
    );
    data.insert(
        ">".to_string(),
        Expression::Function(ensure_tonicity!(|a, b| a > b)),
    );
    data.insert(
        ">=".to_string(),
        Expression::Function(ensure_tonicity!(|a, b| a >= b)),
    );
    data.insert(
        "<".to_string(),
        Expression::Function(ensure_tonicity!(|a, b| a < b)),
    );
    data.insert(
        "<=".to_string(),
        Expression::Function(ensure_tonicity!(|a, b| a <= b)),
    );

    Environment { data, outer: None }
}

fn parse_list_of_floats(args: &[Expression]) -> Result<Vec<f64>, Error> {
    args.iter().map(|x| parse_single_float(x)).collect()
}

fn parse_single_float(exp: &Expression) -> Result<f64, Error> {
    match exp {
        Expression::Number(num) => Ok(*num),
        _ => Err(Error::Reason("expected a number".to_string())),
    }
}

fn eval_if_args(arg_forms: &[Expression], env: &mut Environment) -> Result<Expression, Error> {
    let test_form = arg_forms
        .first()
        .ok_or(Error::Reason("expected test form".to_string()))?;
    let test_eval = eval(test_form, env)?;
    match test_eval {
        Expression::Bool(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = arg_forms
                .get(form_idx)
                .ok_or(Error::Reason(format!("expected form idx={}", form_idx)))?;
            let res_eval = eval(res_form, env);

            res_eval
        }
        _ => Err(Error::Reason(format!(
            "unexpected test form='{}'",
            test_form.to_string()
        ))),
    }
}

fn eval_def_args(arg_forms: &[Expression], env: &mut Environment) -> Result<Expression, Error> {
    let first_form = arg_forms
        .first()
        .ok_or(Error::Reason("expected first form".to_string()))?;
    let first_str = match first_form {
        Expression::Symbol(s) => Ok(s.clone()),
        _ => Err(Error::Reason(
            "expected first form to be a symbol".to_string(),
        )),
    }?;
    let second_form = arg_forms
        .get(1)
        .ok_or(Error::Reason("expected second form".to_string()))?;
    if arg_forms.len() > 2 {
        return Err(Error::Reason("def can only have two forms ".to_string()));
    }
    let second_eval = eval(second_form, env)?;
    env.data.insert(first_str, second_eval);

    Ok(first_form.clone())
}

fn eval_lambda_args(arg_forms: &[Expression]) -> Result<Expression, Error> {
    let params_exp = arg_forms
        .first()
        .ok_or(Error::Reason("expected args form".to_string()))?;
    let body_exp = arg_forms
        .get(1)
        .ok_or(Error::Reason("expected second form".to_string()))?;
    if arg_forms.len() > 2 {
        return Err(Error::Reason(
            "fn definition can only have two forms ".to_string(),
        ));
    }

    Ok(Expression::Lambda(Lambda {
        body_exp: Rc::new(body_exp.clone()),
        params_exp: Rc::new(params_exp.clone()),
    }))
}

fn eval_built_in_form(
    exp: &Expression,
    arg_forms: &[Expression],
    env: &mut Environment,
) -> Option<Result<Expression, Error>> {
    match exp {
        Expression::Symbol(s) => match s.as_ref() {
            "if" => Some(eval_if_args(arg_forms, env)),
            "def" => Some(eval_def_args(arg_forms, env)),
            "fn" => Some(eval_lambda_args(arg_forms)),
            _ => None,
        },
        _ => None,
    }
}

fn env_get(k: &str, env: &Environment) -> Option<Expression> {
    match env.data.get(k) {
        Some(exp) => Some(exp.clone()),
        None => match &env.outer {
            Some(outer_env) => env_get(k, &outer_env),
            None => None,
        },
    }
}

fn parse_list_of_symbol_strings(form: Rc<Expression>) -> Result<Vec<String>, Error> {
    let list = match form.as_ref() {
        Expression::List(s) => Ok(s.clone()),
        _ => Err(Error::Reason("expected args form to be a list".to_string())),
    }?;
    list.iter()
        .map(|x| match x {
            Expression::Symbol(s) => Ok(s.clone()),
            _ => Err(Error::Reason(
                "expected symbols in the argument list".to_string(),
            )),
        })
        .collect()
}

fn env_for_lambda<'a>(
    params: Rc<Expression>,
    arg_forms: &[Expression],
    outer_env: &'a mut Environment,
) -> Result<Environment<'a>, Error> {
    let ks = parse_list_of_symbol_strings(params)?;
    if ks.len() != arg_forms.len() {
        return Err(Error::Reason(format!(
            "expected {} arguments, got {}",
            ks.len(),
            arg_forms.len()
        )));
    }
    let vs = eval_forms(arg_forms, outer_env)?;
    let mut data: HashMap<String, Expression> = HashMap::new();
    for (k, v) in ks.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }
    Ok(Environment {
        data,
        outer: Some(outer_env),
    })
}

fn eval_forms(arg_forms: &[Expression], env: &mut Environment) -> Result<Vec<Expression>, Error> {
    arg_forms.iter().map(|x| eval(x, env)).collect()
}

fn eval(exp: &Expression, env: &mut Environment) -> Result<Expression, Error> {
    match exp {
        Expression::Symbol(k) => {
            env_get(k, env).ok_or(Error::Reason(format!("unexpected symbol k='{}'", k)))
        }
        Expression::Bool(_a) => Ok(exp.clone()),
        Expression::Number(_a) => Ok(exp.clone()),

        Expression::List(list) => {
            let first_form = list
                .first()
                .ok_or(Error::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            match eval_built_in_form(first_form, arg_forms, env) {
                Some(res) => res,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        Expression::Function(f) => f(&eval_forms(arg_forms, env)?),
                        Expression::Lambda(lambda) => {
                            let new_env = &mut env_for_lambda(lambda.params_exp, arg_forms, env)?;
                            eval(&lambda.body_exp, new_env)
                        }
                        _ => Err(Error::Reason("first form must be a function".to_string())),
                    }
                }
            }
        }
        Expression::Function(_) => Err(Error::Reason("unexpected form".to_string())),
        Expression::Lambda(_) => Err(Error::Reason("unexpected form".to_string())),
    }
}

fn parse_eval(expr: String, env: &mut Environment) -> Result<Expression, Error> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;

    Ok(evaled_exp)
}

fn input_expr() -> String {
    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");

    expr
}

fn main() {
    let env = &mut default_env();
    loop {
        print!("lisp > ");
        io::stdout().flush().unwrap();
        let expr = input_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("😚 => {}", res),
            Err(e) => match e {
                Error::Reason(msg) => println!("🔥 => {}", msg),
            },
        }
    }
}
