use std::{collections::HashMap, fs::File};

use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

/// The `Min-Max` algorithm is a naive solution for two-player, zero-sum, turn-taking games.
///
/// The algorithm works by exploring the state space graph alternating between maximization
/// (the player's turn) and minimization (the opponent's turn) steps until if finds the terminal
/// state with the highest `utility`. It chooses the `action` that leads to it.
///
/// This algorithm is unsuitable for large search space games as it needs to explore all
/// the possible states before taking a decision, which become unfeasible very fast.
///
/// NOTE: This algorithm was customized to stop evaluating upon reaching the first terminal state
/// with victory as it's not possible to have any higher score.
use crate::{
    game::{self, Game, State},
    ReLearnError,
};

use super::Player;

pub const FILE: &str = "minmax.bin";

#[derive(Serialize, Deserialize)]
pub struct MinMaxPlayer {
    knowledge: HashMap<State, u8>,
}

impl Player for MinMaxPlayer {
    fn play(&self, _: &Game, state: &State, _: game::Player) -> u8 {
        // SAFETY: We always train the player before playing
        unsafe { *self.knowledge.get(state).unwrap_unchecked() }
    }

    fn learn(&mut self, game: &Game) {
        let state = State::new();
        let player = game::Player::X;

        self.maximize(game, state, player);
    }

    fn save(&self) -> Result<(), ReLearnError> {
        let mut file =
            File::create(FILE).map_err(|err| ReLearnError::SaveAgentError(err.to_string()))?;

        // We use the `rmp_serde` instead of `serde_json` for two reasons:
        // 1. It's a compact format, reducing the learned agent size in disk
        // 2. `serde_json` has a limitation with HashMap<K,V>. The default
        //    serialization/deserialization implementation expects `K` to be `String`, which is not
        //    the case. It's possible to implement a customized serialization/deserialization
        //    function, but it was too much of a hassle.
        self.serialize(&mut Serializer::new(&mut file))
            .map_err(|err| ReLearnError::SaveAgentError(err.to_string()))
    }
}

impl MinMaxPlayer {
    pub(crate) fn new() -> Self {
        MinMaxPlayer {
            knowledge: HashMap::new(),
        }
    }

    fn maximize(&mut self, game: &Game, state: State, player: game::Player) -> (i64, Option<u8>) {
        if let game::Status::Finished(maybe_winner) = game.status(&state) {
            return (self.utility(maybe_winner, player), None);
        }

        // We use a value that any move will surpass, just to initialize the variable
        let mut highest_value = -10;
        let mut best_move: Option<_> = None;

        for &action in game.available_moves(&state) {
            let mut next_state = state.clone();

            // SAFETY: we draw the actions from the `available_moves` method
            unsafe { game.act(player, action, &mut next_state).unwrap_unchecked() };

            let (action_value, _) = self.minimize(game, next_state, player);

            if action_value > highest_value {
                highest_value = action_value;
                best_move = Some(action);
            }
        }

        // SAFETY: Only terminal states have `None` as the action, but in terminal states the game
        // is already finished.
        let action = unsafe { best_move.unwrap_unchecked() };
        self.knowledge.insert(state, action);

        (highest_value, best_move)
    }

    fn minimize(&mut self, game: &Game, state: State, player: game::Player) -> (i64, Option<u8>) {
        if let game::Status::Finished(maybe_winner) = game.status(&state) {
            return (self.utility(maybe_winner, player), None);
        }

        let mut lowest_value = 10;
        let mut worst_move: Option<_> = None;

        for &action in game.available_moves(&state) {
            let mut next_state = state.clone();
            // SAFETY: we draw the actions from the `available_moves` method
            unsafe {
                game.act(player.next_player(), action, &mut next_state)
                    .unwrap_unchecked()
            };
            let (action_value, _) = self.maximize(game, next_state, player);

            if action_value < lowest_value {
                lowest_value = action_value;
                worst_move = Some(action);
            }
        }

        // SAFETY: Only terminal states have `None` as the action, but in terminal states the game
        // is already finished.
        let action = unsafe { worst_move.unwrap_unchecked() };
        self.knowledge.insert(state, action);

        (lowest_value, worst_move)
    }

    fn utility(&self, maybe_winner: Option<game::Player>, player: game::Player) -> i64 {
        match maybe_winner {
            Some(winner) => {
                if winner == player {
                    1
                } else {
                    -1
                }
            }
            None => 0,
        }
    }
}
