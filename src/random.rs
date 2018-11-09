extern crate rand;

use self::rand::prelude::*;
use game::Game;
use player::Player;

/// An implementation of `Player` which can play any Game by performing random moves.
pub struct Random;

impl<G: Game> Player<G> for Random {
    fn suggest(&self, game: &G) -> G::Move {
        let available: Vec<_> = game.legal_moves().collect();
        thread_rng()
            .choose(&available)
            .expect("No legal moves available")
            .clone()
    }
}
