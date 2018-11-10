extern crate mcts;

use game::{Game, GameState};
use player::Player;
use std::{marker::PhantomData, time::Duration};

/// Matches our concept of a `Game` to that of `mcts::GameState`
#[derive(Clone)]
struct State<G> {
    game: G,
}

impl<G> mcts::GameState for State<G>
where
    G: Game,
    G::Move: Sync,
{
    type Move = G::Move;
    type Player = usize;
    type MoveList = Vec<Self::Move>;

    fn current_player(&self) -> Self::Player {
        match self.game.state() {
            GameState::PlayerTurn(p) => p,
            _ => 0, //panic!("Monte Carlo strategy requested active player for finished game."),
        }
    }

    fn available_moves(&self) -> Self::MoveList {
        self.game.legal_moves().collect()
    }

    fn make_move(&mut self, mov: &Self::Move) {
        self.game.play_move(mov)
    }
}

struct Spec<G> {
    g: PhantomData<G>,
}

impl<G> mcts::MCTS for Spec<G>
where
    G: Game + Sync,
    G::Move: Sync,
{
    type State = State<G>;
    type Eval = Heuristic;
    type NodeData = ();
    type ExtraThreadData = ();
    type TreePolicy = mcts::tree_policy::UCTPolicy;
}

struct Heuristic;

impl<G> mcts::Evaluator<Spec<G>> for Heuristic
where
    G: Game + Sync,
    G::Move: Sync,
{
    type StateEvaluation = GameState;

    fn evaluate_new_state(
        &self,
        state: &State<G>,
        moves: &Vec<G::Move>,
        _handle: Option<mcts::SearchHandle<Spec<G>>>,
    ) -> (Vec<()>, Self::StateEvaluation) {
        (vec![(); moves.len()], state.game.state())
    }

    fn evaluate_existing_state(
        &self,
        _state: &State<G>,
        evaln: &GameState,
        _handle: mcts::SearchHandle<Spec<G>>,
    ) -> GameState {
        *evaln
    }

    fn interpret_evaluation_for_player(&self, evaln: &GameState, player: &usize) -> i64 {
        match (evaln, player) {
            (GameState::VictoryPlayerOne, 0) => 1,
            (GameState::VictoryPlayerOne, 1) => -1,
            (GameState::VictoryPlayerTwo, 0) => -1,
            (GameState::VictoryPlayerTwo, 1) => 1,
            _ => 0,
        }
    }
}

/// An implementation of `Player` which can play any Game by using a Monte Carlo Search Tree.
pub struct MonteCarlo;

impl<G> Player<G> for MonteCarlo
where
    G: Game + Sync,
    G::Move: Sync,
{
    fn suggest(&self, game: &G) -> G::Move {
        let mut m = mcts::MCTSManager::new(
            State { game: game.clone() },
            Spec { g: PhantomData },
            Heuristic,
            mcts::tree_policy::UCTPolicy::new(0.5),
        );
        m.playout_parallel_for(Duration::from_millis(50), 4);
        // m.playout_n_parallel(10000, 4);
        m.best_move().expect("unable to find move in time")
    }
}
