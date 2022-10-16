mod commands;
mod game;
mod players;

use clap::{Parser, Subcommand, ValueEnum};
use game::Game;
use players::{HumanPlayer, MinMaxPlayer, Player, RandomPlayer};
use std::sync::Arc;

const GAME: Game = Game {};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Plays a desired number of games and output the result
    Play {
        #[arg(value_enum)]
        player_1: PlayerKind,
        #[arg(value_enum)]
        player_2: PlayerKind,
        game_count: u32,
    },
}
#[derive(Clone, ValueEnum)]
enum PlayerKind {
    Human,
    Random,
    MinMax,
}

fn main() {
    let args = Arguments::parse();

    match args.command {
        Commands::Play {
            player_1,
            player_2,
            game_count,
        } => {
            let mut player_1 = player_1.create_player();
            let mut player_2 = player_2.create_player();

            player_1.learn(&GAME);
            player_2.learn(&GAME);

            commands::play(GAME, Arc::from(player_1), Arc::from(player_2), game_count)
        }
    }
}

impl PlayerKind {
    fn create_player(&self) -> Box<dyn players::Player + Send + Sync> {
        match self {
            PlayerKind::Human => Box::new(HumanPlayer {}),
            PlayerKind::Random => Box::new(RandomPlayer {}),
            PlayerKind::MinMax => Box::new(MinMaxPlayer::new()),
        }
    }
}
