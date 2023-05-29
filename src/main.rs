use tople::die::formula::Formula;

fn main() {
    let f: Formula = "d20+100".parse().unwrap();
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
    println!("{}", f.calculate());
}
