mod game;
mod player;

use std::sync::atomic::{AtomicU64, Ordering};

use game::Game;
use player::{Player, RandomPlayer};
use rayon::prelude::*;

fn main() {
    let player_1 = RandomPlayer {};
    let player_2 = RandomPlayer {};

    let victories = AtomicU64::new(0);
    let draws = AtomicU64::new(0);
    let count = 1_000_000_000;

    (0..count).into_par_iter().for_each(|_| {
        let winner = play_game(&player_1, &player_2);

        match winner {
            Some(game::Player::X) => {
                victories.fetch_add(1, Ordering::Relaxed);
            }
            None => {
                draws.fetch_add(1, Ordering::Relaxed);
            }
            _ => (),
        };
    });

    let victories = victories.load(Ordering::Relaxed);
    let draws = draws.load(Ordering::Relaxed);

    println!(
        "Win: {}, Draw: {}, Loss: {}",
        victories as f64 / count as f64,
        draws as f64 / count as f64,
        (count - victories - draws) as f64 / count as f64
    )
}

/// Play a single game and return the winner for the match
fn play_game<T, U>(player_1: &T, player_2: &U) -> Option<game::Player>
where
    T: Player,
    U: Player,
{
    let players: [Box<&dyn Player>; 2] = [Box::new(player_1), Box::new(player_2)];
    let mut game = Game::new();

    let mut player_idx = 0;

    while !game.has_finished() {
        let player = &players[player_idx % 2];
        let action = player.play(&game.board, game.available_actions());
        if game.act(action).is_ok() {
            player_idx += 1;
        };
    }

    game.winner()
}
