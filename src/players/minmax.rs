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
use crate::game::{self, Game, State};

use super::Player;

pub struct MinMaxPlayer;

impl Player for MinMaxPlayer {
    fn play(&self, game: &Game, state: &State, player: &game::Player) -> usize {
        self.search(game, state, player)
    }
}

impl MinMaxPlayer {
    fn search(&self, game: &Game, state: &State, player: &game::Player) -> usize {
        let (_, action) = self.maximize(game, state, player);

        // SAFETY: Only terminal states have `None` as the action, but in terminal states the game
        // is already finished.
        unsafe { action.unwrap_unchecked() }
    }

    fn maximize(&self, game: &Game, state: &State, player: &game::Player) -> (i64, Option<usize>) {
        if let game::Status::Finished(maybe_winner) = game.status(state) {
            return (self.utility(maybe_winner, player), None);
        }

        // We use a value that any move will surpass, just to initialize the variable
        let mut highest_value = -10;
        let mut best_move: Option<usize> = None;

        for action in game.available_moves(state) {
            let mut next_state = state.clone();

            // SAFETY: we draw the actions from the `available_moves` method
            unsafe {
                game.act(player.clone(), *action, &mut next_state)
                    .unwrap_unchecked()
            };

            let (action_value, _) = self.minimize(game, &next_state, player);

            if action_value > highest_value {
                highest_value = action_value;
                best_move = Some(*action);
            }

            // Because we defined in the `utility` function that there is no value higher than 1, we
            // can stop searching here, as we won't find a better move.
            if highest_value == 1 {
                break;
            }
        }

        (highest_value, best_move)
    }

    fn minimize(&self, game: &Game, state: &State, player: &game::Player) -> (i64, Option<usize>) {
        if let game::Status::Finished(maybe_winner) = game.status(state) {
            return (self.utility(maybe_winner, player), None);
        }

        let mut lowest_value = 10;
        let mut worst_move: Option<usize> = None;

        for action in game.available_moves(state) {
            let mut next_state = state.clone();
            game.act(player.next_player(), *action, &mut next_state)
                .unwrap();
            let (action_value, _) = self.maximize(game, &next_state, player);

            if action_value < lowest_value {
                lowest_value = action_value;
                worst_move = Some(*action);
            }

            // Because we defined in the `utility` function that there is no value lower than 1, we
            // can stop searching here, as we won't find a better opponent move.
            if lowest_value == -1 {
                break;
            }
        }

        (lowest_value, worst_move)
    }

    fn utility(&self, maybe_winner: Option<game::Player>, player: &game::Player) -> i64 {
        match maybe_winner {
            Some(winner) => {
                if winner == *player {
                    1
                } else {
                    -1
                }
            }
            None => 0,
        }
    }
}
