use std::{io, ops::Index};

pub struct Player{
name:String,
symbol:char,
}

impl Player{
    pub fn new (name:String,symbol:char)-> Self {
        Self{
            name,
            symbol,
        }
    }
}


pub struct Board {
    cells:[char;9],
    }


impl Board{
    pub fn new()-> Self{
       Board {
        cells:['-';9],
       }
    }

    pub fn draw(&self) {
    for i in 0..3 {
        let row_start =i *3;
        println!(
            " {} | {} | {} ",
            self.cells[row_start],
            self.cells[row_start + 1],
            self.cells[row_start + 2]
        );
        if i < 2 {
            println!("---+---+---");
        }
    }
}

    
    pub fn insert_symbol(&mut self, index:usize,symbol:char)-> bool{
        if index >= self.cells.len(){
        println! ("Board is too small");
        return false;
        
        }
        if self.cells[index] != ('-'){
        println! ("Cell isn't empty");
        return false;
        }
        self.cells[index] = symbol;
        println!("symbol set");
        return true;
    }
    
}

pub struct Game{
    status :String,
    players:Vec<Player>,
    board:Board,
}

impl Game {
    pub fn new(players:Vec<Player> , board:Board,) -> Self{
        return Self { status: "Not_Started".to_string(), players, board };
    }
}




    pub fn get_player_move (player_name: String)-> usize{
        loop {
            println!("Enter Field");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.trim().parse::<usize>() {
                        Ok(index) if index <9 => return index,
                        _ => println!("invalid input choose a number between 0 and 8"),
                        
                    }
                }
                Err(_)=> println!("Error with Input, please try again"),
            }
        }
    }


    pub fn check_game_status (cells:&[char;9])-> String{
        let win_com = [
            [0,1,2],
            [3,4,5],
            [6,7,8],
            [0,3,6],
            [1,4,7],
            [2,5,8],
            [0,4,8],
            [2,4,6],
        ];
        for combo in win_com.iter(){
            let [a,b,c]= *combo;
            if cells[a] != '-' && cells[a] == cells[b] && cells[b]== cells[c]{
                println!("WIN");
                return format!("{} wins",cells[a]);
            }
        }
        if !cells.contains(&'-'){
            println!("DRAW");
            return "Draw".to_string();
        }
        "In Progress".to_string()
    }
    






fn main() {
    let board = Board::new();
    let player1 = Player::new("Elon".to_string(), 'X');
    let player2 = Player::new("Epstein".to_string(), 'O');

    let mut game = Game::new(vec![player1, player2], board);
    game.status = "In Progress".to_string();

    let mut current_player = 0;

    loop {
        let player = &game.players[current_player];

        game.board.draw();

        let status = check_game_status(&game.board.cells);
        if status != "In Progress" {
            println!("{}", status);
            break;
        }

        let index = get_player_move(player.name.clone());

        if game.board.insert_symbol(index, player.symbol) {
            current_player = (current_player + 1) % game.players.len();
        } else {
            println!("Invalid move, try again.");
        }
    }
}



