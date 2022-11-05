mod commands;
mod game;
mod players;

use clap::{Parser, Subcommand, ValueEnum};
use erased_serde::{Deserializer, Serializer};
use game::Game;
use players::{HumanPlayer, MinMaxPlayer, Player, RandomPlayer};
use std::{fs::File, sync::Arc};

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

fn main() -> Result<(), RelearnError> {
    let args = Arguments::parse();

    match args.command {
        Commands::Play {
            player_1,
            player_2,
            game_count,
        } => {
            let mut player_1 = player_1.load_or_create_player()?;
            let mut player_2 = player_2.load_or_create_player()?;

            player_1.learn(&GAME);
            player_2.learn(&GAME);

            commands::play(GAME, Arc::from(player_1), Arc::from(player_2), game_count);
        }
        Commands::Learn { player } => match player {
            PlayerKind::Human => return Err(RelearnError::NonTrainablePlayer),
            PlayerKind::Random => return Err(RelearnError::NonTrainablePlayer),
            PlayerKind::MinMax => {
                let mut player = player.create_player();
                player.learn(&GAME);
                save_player(player)?;
            }
        },
    }

    Ok(())
}

impl PlayerKind {
    fn create_player(&self) -> Box<dyn players::Player + Sync + Send> {
        match self {
            PlayerKind::Human => Box::new(HumanPlayer {}),
            PlayerKind::Random => Box::new(RandomPlayer {}),
            PlayerKind::MinMax => Box::new(MinMaxPlayer::new()),
        }
    }

    fn load_or_create_player(
        &self,
    ) -> Result<Box<dyn players::Player + Sync + Send>, RelearnError> {
        match self.load_player() {
            Ok(player) => Ok(player),
            Err(err) => {
                if matches!(err, RelearnError::NonTrainablePlayer) {
                    return Ok(self.create_player());
                } else {
                    Err(err)
                }
            }
        }
    }
    fn load_player(&self) -> Result<Box<dyn players::Player + Sync + Send>, RelearnError> {
        let player: Box<dyn players::Player + Sync + Send> = match self {
            PlayerKind::Human => return Err(RelearnError::NonTrainablePlayer),
            PlayerKind::Random => return Err(RelearnError::NonTrainablePlayer),
            PlayerKind::MinMax => {
                let mut file = File::open("minmax.bin").map_err(|_| {
                    RelearnError::CreateFileError(
                        "The player file was not found, please run `relearn <PLAYER>` first"
                            .to_string(),
                    )
                })?;

                let mut deserializer = rmp_serde::Deserializer::new(&mut file);
                let mut deserializer: Box<dyn Deserializer> =
                    Box::new(<dyn Deserializer>::erase(&mut deserializer));
                let player: MinMaxPlayer = erased_serde::deserialize(&mut deserializer)
                    .map_err(|err| RelearnError::SerializeError(err.to_string()))?;
                Box::new(player)
            }
        };

        Ok(player)
    }
}

fn save_player(player: Box<dyn Player>) -> Result<(), RelearnError> {
    let mut file =
        File::create("minmax.bin").map_err(|err| RelearnError::CreateFileError(err.to_string()))?;

    let mut serializer = rmp_serde::Serializer::new(&mut file);
    let mut serializer: Box<dyn Serializer> = Box::new(<dyn Serializer>::erase(&mut serializer));

    player
        .erased_serialize(&mut serializer)
        .map_err(|err| RelearnError::SerializeError(err.to_string()))?;

    Ok(())
}

#[derive(Debug)]
pub enum RelearnError {
    CreateFileError(String),
    SerializeError(String),
    NonTrainablePlayer,
}
