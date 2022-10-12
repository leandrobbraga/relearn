mod game;
mod player;

use std::{fmt::Display, ops::AddAssign, thread};

use game::{Game, TicTacToe};
use player::{MinMaxPlayer, Player, RandomPlayer};

const GAME: TicTacToe = TicTacToe {};
const PLAYER_1: MinMaxPlayer = MinMaxPlayer {};
const PLAYER_2: RandomPlayer = RandomPlayer {};
const GAME_COUNT: u64 = 10_000;

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
            "Win: {:.2}%, Draw: {:.2}%, Loss: {:.2}%, Game Count: {}",
            (self.victories as f64 / GAME_COUNT as f64) * 100.0,
            (self.draws as f64 / GAME_COUNT as f64) * 100.0,
            ((GAME_COUNT - self.victories - self.draws) as f64 / GAME_COUNT as f64) * 100.0,
            self.losses + self.victories + self.draws
        )
    }
}

fn main() {
    let mut games_results = GamesResult {
        victories: 0,
        draws: 0,
        losses: 0,
    };

    let available_parallelism = usize::min(
        std::thread::available_parallelism().unwrap().get(),
        GAME_COUNT as usize,
    );

    thread::scope(|s| {
        let mut handlers = Vec::with_capacity(available_parallelism);

        for _ in 0..available_parallelism {
            // NOTE: This code is not correct because it just truncates the division result,
            // but it's fine for this application.
            handlers.push(s.spawn(|| {
                play_games(
                    &PLAYER_1,
                    &PLAYER_2,
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

    let mut first_player = false;

    for _ in 0..n {
        // We alternate the players
        first_player = !first_player;

        let result = match first_player {
            true => GAME.play(player_1, player_2),
            false => GAME.play(player_2, player_1),
        };

        match result {
            Some(player) => match player {
                game::Player::X if first_player => victories += 1,
                game::Player::X => losses += 1,
                game::Player::O if !first_player => victories += 1,
                game::Player::O => losses += 1,
            },
            None => draws += 1,
        };
    }

    GamesResult {
        victories,
        draws,
        losses,
    }
}
