mod parser;

fn main() {
    let code = String::from(
"
fn something argument
    func_one 
    func_two
    func_three 3_arg 3arg 
    ensuing argument 4th 5th 6th"
    );
    println!("{:#?}", parser::tree::make_tree(&parser::lexer::lex(&code).unwrap()).unwrap());
}
