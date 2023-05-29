use tople::DieFormula;

fn main() {
    let f: DieFormula = "d20+100".parse().unwrap();
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
}
