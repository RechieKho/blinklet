mod mark;
mod error;
mod interpreter;
mod parser;

use interpreter::object::Object;

fn main() {
    let code = String::from("greet you");
    let mut context = Object::default();
    context.run_code(&code);
}
