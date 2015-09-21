extern crate minimax;

use minimax::{Game, Minimax};

struct TestGame;

impl Game<i32, i32, f32> for TestGame {
    fn get_moves(&self, root: i32) -> Vec<i32> {
        let res = vec![root + 1, root + 0];
        res
    }
    
    fn eval(&self, state: i32) -> f32 {
        return state as f32;
    }

    fn apply(&self, state: i32, m: i32) -> i32 {
        return (state + m) % 10;
    }
}

fn main() {
    let g = TestGame;
    let muv = Minimax::best_move(1, g, 1);
    println!("helo thar: {}", muv);
}
