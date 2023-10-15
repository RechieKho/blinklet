#![deny(rust_2018_idioms)]

mod backtrace;
mod interpreter;
mod log;
mod mark;
mod parser;

use interpreter::context::Context;
use interpreter::resource::ResourcePath;
use interpreter::variant::strand::Strand;
use interpreter::variant::Variant;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(
            "usage: {} {{script_path}} [script_arguments...]",
            args.first().unwrap()
        );
        return;
    }

    let path = ResourcePath::try_from(PathBuf::from(&args[1]));
    if path.is_err() {
        let error = path.unwrap_err();
        println!("\n\n{error}");
        return;
    }
    let path = path.unwrap();

    let script_args = &args[2..];
    let mut context = Context::default();
    for arg in script_args.iter() {
        context
            .slots
            .push(Variant::STRAND(Strand::from(arg.clone())));
    }

    let result = context.run_code(path);
    if result.is_err() {
        let error = result.unwrap_err();
        println!("\n\n{error}");
        return;
    }
}
