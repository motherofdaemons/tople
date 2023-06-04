use tople::die::lexer::Lexer;

fn main() {
    let f: Lexer = "d20 + 100".parse().unwrap();
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
}
