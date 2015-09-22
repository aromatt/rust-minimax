extern crate minimax;

use minimax::{Game, Minimax};
use std::vec::Vec;

struct TestGame;

impl Game<i32, i32, f32> for TestGame {
    fn get_moves(&self, root: i32) -> Vec<i32> {
        let mut moves = Vec::new();
        if root < 100 { moves.push(1); }
        if root < 99 { moves.push(2); }  
        if root < 98 { moves.push(3); }
        if root < 96 { moves.push(5); }
        moves
    }
    
    fn eval(&self, state: i32) -> f32 {
        if state == 100 {
            -1.0
        } else {
            0.0
        }
    }

    fn apply(&self, state: i32, m: i32) -> i32 {
        state + m
    }
}

fn main() {
    let g = TestGame;
    let muv = Minimax::best_move(100, &g, 95);
    println!("helo thar: {}", muv);
}
