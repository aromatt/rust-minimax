pub trait Game<State: Copy, Move: Copy, Value: PartialOrd> {
    fn get_moves(&self, State) -> Vec<Move>;
    fn eval(&self, State) -> Value;
    fn apply(&self, State, Move) -> State;
}

pub struct Minimax;

impl Minimax {
    pub fn best_move
        <State: Copy, Move: Copy, Value: PartialOrd, GameType: Game<State, Move, Value>>
        (depth: i32, game: &GameType, root: State) -> Move
    {
        let (mv, _score) = Minimax::min_max(depth, game, root, false);
        mv.expect("no moves")
    }

    fn min_max
        <State: Copy, Move: Copy, Value: PartialOrd, GameType: Game<State, Move, Value>>
        (depth: i32, game: &GameType, root: State, min: bool) -> (Option<Move>, Value)
    {
        if depth == 0 {
            return (None, game.eval(root));
        }

        let moves = game.get_moves(root);
        if moves.len() == 0 {
            return (None, game.eval(root));
        }
        
        let (first, rest) = moves.split_at(1);

        let mut muu: Move = first[0];
        let (_subMove, mut bezt) = Minimax::min_max(depth - 1, game, game.apply(root, muu), !min);
        for &m in rest {
            let new_state = game.apply(root, m);
            let (_subMove, vallie) = Minimax::min_max(depth - 1, game, new_state, !min);
            if (!min && vallie > bezt) || 
                (min && vallie < bezt)
            {
                bezt = vallie;
                muu = m;
            }
        }

        (Some(muu), bezt)
    }
}

#[test]
fn it_works() {
}
