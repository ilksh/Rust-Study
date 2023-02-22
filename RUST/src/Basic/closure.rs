// clousre (Python: Lambda, Rust: closure)
// let name = |parameter| defintion;
fn main() {
    let plus2 = |n| n + 2; // decalre clousre with parameter of n
    println!("{}", plus2(8));
    println!("{}", plus2(-2));
}
