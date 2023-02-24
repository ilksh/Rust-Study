fn is_prime (n : usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false
        }
    }
    return true;
}

fn get_primes(primes: &mut[usize; 100]) {
    let mut i = 2;
    let mut cnt = 0;

    while cnt < 100 {
        if is_prime(i) {
            primes[cnt] = i;
            cnt += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut primes = [0; 100];

    get_primes(&mut primes);

    println!("{:?}", primes);
}