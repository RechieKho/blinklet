#![deny(rust_2018_idioms)]

mod error;
mod interpreter;
mod mark;
mod parser;

use interpreter::context::Context;

fn main() {
    let name = String::from("test");
    let code = String::from("greet you");
    let mut context = Context::default();
    let _ = context.run_code(&name, &code);
}
