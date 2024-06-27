// Kate and Eliana's summer project...
// building pente in rust mwahahahahahaha
use std::io;
use std::io::Write;

// struct to remember a player's information
#[derive(Debug)]
struct Player {
    num_captures: i32,
    five_in_a_row: bool
}

#[derive(Clone)]
enum BoardVal {
    Empty,
    Player1,
    Player2,
}

fn print_board(board: Vec<Vec<BoardVal>>) {
    // print column identifiers
    println!();
    print!("{:3}", 0);
    for col in 1..board.len() {
        print!("{:4}", col);
    }   
    println!();

    // print board rows
    for row in 0..board.len() {
        // print row identifier
        print!("{} ", (65 + row) as u8 as char);

        // print row contents
        for col in 0..board[row].len() {
            print!("{}", 
                match board[row][col] {
                    BoardVal::Empty => {
                        if row == 0 {
                            if col == 0 { "┌" }   
                            else if col == board.len()-1 { "┐" } 
                            else { "┬" }
                        }
                        else if row == board.len()-1 {
                            if col == 0 { "└" }   
                            else if col == board.len()-1 { "┘" } 
                            else { "┴" }
                        } 
                        else if col == 0 { "├" }
                        else if col == board.len()-1 { "┤" }
                        else { "┼" }
                    },
                    BoardVal::Player1 => "◎",
                    BoardVal::Player2 => "●",
                }
            );
            // don't print filler if on last column
            if col == board.len()-1 { continue; }
            print!("───");
        }
        println!();

        // if on last row, don't print blank line underneath
        if row == board.len()-1 { break; }

        // print blank lines
        print!("  ");
        for _col in 0..board[row].len() {
            print!("│   ")
        }
        println!();
    }
    println!();
}

// the heart of the game
fn main() {
    let mut bs: usize;
    loop {
        print!("Please enter the board size: ");
        io::stdout().flush().expect("Could not flush stdout");
        let mut board_size_str = String::new();
        io::stdin()
            .read_line(&mut board_size_str)
            .expect("Failed to read line");

        // I'm so sorry for how cursed this is lol
        bs = match board_size_str.trim().parse::<usize>() {
            Ok(x) => match x { 
                5..=50 => x, 
                _ => { eprintln!("Index entered was not within allowed range of 5 to 50"); 0 }, 
            },
            Err(_) => { eprintln!("Index entered was not an unsigned number"); 0 }, 
        };
        if bs != 0 { break; }
    }
    
    let mut board = vec![vec![BoardVal::Empty; bs]; bs];
    let mut coordinates: (&str, &str);
    let mut row: usize;
    let mut col: usize;
    let mut coord_str: String;
    loop {
        print!("Please enter a row and column in the form A8: ");
        io::stdout().flush().expect("Could not flush stdout");
        coord_str = String::new();
        io::stdin()
            .read_line(&mut coord_str)
            .expect("Failed to read line");
        if coord_str.len() <= 1 { continue; }
        coordinates = coord_str.trim().split_at(1);

        row = match coordinates.0.to_string().parse::<char>() {
            Ok(x) => { 
                if x.is_ascii_uppercase() { 
                    if x as usize >= 'A' as usize + bs {
                        eprintln!("Row entered was not within board size"); 
                        usize::MAX
                    } else {
                        x as usize - 'A' as usize
                    }
                } else {
                    eprintln!("Row entered was not an uppercase letter"); 
                    usize::MAX
                }
            },
            Err(_) => { eprintln!("Row entered was not a character"); usize::MAX },
        };

        col = match coordinates.1.to_string().parse::<usize>() {
            Ok(x) => { 
                if x >= bs { 
                    eprintln!("Col entered was not within board size"); 
                    usize::MAX
                } else {
                    x
                }
            },
            Err(_) => { eprintln!("Col entered was not an unsigned number"); usize::MAX },
        };
        
        if row != usize::MAX && col != usize::MAX { break; }
    }

    println!("You entered row {} column {}!", coordinates.0, coordinates.1);
    board[row][col] = BoardVal::Player1;

    print_board(board);

    let player1 = Player{num_captures: 3, five_in_a_row: true};
    println!("player 1: {:?}, \nnum captures: {}, 5? : {}", player1, player1.num_captures, player1.five_in_a_row);
}

