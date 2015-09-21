pub trait Game<State: Copy, Move: Copy, Value: PartialOrd> {
    fn get_moves(&self, State) -> Vec<Move>;
    fn eval(&self, State) -> Value;
    fn apply(&self, State, Move) -> State;
}

pub struct Minimax;

impl Minimax {
    pub fn best_move
        <State: Copy, Move: Copy, Value: PartialOrd, GameType: Game<State, Move, Value>>
        (depth: i32, game: GameType, root: State) -> Move
    {
        let moves = game.get_moves(root);
        
        let (first, rest) = moves.split_at(1);
        let mut muu : Move = first[0];
        let mut bezt = game.eval(game.apply(root, muu));
        for &m in rest {
            let new_state = game.apply(root, m);
            let vallie = game.eval(new_state);
            if vallie > bezt {
                bezt = vallie;
                muu = m;
            }
        }

        muu
    }
}

#[test]
fn it_works() {
}
