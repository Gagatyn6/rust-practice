fn main() {
    let triangles = 5;
    
    (1..=triangles).for_each(|i| {
     
        (0..i).for_each(|j| {
          
            (0..(triangles - i + j)).for_each(|_| print!(" "));
            
            (0..(2 * j + 1)).for_each(|_| print!("*"));
            println!();
        });
    });
}
