use std::io;
use std::mem::{transmute};

#[deriving(PartialEq,Show)]
enum Player {
    X,
    O
}

struct Board([[Option<Player>,..3],..3]);

impl Board {
    fn all_filled(&self) -> bool{
        let &Board(arr) = self;
        for row in arr.iter(){
            for column in row.iter(){
                if column.is_none() {
                    return false;
                }
            }
        }
        return true;
    }

    fn all_same(a:Option<Player>, b:Option<Player>, c:Option<Player>) -> bool{
        if a.is_none() || b.is_none() || c.is_none(){
            return false;
        }
        a.unwrap() == b.unwrap() && b.unwrap() == c.unwrap()
    }

    fn winner(&self) -> Option<Player>{
        let &Board(arr) = self;

        for row in arr.iter(){
            if Board::all_same(row[0], row[1], row[2]){
                return row[0];
            }
        }

        for i in range(0u,3){
            if Board::all_same(arr[0][i], arr[1][i],arr[2][i]){
                return arr[0][i];
            }
        }

        if Board::all_same(arr[0][0],arr[1][1],arr[2][2]){
            return arr[2][2];
        }

        if Board::all_same(arr[0][2],arr[1][1],arr[2][0]){
            return arr[2][0];
        }

        return None;
    }

    fn print_board(&self){
        let &Board(arr) = self;
        for row in arr.iter(){
            for column in row.iter(){
                if column.is_none(){
                    print!("E");
                }
                else {
                    print!("{}", column.unwrap());
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
}



fn main(){

    let mut board = Board([[None,None,None]
                          ,[None,None,None]
                          ,[None,None,None]
                          ]);

    loop {
        board.print_board();
        if board.get_player_input(X).is_some(){
            println!("Congratulations player X. You won!");
            board.print_board();
            break;
        }
        else{
            if board.all_filled(){
                println!("The game was a tie.");
                break;
            }
        }

        board.print_board();
        if board.get_player_input(O).is_some(){
            println!("Congratulations player O. You won!");
            board.print_board();
            break;
        }
        else {
            if board.all_filled(){
                println!("The game was a tie.");
                break;
            }
        }
    }
}
