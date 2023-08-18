use std::io::{self, Write};

struct LangtonsAnt {
    board: Vec<Vec<char>>,
    size: usize,
    coordinates: (usize, usize)
}

fn get_board_size() -> usize {
    // Prompt user for board size
    print!("\nProvide board size: ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(err) => println!("Buffer Flushing Error: {}", err)
    }
    // Get board size
    let mut input_size = String::new();
    io::stdin()
        .read_line(&mut input_size)
        .expect("Failed to read line");
    // Parse and return board size
    return input_size.trim().parse().unwrap();
}

fn main() {
    // Obtain board size and ant's coordinates
    let size = get_board_size();
}