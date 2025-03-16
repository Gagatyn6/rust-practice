fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

fn main() {
    let a = 56; 
    let b = 98;
  
    let result = gcd(a, b);


    println!("GCD of {} and {} is {}", a, b, result);
}
