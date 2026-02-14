use std::{i32, io,};

pub struct Player {
    name: String,
    symbol: char,
}

impl Player {
    pub fn new(name: String, symbol: char) -> Self {
        Self { name, symbol }
    }
}

pub struct Board {
    pub cells: [char; 9],
}

impl Board {
    pub fn new() -> Self {
        Self { cells: ['-'; 9] }
    }

    pub fn draw(&self) {
        for i in 0..3 {
            let row_start = i * 3;
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

    pub fn insert_symbol(&mut self, index: usize, symbol: char) -> bool {
        if index >= self.cells.len() || self.cells[index] != '-' {
            return false;
        }
        self.cells[index] = symbol;
        true
    }

    pub fn status(&self) -> GameStatus {
        let win_combinations = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for combo in win_combinations {
            let [a, b, c] = combo;
            if self.cells[a] != '-' && self.cells[a] == self.cells[b] && self.cells[b] == self.cells[c] {
                return GameStatus::Win(self.cells[a]);
            }
        }

        if !self.cells.contains(&'-') {
            return GameStatus::Draw;
        }

        GameStatus::InProgress
    }
}

#[derive(Clone, Copy)]
pub enum GameStatus {
    InProgress,
    Win(char),
    Draw,
}

pub struct Game {
    players: Vec<Player>,
    board: Board,
}

impl Game {
    pub fn new(players: Vec<Player>, board: Board) -> Self {
        Self { players, board }
    }
}

pub fn get_player_move(player_name: &str) -> usize {
    loop {
        println!("{}: Enter field (0-8)", player_name);

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(index) = input.trim().parse::<usize>() {
                if index < 9 {
                    return index;
                }
            }
        }
        println!("Invalid input. Choose a number between 0 and 8.");
    }
}

pub fn get_player_name(prompt:&str)-> Result<String,io::Error>{
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let name = input.trim();
    if name.is_empty(){
        Ok("Player".to_string())
    }
    else {
        Ok(name.to_string())
    }
}

pub fn player_vs_player()-> Result<(),io::Error>{
    let name1= get_player_name("Enter name for Player 1 (X):")?;
    let name2= get_player_name("Enter name for Player 2 (O):")?;
    let player1 = Player::new(name1, 'X');
    let player2 = Player::new(name2, 'O');
    let mut game = Game::new(vec![player1, player2], Board::new());
    let mut current_player = 0;
    let player = &game.players[current_player];
    let _index = get_player_move(&player.name);
    loop {
        game.board.draw();
        let player = &game.players[current_player];
        let index= get_player_move(&player.name);
        if game.board.insert_symbol(index, player.symbol){
            match game.board.status() {
                GameStatus::Win(symbol) => {
                    println!("{} wins!", symbol);
                    break;
                }
                GameStatus::Draw => {
                    println!("Draw!");
                    break;
                }
                GameStatus::InProgress => {
                    current_player  = (current_player +1)% 2;
                }
            }
        }
        else {
            println!("Invalid move please try again");
        }
    }
    Ok(())
}

pub fn player_vs_ai()->Result<(),io::Error>{
    let name1= get_player_name("Enter name for Player 1 (X):")?;
    let player1 = Player::new(name1, 'X');
    let ai_player = Player::new("AI".to_string(), 'O');
    let mut game = Game::new(vec![player1, ai_player], Board::new());
    let mut current_player = 0;
    let player = &game.players[current_player];
    let _index = get_player_move(&player.name);
    loop {
        game.board.draw();
        let player = &game.players[current_player];
        match game.board.status() {
            GameStatus::Win(symbol) => {
                println!("{} wins!", symbol);
                break;
            }
            GameStatus::Draw => {
                println!("Draw!");
                break;
            }
            GameStatus::InProgress => {}
        }

        if current_player == 0 {
            let index = get_player_move(&player.name);
            if game.board.insert_symbol(index, player.symbol) {
                current_player = 1;
            }
            else {
                println!("Invalid Input please try again");
            }
        } else {
            println!("Bot Move:");
            ai_move(&mut game.board, 'O', 'X');
            current_player = 0;
        }
    }
    Ok(())

}


pub fn minmax(board: &mut Board, depth: i32, is_maximizing: bool, ai_symbol: char, human_symbol: char) -> i32 {
    let status = board.status();

    match status {
        GameStatus::Win(winner) => {
            if winner == ai_symbol {
                return 10 - depth;
            } else if winner == human_symbol {
                return depth - 10;
            } else {
                return 0;
            }
        }
        GameStatus::Draw => return 0,
        GameStatus::InProgress => {}
    }

    if is_maximizing {
        let mut best_score = i32::MIN;
        for i in 0..9 {
            if board.cells[i] == '-' {
                board.cells[i] = ai_symbol;
                let score = minmax(board, depth + 1, false, ai_symbol, human_symbol);
                board.cells[i] = '-';
                best_score = best_score.max(score);
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for i in 0..9 {
            if board.cells[i] == '-' {
                board.cells[i] = human_symbol;
                let score = minmax(board, depth + 1, true, ai_symbol, human_symbol);
                board.cells[i] = '-';
                best_score = best_score.min(score);
            }
        }
        best_score
    }
}

pub fn ai_move(board: &mut Board, ai_symbol: char, human_symbol: char) {
    let mut best_score = i32::MIN;
    let mut best_move: Option<usize> = None;

    for i in 0..9 {
        if board.cells[i] == '-' {
            board.cells[i] = ai_symbol;
            let score = minmax(board, 0, false, ai_symbol, human_symbol);
            board.cells[i] = '-';

            if score > best_score {
                best_score = score;
                best_move = Some(i);
            }
        }
    }

    if let Some(best_index) = best_move {
        board.cells[best_index] = ai_symbol;
    }
}

fn main()-> Result<(),io::Error> {   
    println!("Select mode:");
    println!("1 for Player VS AI");
    println!("2 for Player VS Player");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "2" => player_vs_player()?,
        "1" => player_vs_ai()?,
        _ => println!("Invalid Entry"),
    }
    Ok(())
}   

