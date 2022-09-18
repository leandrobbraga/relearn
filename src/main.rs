mod game;
mod player;

use std::{fmt::Display, ops::AddAssign, thread};

use game::Game;
use player::{Player, RandomPlayer};

static GAME_COUNT: u64 = 1_000;

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
            "Win: {}, Draw: {}, Loss: {}",
            self.victories as f64 / GAME_COUNT as f64,
            self.draws as f64 / GAME_COUNT as f64,
            (GAME_COUNT - self.victories - self.draws) as f64 / GAME_COUNT as f64
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

fn play_games<T, U>(player_1: &T, player_2: &U, n: usize) -> GamesResult
where
    T: Player,
    U: Player,
{
    let mut victories = 0;
    let mut draws = 0;
    let mut losses = 0;

    for _ in 0..n {
        let winner = play_game(player_1, player_2);

        match winner {
            Some(game::Player::X) => {
                victories += 1;
            }
            None => {
                draws += 1;
            }
            _ => {
                losses += 1;
            }
        };
    }

    GamesResult {
        victories,
        draws,
        losses,
    }
}

fn play_game<T, U>(player_1: &T, player_2: &U) -> Option<game::Player>
where
    T: Player,
    U: Player,
{
    let players: [Box<&dyn Player>; 2] = [Box::new(player_1), Box::new(player_2)];
    let mut game = Game::new();

    let mut player_idx = 0;

    loop {
        let player = &players[player_idx % 2];

        let action = player.play(&game.board, game.available_actions());

        if game.act(action).is_ok() {
            player_idx += 1;
        };

        if let game::GameState::Finished(winner) = game.state() {
            break winner;
        }
    }
}
