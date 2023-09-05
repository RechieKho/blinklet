mod interpreter;
mod parser;

use interpreter::evaluator::EvaluationContext;

fn main() {
    let code = String::from("greet");
    let mut context = EvaluationContext::default();
    context.run_code(&code);
}
