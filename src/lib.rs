pub trait Game<State: Clone, Move: Clone, Value: PartialOrd> {
    fn get_moves(&self, &State) -> Vec<Move>;
    fn eval(&self, &State, my_turn: bool) -> Value;
    fn apply(&self, &State, Move) -> State;
}

pub struct Minimax;

impl Minimax {
    pub fn best_move
        <State: Clone, Move: Clone, Value: PartialOrd, GameType: Game<State, Move, Value>>
        (depth: i32, game: &GameType, root: &State) -> Move
    {
        let (mv, _score) = Minimax::min_max(depth, game, root, false);
        mv.expect("no moves")
    }

    fn min_max
        <State: Clone, Move: Clone, Value: PartialOrd, GameType: Game<State, Move, Value>>
        (depth: i32, game: &GameType, root: &State, min: bool) -> (Option<Move>, Value)
    {
        if depth == 0 {
            return (None, game.eval(root, min));
        }

        let moves = game.get_moves(root);
        if moves.len() == 0 {
            return (None, game.eval(root, min));
        }

        let best: (Option<Move>, Option<Value>) = moves.iter().fold((None, None), |acc, item| {
            let new_state = game.apply(root, item.clone());
            let (_sub_move, sub_value) = Minimax::min_max(depth - 1, game, &new_state, !min);
            if let (Some(acc_move), Some(acc_value)) = acc {
                if (!min && sub_value > acc_value) ||
                    (min && sub_value < acc_value)
                {
                    return (Some(item.clone()), Some(sub_value));
                } else {
                    return (Some(acc_move), Some(acc_value));
                }
            } else {
                return (Some(item.clone()), Some(sub_value));
            }
        });

        let (mv, val) = best;
        let val = val.expect("no moves?");
        (mv, val)
    }
}

#[test]
fn it_works() {
}
