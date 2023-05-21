use tople::{Die, DieFormula};

fn main() {
    let f: DieFormula = "d20".parse().unwrap();
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
}
