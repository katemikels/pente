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

fn print_board(board: Vec<Vec<char>>){
    println!("Contents of the board:");
    for row in board {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    // for row in 0..board.len() {
    //     for col in 0..board[row].len(){
    //         print!("{}", board[row][col]);
    //     }
    //     println!("");
    // }
}

// the heart of the game
fn main() {
    // argument code?
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let tam: &i32 = &args[2].parse::<usize>().unwrap();

    let mut bs: usize;
    loop {
        print!("Please enter the board size: ");
        io::stdout().flush().expect("Could not flush stdout");
        let mut board_size_str = String::new();
        io::stdin()
            .read_line(&mut board_size_str)
            .expect("Failed to read line");

        // let bs: usize = board_size_str
        //     .trim()
        //     .parse()
        //     .expect("Index entered was not a number");
    
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

    // let collected_iterator: Vec<i32> = (0..bs).collect();
    // println!("Collected (0..10) into: {:?}", collected_iterator);
    // let mut board:Vec<Vec<char>> = vec![vec![]];
    // let mut board:Vec<Vec<char>> = Vec::new();
    
    let mut board = vec![vec!['.'; bs]; bs];

    // for i in 0..bs {
    //     board.push(vec![]);
    //     for _j in 0..bs {
    //         board[i].push('.');
    //     }
    // }

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
                        x as usize
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
    // let coordinates= coord_str.trim();
    // let row = &coordinates[0..1];
    // let col = &coordinates[1..2];
    println!("You entered row {} column {}!", coordinates.0, coordinates.1);

    print_board(board);

    let player1 = Player{num_captures: 3, five_in_a_row: true};
    println!("player 1: {:?}, \nnum captures: {}, 5? : {}", player1, player1.num_captures, player1.five_in_a_row);
}

