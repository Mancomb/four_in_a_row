use game::Game;

/// Indicates a player who can play a particular `Game`
pub trait Player<G: Game> {
    /// Suggest a move to take.
    fn suggest(&self, game: &G) -> G::Move;
}
