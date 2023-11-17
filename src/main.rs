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

macro_rules! print_error {
    ($error:expr) => {
        eprintln!("\n\n{:-^1$}", "Error", 60);
        eprintln!("{}", $error);
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!(
            "usage: {} {{script_path}} [script_arguments...]",
            args.first().unwrap()
        );
        return;
    }

    let path = match ResourcePath::try_from(PathBuf::from(&args[1])) {
        Ok(path) => path,
        Err(error) => {
            print_error!(error);
            return;
        }
    };

    let script_args = &args[2..];
    let context = Context::new();

    let mut context = match context {
        Ok(context) => context,
        Err(error) => {
            print_error!(error);
            return;
        }
    };

    for arg in script_args.iter() {
        context
            .slots
            .push(Variant::STRAND(Strand::from(arg.clone())));
    }

    let _ = match context.run_resource(path) {
        Ok(signal) => signal,
        Err(error) => {
            print_error!(error);
            return;
        }
    };
}
