extern crate minimax;

use minimax::{Game, Minimax};
use std::vec::Vec;
use std::io;

struct TestGame;

impl Game<i32, i32, f32> for TestGame {
    fn get_moves(&self, root: i32) -> Vec<i32> {
        let mut moves = Vec::new();
        if root < 30 { moves.push(1); }
        if root < 29 { moves.push(2); }  
        if root < 28 { moves.push(3); }
        if root < 26 { moves.push(5); }
        moves
    }
    
    fn eval(&self, state: i32, my_turn: bool) -> f32 {
        if state == 30 {
            if my_turn {
                -1.0
            } else {
                1.0
            }
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
    let mut state = 0;
    println!("Rule: The player who adds the 30th pebble to the imaginary pile, loses.");
    loop {
        println!("{} pebbles.", state);
        let mut player_add = String::new();
        println!("Add 1, 2, 3 or 5 pebbles?");
        io::stdin().read_line(&mut player_add)
            .ok()
            .expect("Failed reading input");
        let player_add: i32 = player_add.trim().parse()
            .ok()
            .expect("Please type a number!");
        if !(player_add == 1 ||
             player_add == 2 ||
             player_add == 3 ||
             player_add == 5) {
            println!("1, 2, 3 or 5!");
            continue;
        }

        state += player_add;
        println!("{} pebbles.", state);
        if state >= 30 {
            println!("You passed 30 and lost!");
            break;
        }

        let muv = Minimax::best_move(10, &g, state);
        println!("I add {}.", muv);
        state += muv;
        if state >= 30 {
            println!("You won!");
            break;
        }
    }
}
