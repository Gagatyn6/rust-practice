fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    if total % n != 0 {
        return None;
    }

    let target = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &load in shipments.iter() {
        balance += (load as i32 - target as i32);
        moves += balance.abs() as usize;
    }

    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let base: u32 = rng.gen_range(10..50);
    let target = base * n as u32;

    let mut shipments: Vec<u32> = vec![base; n];

    for i in 0..n {
        let adj: i32 = rng.gen_range(-5..5);
        let new_val = (shipments[i] as i32 + adj).max(0) as u32;
        shipments[i] = new_val;
    }

    shipments
}

fn main() {
    let shipments = gen_shipments(5);
    println!("Shipments: {:?}", shipments);

    match count_permutation(&shipments) {
        Some(moves) => println!("Мінімальна кількість переміщень: {}", moves),
        None => println!("Неможливо вирівняти вантажі"),
    }
}
