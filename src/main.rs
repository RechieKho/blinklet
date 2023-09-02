mod parser;

fn main() {
    let code = String::from(
"
print 'Hello world'
let variable
    Add 2 3
fn something argument
    func_one 
    func_two
    func_three"
    );
    println!("{:#?}", parser::lexer::lex(&code).unwrap());
}
