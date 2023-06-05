use tople::die::{lexer::Lexer, parser::Parser};

fn main() {
    let lexer = Lexer::new("d20 + 100").unwrap();
    let parser = Parser::new(&lexer).unwrap();
    println!("{}", parser.parse());
    let parser = Parser::new(&lexer).unwrap();
    println!("{}", parser.parse());
}
