use crate::game::Board;
use std::io;

pub struct HumanPlayer;
pub struct RandomPlayer;

pub trait Player {
    fn play(&self, board: &Board, available_moves: Vec<usize>) -> usize;
}

impl Player for HumanPlayer {
    fn play(&self, board: &Board, available_moves: Vec<usize>) -> usize {
        println!("{board}");
        println!("Available moves: {available_moves:?}");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let action: usize = buf.trim().parse().unwrap();

        action
    }
}

impl Player for RandomPlayer {
    fn play(&self, _: &Board, available_moves: Vec<usize>) -> usize {
        let i = fastrand::usize(..available_moves.len());
        available_moves[i]
    }
}
