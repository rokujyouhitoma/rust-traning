# Purpose

 - Rustに慣れていないので、まずはLISPでも実装しながら覚える

# History

## 2021.7.24

 - Improvement of `Exp`. Support for `Exp.Number`

## 2021.7.23

 - Improvement of `fn parse_eval`
   - Implement of `fn tokenize`
   - Implement of `fn parse`

## 2021.7.22

 - `impl fmt::Display for Exp`
 - Execute `cargo fmt`

## 2021.7.21

 - `enum Err`
 - Improvement of `fn eval`
   - `fn parse_eval`

## 2021.7.20

 - `fn default_env`

## 2021.7.18

 - Dummy of `Exp` . `Emv` , `fn eval`
 - Read, Print and Loop of REPL

# Reference

 - SICP
 - https://github.com/stopachka/risp/blob/master/src/main.rs
 - https://blog.livewing.net/rust-bf
 - http://norvig.com/lispy.html
