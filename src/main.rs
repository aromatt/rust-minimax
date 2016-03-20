extern crate minimax;

use minimax::{Game, Minimax};
use std::vec::Vec;
use std::io;

struct TestGame;

impl Game<i32, i32> for TestGame {
    fn get_moves(&self, root: &i32) -> Vec<i32> {
        let mut moves = Vec::new();
        if *root < 10 { moves.push(1); }
        if *root < 9 { moves.push(2); }
        if *root < 8 { moves.push(3); }
        if *root < 6 { moves.push(5); }
        moves
    }

    fn eval(&self, state: &i32, my_turn: bool) -> i32 {
        if *state == 10 {
            if my_turn {
                -1
            } else {
                1
            }
        } else {
            0
        }
    }

    fn apply(&self, state: &i32, m: i32) -> i32 {
        *state + m
    }
}

fn main() {
    let g = TestGame;
    let mut state = 0;
    println!("Rule: The player who adds the 10th pebble to the imaginary pile, loses.");
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
        if state >= 10 {
            println!("You passed 10 and lost!");
            break;
        }

        let mv = Minimax::best_move(5, &g, &state);
        println!("I add {}.", mv);
        state += mv;
        if state >= 10 {
            println!("You won!");
            break;
        }
    }
}
