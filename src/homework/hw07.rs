fn main() {
    let input_string = String::from("Hello, World!"); 

    let lower_case = input_string.to_lowercase();
    println!("Верхній на нижній: {}", lower_case);

  
    let upper_case = input_string.to_uppercase();
    println!("Нижній на верхній: {}", upper_case);
}
