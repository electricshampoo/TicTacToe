use std::io;
use std::mem::{transmute};

#[deriving(PartialEq,Show)]
enum Player {
    X,
    O
}

struct Board([[Option<Player>,..3],..3]);

fn all_same(a:Option<Player>, b:Option<Player>, c:Option<Player>) -> bool{
    if a.is_none() || b.is_none() || c.is_none(){
        false
    }
    else {
        a.unwrap() == b.unwrap() && b.unwrap() == c.unwrap()
    }
}

impl Board {
    fn new() -> Board{
        Board([[None,None,None]
              ,[None,None,None]
              ,[None,None,None]
              ])
    }

    fn all_filled(&self) -> bool{
        let &Board(arr) = self;

        for row in arr.iter(){
            for slot in row.iter(){
                if slot.is_none(){
                    return false;
                }
            }
        }

        true
    }

    fn winner(&self) -> Option<Player>{
        let &Board(arr) = self;

        //check rows 
        for row in arr.iter(){
            if all_same(row[0], row[1], row[2]){
                return row[2];
            }
        }

        //check columns
        for i in range(0u,3){
            if all_same(arr[0][i], arr[1][i],arr[2][i]){
                return arr[2][i];
            }
        }

        //check top left to bottom right diagonal
        if all_same(arr[0][0],arr[1][1],arr[2][2]){
            return arr[2][2];
        }

        //check bottom left to top right diagonal
        if all_same(arr[0][2],arr[1][1],arr[2][0]){
            return arr[2][0];
        }

        None
    }

    fn print_board(&self){
        let &Board(arr) = self;
        for row in arr.iter(){
            for slot in row.iter(){
                if slot.is_none(){
                    print!("E");
                }
                else {
                    print!("{}", slot.unwrap());
                }
            }

            print!("\n");
        }
    }

    fn taken(&self, row:uint, col:uint) -> bool{
        let &Board(arr) = self;
        arr[row][col].is_some()
    }

    fn set(&mut self, row:uint, col:uint, player:Player){
        let arr = unsafe {transmute::<&mut Board, &mut [[Option<Player>, ..3],..3]>(self)};
        arr[row][col] = Some(player);
    }

    fn get_player_input(&mut self, player:Player) -> Option<Player>{
        let mut reader = io::stdin();
        loop {
            print!("Player {} where do you want to put your marker (row column): ", player);
            let input = reader.read_line().unwrap();

            let row = input.as_slice().char_at(0).to_digit(3).unwrap();
            let col = input.as_slice().char_at(2).to_digit(3).unwrap();

            if self.taken(row, col){
                println!("This spot is not empty. Try again.");
            }
            else {
                self.set(row, col, player);
                break;
            }
        }

        self.winner()
    }

    fn run_game_iteration(&mut self, player:Player) -> bool{
        self.print_board();

        if self.get_player_input(player).is_some(){
            println!("Congratulations player {}. You won!", player);
            self.print_board();
            return true;
        }
        else{
            if self.all_filled(){
                println!("The game was a tie.");
                return true;
            }
        }

        false
    }

    fn run_game(){
        let mut board = Board::new();

        loop {
            if board.run_game_iteration(X){
                break;
            }
            if board.run_game_iteration(O){
                break;
            }
        }
    }
}

fn main(){
    Board::run_game();
}
