// Kate and Eliana's summer project...
// building pente in rust mwahahahahahaha
use std::io;

// struct to remember a player's information
#[derive(Debug)]
struct Player {
    num_captures: i32,
    five_in_a_row: bool
}

fn print_board(board: Vec<Vec<char>>){
    println!("Contents of the board:");
    for row in 0..board.len() {
        for col in 0..board[row].len(){
            print!("{}", board[row][col]);
        }
        println!("");
    }
}

// the heart of the game
fn main() {
    // argument code?
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let tam: &i32 = &args[2].parse::<usize>().unwrap();

    let mut board_size_str: String = String::new();
    io::stdin()
        .read_line(&mut board_size_str)
        .expect("Failed to read line");

    let bs: usize = board_size_str
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // let collected_iterator: Vec<i32> = (0..bs).collect();
    // println!("Collected (0..10) into: {:?}", collected_iterator);
    // let mut board:Vec<Vec<char>> = vec![vec![]];
    let mut board:Vec<Vec<char>> = Vec::new();
    for i in 0..bs {
        board.push(vec![]);
        for _j in 0..bs {
            board[i].push('.');
        }
    }

    print!("Please enter a row and column in the form A8: ");
    let mut coord_str: String = String::new();
    io::stdin()
        .read_line(&mut coord_str)
        .expect("Failed to read line");
       
    let coordinates = coord_str.trim();
    let row = &coordinates[0..1];
    let col = &coordinates[1..2];
    println!("You entered row {} column {}!", row, col);

    print_board(board);

    let player1 = Player{num_captures: 3, five_in_a_row: true};
    print!("player 1: {:?}, \nnum captures: {}, 5? : {}", player1, player1.num_captures, player1.five_in_a_row);
}

