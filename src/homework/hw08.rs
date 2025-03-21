fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; 
    }

    for i in 2..=((n as f64).sqrt() as u32) { 
        if n % i == 0 {
            return false;
        }
    }

    true 
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 16, 17, 18, 19, 20];

    for &num in &numbers {
        if is_prime(num) {
            println!("{} є простим числом.", num);
        } else {
            println!("{} не є простим числом.", num);
        }
    }
}
