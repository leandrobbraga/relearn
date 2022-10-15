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
        } => commands::play(
            GAME,
            player_1.get_player(),
            player_2.get_player(),
            game_count,
        ),
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
