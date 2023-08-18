use std::io::{self, Write};
#[derive(Debug)]
enum Direction {
    North, South, West, East,
}
struct LangtonsAnt {
    board: Vec<Vec<char>>,
    size: usize,
    current_cell: (usize, usize, bool),
    direction: Direction
}

impl LangtonsAnt {
    fn new(size: usize, coordinates: (usize, usize)) -> LangtonsAnt {
        println!("\nCreating Board!");
        let mut board: Vec<Vec<char>> = Vec::new();
        for row in 0..size {
            let mut v: Vec<char> = vec![' '; size as usize];
            if row == coordinates.0 {
                v[coordinates.1] = '&';
            }
            board.push(v);
        }
        let current_cell = (
            coordinates.0,
            coordinates.1,
            true
        );
        let direction = Direction::North;
        return LangtonsAnt { 
            board,
            size,
            current_cell,
            direction
        };
    }

    fn print_board(&self) {
        println!("\nPrinting Board!");
        let border: Vec<char> = vec!['-'; self.size];
        println!(" {}", border.iter().collect::<String>());
        for row in &self.board {
            println!("|{}|", row.iter().collect::<String>());
        }
        println!(" {}", border.iter().collect::<String>());
        println!("Ant ({}, {}, {})",
        self.current_cell.0, self.current_cell.1, self.current_cell.2);
    }

    fn step(&mut self) {
        self._turn();
    }

    fn _turn(&mut self) {
        let cell_is_white = self.current_cell.2;
        self.direction = match self.direction {
            Direction::North => {
                if cell_is_white {
                    Direction::East
                } else {
                    Direction::West
                }
            },
            Direction::East => {
                if cell_is_white {
                    Direction::South
                } else {
                    Direction::North
                }
            },
            Direction::South => {
                if cell_is_white {
                    Direction::West
                } else {
                    Direction::East
                }
            },
            Direction::West => {
                if cell_is_white {
                    Direction::North
                } else {
                    Direction::South
                }
            }
        };
    }

    fn _change_cell_color(&mut self) {
        if self.current_cell.2 {
            self.current_cell.2 = false
        } else {
            self.current_cell.2 = true
        }
    }
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

fn get_ant_coordinates() -> (usize, usize) {
    // Prompt user for ant coordinates
    print!("\nProvide coordinates separated by 'space' (e.g '5 3'): ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(err) => println!("Buffer Flushing Error: {}", err)
    }
    // Get ant coordinates
    let mut input_coordinates = String::new();
    io::stdin()
    .read_line(&mut input_coordinates)
    .expect("Failed to read line");
    // Parse and return ant coordinates
    let vec_coordinates: Vec<_> = input_coordinates
    .trim()
    .split_whitespace()
    .collect();
    return (
        vec_coordinates[0].parse::<usize>().unwrap(),
        vec_coordinates[1].parse::<usize>().unwrap()
    );
}
fn main() {
    // Obtain board size and ant's coordinates
    let size = get_board_size();
    let coordinates = get_ant_coordinates();

    println!("\n--User input--\nBoard Size: {}\nCoordinates: {} {}",
    size, coordinates.0, coordinates.1);

    // Construct Langton's Ant
    let mut ant = LangtonsAnt::new(size, coordinates);

    // Print current state of board
    ant.print_board();

    ant.step();
    ant.print_board();
}