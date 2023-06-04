use tople::die::lexer::Lexer;

fn main() {
    let f = Lexer::new("d20 + 100").unwrap();
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
}
