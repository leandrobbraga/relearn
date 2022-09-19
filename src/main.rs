mod game;
mod player;

use std::{fmt::Display, ops::AddAssign, thread};

use game::{Game, TicTacToe};
use player::{Player, RandomPlayer};

static GAME_COUNT: u64 = 1_000_000_000;

struct GamesResult {
    victories: u64,
    draws: u64,
    losses: u64,
}

impl AddAssign for GamesResult {
    fn add_assign(&mut self, rhs: Self) {
        self.victories += rhs.victories;
        self.draws += rhs.draws;
        self.losses += rhs.losses;
    }
}

impl Display for GamesResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Win: {}, Draw: {}, Loss: {}, Game Count: {}",
            self.victories as f64 / GAME_COUNT as f64,
            self.draws as f64 / GAME_COUNT as f64,
            (GAME_COUNT - self.victories - self.draws) as f64 / GAME_COUNT as f64,
            self.losses + self.victories + self.draws
        )
    }
}

fn main() {
    let player_1 = RandomPlayer {};
    let player_2 = RandomPlayer {};

    let mut games_results = GamesResult {
        victories: 0,
        draws: 0,
        losses: 0,
    };
    let available_parallelism = std::thread::available_parallelism().unwrap().get();

    thread::scope(|s| {
        let mut handlers = Vec::with_capacity(available_parallelism);

        for _ in 0..available_parallelism {
            // NOTE: This code is not correct because it just truncates the division result,
            // but it's fine for this application.
            handlers.push(s.spawn(|| {
                play_games(
                    &player_1,
                    &player_2,
                    GAME_COUNT as usize / available_parallelism,
                )
            }))
        }

        for handler in handlers {
            games_results += handler.join().unwrap();
        }
    });

    print!("{games_results}")
}

fn play_games(player_1: &impl Player, player_2: &impl Player, n: usize) -> GamesResult {
    let mut victories = 0;
    let mut draws = 0;
    let mut losses = 0;

    let mut game = TicTacToe::new(player_1, player_2);

    for _ in 0..n {
        let result = game.play();

        match result {
            game::GameResult::Victory => victories += 1,
            game::GameResult::Draw => draws += 1,
            game::GameResult::Loss => losses += 1,
        };
    }

    GamesResult {
        victories,
        draws,
        losses,
    }
}
