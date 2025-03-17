use rand::Rng;
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32, i32, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    Some((min_index, data[min_index], data[min_index + 1], min_sum))
}

fn print_results() {
    let vec = gen_random_vector(20);

    print!("indexes: ");
    for i in 0..vec.len() {
        print!("{:2}. ", i);
    }
    println!();

    println!("data:   {:?}", vec);

    if let Some((idx, a, b, sum)) = min_adjacent_sum(&vec) {
        println!("indexes: {}\\__ __/{:width$}", " ".repeat(idx * 4), "", width = (vec.len() - idx - 2) * 4);
        println!("min adjacent sum={}+{}={} at indexes:{},{}", a, b, sum, idx, idx + 1);
    } else {
        println!("Масив занадто малий для пошуку пари.");
    }
}

fn main() {
    print_results();
}
