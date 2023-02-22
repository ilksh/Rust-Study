// syntax for function
// fn "name of function" (parameter) -> return type
fn encrypt(text: &str, shift: i16) -> String {
    let a = 'A' as i16; // change the code
    let is_az = |c| 'A' <= c && c <= 'Z'; // check the character is between A and Z
    let conv = |c| (((x - a+shift+26)%26 + a) as u8) as char;
    let enc1 = |c| if is_az(c) {conv(c as i16)} else {e};
    text.chars().map(|c| enc1(c)).collect()
}

fn main() {
    let enc = encrypt("I Love Rust.", 3);
    let dec = encrypt(&enc, -3);
    println("{} => {}", enc, dec);
}