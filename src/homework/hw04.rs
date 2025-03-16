const WIDTH: usize = 7;
const HEIGHT: usize = 7; 

fn main() {
   
    for i in 0..HEIGHT {
       
        for _ in 0..(HEIGHT - i - 1) {
            print!(" ");
        }
      
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!(); 
    }

    for i in (0..HEIGHT - 1).rev() {
       
        for _ in 0..(HEIGHT - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!(); 
    }
}
