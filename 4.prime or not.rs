  fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let number_to_check = 17;
    if is_prime(number_to_check) {
        println!("{} is a prime number.", number_to_check);
    } else {
        println!("{} is not a prime number.", number_to_check);
    }
}

