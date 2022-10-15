mod game;
mod players;

use clap::{Parser, ValueEnum};
use game::Game;
use players::{HumanPlayer, MinMaxPlayer, Player, RandomPlayer};
use std::{fmt::Display, ops::AddAssign, sync::Arc, thread};

const GAME: Game = Game {};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(value_enum)]
    player_1: PlayerKind,
    #[arg(value_enum)]
    player_2: PlayerKind,
    game_cont: u64,
}

#[derive(Clone, ValueEnum)]
enum PlayerKind {
    Human,
    Random,
    MinMax,
}

struct GamesResult {
    victories: u64,
    draws: u64,
    losses: u64,
}

fn main() {
    let args = Arguments::parse();

    let player_1 = args.player_1.get_player();
    let player_2 = args.player_2.get_player();

    let mut games_results = GamesResult {
        victories: 0,
        draws: 0,
        losses: 0,
    };

    let available_parallelism = usize::min(
        std::thread::available_parallelism().unwrap().get(),
        args.game_cont as usize,
    );

    thread::scope(|s| {
        let mut handlers = Vec::with_capacity(available_parallelism);

        for _ in 0..available_parallelism {
            handlers.push(s.spawn(|| {
                play_games(
                    &*player_1.clone(),
                    &*player_2.clone(),
                    // NOTE: This code is not correct because it just truncates the division result,
                    // but it's fine for this application.
                    args.game_cont as usize / available_parallelism,
                )
            }))
        }

        for handler in handlers {
            games_results += handler.join().unwrap();
        }
    });

    print!("{games_results}")
}

fn play_games(player_1: &dyn Player, player_2: &dyn Player, n: usize) -> GamesResult {
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

impl PlayerKind {
    fn get_player(&self) -> Arc<dyn players::Player + Send + Sync> {
        match self {
            PlayerKind::Human => Arc::new(HumanPlayer {}),
            PlayerKind::Random => Arc::new(RandomPlayer {}),
            PlayerKind::MinMax => Arc::new(MinMaxPlayer),
        }
    }
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
        let game_count = self.victories + self.draws + self.losses;

        write!(
            f,
            "Win: {:.2}%, Draw: {:.2}%, Loss: {:.2}%, Game Count: {}",
            (self.victories as f64 / game_count as f64) * 100.0,
            (self.draws as f64 / game_count as f64) * 100.0,
            ((game_count - self.victories - self.draws) as f64 / game_count as f64) * 100.0,
            game_count
        )
    }
}
