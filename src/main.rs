#![deny(rust_2018_idioms)]

mod interpreter;
mod log;
mod mark;
mod parser;

use interpreter::context::Context;
use interpreter::value::Value;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(
            "usage: {} {{script_path}} [script_arguments...]",
            args.first().unwrap()
        );
        return;
    }

    let path = &args[1];
    let script_args = &args[2..];

    let mut context = Context::default();

    for arg in script_args.iter() {
        context.slots.push(Value::STRING(arg.clone()));
    }

    let result = context.run_code(String::from(path));
    if result.is_err() {
        let error = result.unwrap_err();
        println!("{error}");
    }
}
