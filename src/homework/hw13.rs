use std::collections::HashSet;

fn calculate_occupied_area(rectangles: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut occupied_cells = HashSet::new();

    for &((x1, y1), (x2, y2)) in rectangles.iter() {
        for x in x1..x2 {
            for y in y1..y2 {
                occupied_cells.insert((x, y));
            }
        }
    }

    occupied_cells.len() as u32
}

fn main() {
    let rectangles = vec![
        ((1, 1), (4, 7)),  
        ((3, 2), (13, 4)),
        ((5, 1), (9, 9)),  
    ];

    let total_area = calculate_occupied_area(&rectangles);
    println!("Загальна зайнята площа: {}", total_area);
}
