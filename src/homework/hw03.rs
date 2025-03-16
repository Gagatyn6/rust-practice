const WIDTH: usize = 20;  
const HEIGHT: usize = 10; 

fn main() {
   
    for i in 0..WIDTH {
        print!("*");
    }
    println!();  
    
    for _ in 1..HEIGHT - 1 {
        print!("*");
        for _ in 1..WIDTH - 1 {
            print!(" ");
        }
        println!("*");
    }

   
    for i in 0..WIDTH {
        print!("*");
    }
    println!();  
}
