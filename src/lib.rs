use std::cmp;

pub trait Game<State: Clone, Move: Copy> {
    fn get_moves(&self, &State) -> Vec<Move>;
    fn eval(&self, &State, my_turn: bool) -> i32;
    fn apply(&self, &State, Move) -> State;
    fn gameover(&self, &State) -> bool;
}

pub struct Minimax;
impl Minimax {
    pub fn best_move<State, Move, GameType>(depth: i32, game: &GameType, root: &State) -> Move
        where State: Clone,
              Move: Copy,
              GameType: Game<State, Move> {
        let (a, b) = (std::i32::MIN, std::i32::MAX);
        let (mv, _score) = Minimax::min_max(depth, game, root, false, a, b);
        mv.expect("no moves")
    }

    fn min_max<State, Move, GameType>(depth: i32, game: &GameType, root: &State, do_min: bool,
                                      mut a: i32, mut b: i32) -> (Option<Move>, i32)
        where State: Clone,
              Move: Copy,
              GameType: Game<State, Move> {

        // Don't need to do anything if depth is 0, someone has won, or there are no moves
        if depth == 0 {
            return (None, game.eval(root, do_min));
        }
        if game.gameover(root) {
            return (None, game.eval(root, do_min));
        }
        let moves = game.get_moves(root);
        if moves.len() == 0 {
            return (None, game.eval(root, do_min));
        }

        let mut best_mv: Option<Move> = None;
        let mut best_v: i32 = std::i32::MIN;
        for &mv in moves.iter() {
            let child = game.apply(root, mv);
            let (_child_mv, child_v) = Minimax::min_max(depth - 1, game, &child, !do_min, a, b);

            // If nothing else, just choose the first move
            if best_mv.is_none() {
                best_mv = Some(mv);
                best_v = child_v;
                continue;
            }

            // Otherwise, apply alpha-beta pruning
            if !do_min {
                if child_v > best_v {
                    best_v = child_v;
                    best_mv = Some(mv);
                }
                a = cmp::max(a, best_v);
                if b <= a { break; }
            } else {
                if child_v < best_v {
                    best_v = child_v;
                    best_mv = Some(mv);
                }
                b = cmp::min(b, best_v);
                if b <= a { break; }
            }
        };

        best_mv.expect("no moves?");
        (best_mv, best_v)
    }
}

#[test]
fn it_works() {
}
