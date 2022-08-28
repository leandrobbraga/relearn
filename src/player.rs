use crate::game::Board;
use rand::prelude::*;
use std::io;

pub trait Player {
    fn play(&self, board: &Board, available_moves: Vec<usize>) -> usize;
}

#[derive(Clone)]
pub struct HumanPlayer;

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

pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn play(&self, _: &Board, available_moves: Vec<usize>) -> usize {
        *available_moves.choose(&mut rand::thread_rng()).unwrap()
    }
}
