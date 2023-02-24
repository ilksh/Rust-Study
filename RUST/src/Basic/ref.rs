fn val_change(num: &mut u32) {
    *num = 100;
}

fn main() {
    let mut num = 987654321;
    val_change(&mut num);
    println!("new num = {}", num);
}