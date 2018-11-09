#[derive(PartialEq, Eq)]
pub enum GameState {
    /// The game is not yet finished. The index indicates those players turn it is. Counting starts
    /// at `0`.
    PlayerTurn(usize),
    /// The game has finished with a draw
    Draw,
    /// The first player won
    VictoryPlayerOne,
    /// The second player won
    VictoryPlayerTwo,
}

/// A null sum game which can be solved with monte carlo search trees and heuristics.
pub trait Game {
    type Move: Clone;

    /// Obtains iterator over list of legal moves
    fn legal_moves<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Move> + 'a>;

    /// Game State
    fn state(&self) -> GameState;

    /// Play a move
    fn play_move(&mut self, mov: &Self::Move);
}
