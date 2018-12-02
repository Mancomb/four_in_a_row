mod bitboard;
mod connect_four;
mod game;
mod monte_carlo;
mod player;
mod random;
mod user;

use connect_four::ConnectFour;
use game::{Game, GameState};
use player::Player;
use random::Random;
use std::io;
use user::User;

fn main() {
    let mut game = ConnectFour::new();

    // Change these variables to set Players either to User, Random or Search Tree
    let player_one = User;
    let player_two = monte_carlo::MonteCarlo; //Random;

    let players: [&dyn Player<_>; 2] = [&player_one, &player_two];

    game.print_to(io::stdout());
    let result = loop {
        match game.state() {
            GameState::PlayerTurn(p) => {
                let mov = players[p].suggest(&game);
                game.play_move(&mov);
                game.print_to(io::stdout());
                continue;
            }
            GameState::Draw => break "Draw",
            GameState::VictoryPlayerOne => break "Victory Player One",
            GameState::VictoryPlayerTwo => break "Victory Player Two",
        }
    };
    println!("{}", result);
}
