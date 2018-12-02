use game::Game;
use player::Player;
use std::{fmt::Display, io, str::FromStr};

/// A player who can play Games those moves can be parsed from console
pub struct User;

impl<G> Player<G> for User
where
    G: Game,
    G::Move: FromStr,
    <G::Move as FromStr>::Err: Display,
{
    fn suggest(&self, _game: &G) -> G::Move {
        loop {
            println!("Enter move: ");
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Error reading input from stdin.");
            match buf.trim().parse() {
                Ok(mov) => break mov,
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}
