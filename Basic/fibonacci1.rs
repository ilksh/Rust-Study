fn main() {
    let mut prev1 = 1; // declare mutable integer variable and assign value as 1
    let mut prev2 = 1; // declare mutable integer variable and assign value as 1
    
    println!("{}", prev1); // print the first term
    println!("{}", prev2); // print the second term

    for _ in 0..30 { // iterate 30 times to calculate fibonacci series
        let cur = prev1 + prev2; // declare immutable variable and assign values as prev1 + prev2
        println!("{}", cur); // print current value
       
        prev1 = prev2; // update information for the next
        prev2 = cur; // update information for the next
    } 
}