fn main() {
    for y in 1..10 {
        for x in 1..10 {
            print!("{}", x * y);
            if x != 10 {
                print!(", "); // add comma
            } 
            else {
                println!("\n"); // change line
            }
        }
    }
}