use bitboard::Bitboard;
use game::{Game, GameState};
use std::{io, str::FromStr};

/// An integer ranging from 0 to 6
#[derive(Clone)]
pub struct Column(u8);

impl FromStr for Column {
    type Err = &'static str;
    fn from_str(source: &str) -> Result<Column, Self::Err> {
        match source.as_bytes().first() {
            Some(v @ b'0'...b'6') => Ok(Column(v - b'0')),
            _ => Err("Only digits from 0 to 6 count as valid moves."),
        }
    }
}

/// State of a field in a four in a row board
#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    /// Field is not captured by either player
    Empty,
    /// Field contains a stone from Player 1
    PlayerOne,
    /// Field contains a stone from Player 1
    PlayerTwo,
}

#[derive(Clone)]
pub struct ConnectFour {
    /// One bitboard for each player
    bitboards: [Bitboard; 2],
    /// Number of turns played
    turns: usize,
}

impl ConnectFour {
    /// Create an empty board
    pub fn new() -> ConnectFour {
        ConnectFour {
            bitboards: [Bitboard::new(); 2],
            turns: 0,
        }
    }

    /// Prints out a text representation of a board to `out`
    pub fn print_to(&self, mut out: impl io::Write) {
        for row in (0..6).rev() {
            for field in (0..7).map(|column| self.field(row, column)) {
                let c = match field {
                    Field::PlayerOne => 'X',
                    Field::PlayerTwo => 'O',
                    Field::Empty => ' ',
                };
                write!(out, "|{}", c);
            }
            writeln!(out, "|");
        }
        writeln!(out, "---------------\n 0 1 2 3 4 5 6");
    }

    fn field(&self, row: u8, column: u8) -> Field {
        if !self.bitboards[0].is_empty(row, column) {
            Field::PlayerOne
        } else if !self.bitboards[1].is_empty(row, column) {
            Field::PlayerTwo
        } else {
            Field::Empty
        }
    }
}

impl Game for ConnectFour {
    /// Moves are represented by integers from 0 to 6
    type Move = Column;

    /// Returns an iterator over all possible legal_moves.
    fn legal_moves<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Move> + 'a> {
        Box::new(
            (0..7)
                .filter(move |&i| self.field(5, i) == Field::Empty)
                .map(Column),
        )
    }

    /// Inserts a stone for the current player. Panics on illegal moves.
    fn play_move(&mut self, &Column(mov): &Self::Move) {
        let free = (0..6)
            .find(|&row| self.field(row, mov) == Field::Empty)
            .expect("Illegal Move: Column full");
        self.bitboards[self.turns % 2].place_stone(free, mov);
        self.turns += 1;
    }

    fn state(&self) -> GameState {
        if self.bitboards[0].is_win() {
            GameState::VictoryPlayerOne
        } else if self.bitboards[1].is_win() {
            GameState::VictoryPlayerTwo
        } else if self.turns == 7 * 6 {
            GameState::Draw
        } else {
            GameState::PlayerTurn(self.turns % 2)
        }
    }
}
