mod commands;
mod game;
mod players;

use clap::{Parser, Subcommand, ValueEnum};
use game::Game;
use players::{HumanPlayer, MinMaxPlayer, Player, RandomPlayer};
use std::{fmt, fs::File, sync::Arc};

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
    Learn {
        player: PlayerKind,
    },
}
#[derive(Clone, ValueEnum)]
enum PlayerKind {
    Human,
    Random,
    MinMax,
}

fn main() -> Result<(), ReLearnError> {
    let args = Arguments::parse();

    match args.command {
        Commands::Play {
            player_1,
            player_2,
            game_count,
        } => {
            let mut player_1 = player_1.load_player()?;
            let mut player_2 = player_2.load_player()?;

            player_1.learn(&GAME);
            player_2.learn(&GAME);

            commands::play(GAME, Arc::from(player_1), Arc::from(player_2), game_count);
        }
        Commands::Learn { player } => {
            let mut player = player.create_player();
            player.learn(&GAME);
            player.save()?;
        }
    };

    Ok(())
}

impl PlayerKind {
    fn load_player(&self) -> Result<Box<dyn players::Player + Sync + Send>, ReLearnError> {
        match self {
            PlayerKind::Human | PlayerKind::Random => Ok(self.create_player()),
            PlayerKind::MinMax => {
                let file = File::open("minmax.bin")
                    .map_err(|err| ReLearnError::LoadAgentError(err.to_string()))?;

                let mut deserializer = rmp_serde::Deserializer::new(file);
                let player: MinMaxPlayer = serde::Deserialize::deserialize(&mut deserializer)
                    .map_err(|err| ReLearnError::LoadAgentError(err.to_string()))?;

                Ok(Box::new(player))
            }
        }
    }

    fn create_player(&self) -> Box<dyn players::Player + Sync + Send> {
        match self {
            PlayerKind::Human => Box::new(HumanPlayer {}),
            PlayerKind::Random => Box::new(RandomPlayer {}),
            PlayerKind::MinMax => Box::new(MinMaxPlayer::new()),
        }
    }
}

#[derive(Debug)]
pub enum ReLearnError {
    SaveAgentError(String),
    LoadAgentError(String),
}

impl fmt::Display for ReLearnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReLearnError::SaveAgentError(error_msg) => {
                write!(f, "Could not save the learned agent. Err: {error_msg}")
            }
            ReLearnError::LoadAgentError(error_msg) => {
                write!(f, "Could not load the agent. Err: {error_msg}")
            }
        }
    }
}
