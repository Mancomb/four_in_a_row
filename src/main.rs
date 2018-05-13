extern crate term;

use std::fmt;
use std::io::prelude::*;

#[derive(PartialEq, Eq)]
enum Player {
    Red,
    Yellow
}

#[derive(PartialEq, Eq)]
enum Field {
    Empty,
    Stone(Player)
}

#[derive(Default)]
struct Board {
    fields: [[Field;7];6],
    num_stones: u8
}

impl Board {

    pub fn active_player(&self ) -> Player
    {
        if self.num_stones % 2 == 0 {
            Player::Red
        } else {
            Player::Yellow
        }
    }

    fn top_position(&self , column: u8) -> u8 
    {
        if let Some(i) = (0..6).into_iter().find(|&i| self.fields[i as usize][column as usize] != Field::Empty){
            i
        } else {
            6
        }
    }

    pub fn apply_move(&mut self, column: u8)
    {
        match self.top_position(column) {
            0 => panic!("Column already full"),
            row => {
                self.fields[row as usize - 1][column as usize] = Field::Stone(self.active_player());
                self.num_stones += 1;
            }
        }
    }
}

impl Default for Field{
    fn default() -> Field{
        Field::Empty
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut t = term::stdout().unwrap();
        let c = match self {
            Field::Empty => "   ",
            Field::Stone(Player::Red) => {
                t.fg(term::color::GREEN).unwrap();
                " O "
                },
            Field::Stone(Player::Yellow) => {
                t.fg(term::color::RED).unwrap();
                " O "
            }
        };
        write!(f, "{}", c).unwrap();
        t.reset().unwrap();
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.fields{
            write!(f, "|")?;
            for field in row{
                write!(f, "{}", field)?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "-----------------------\n")?;
        write!(f, "  0  1  2  3  4  5  6  \n")?;        
        Ok(())
    }
}

fn main() {    
    let mut board = Board::default();
    

    println!("Please enter a column (0-6): ");
    let input = std::io::stdin();
    let input = std::io::BufReader::new(input);
    println!("{}", board);

    for col in input.lines(){
        let col = col.expect("Invalid utf8").parse().expect("Input is not a number");
        board.apply_move(col);
        println!("{}", board);
    }
    
}
