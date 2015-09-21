trait Game<State: Copy, Move: Copy, Value: PartialOrd> {
    fn get_moves(&self, State) -> Vec<Move>;
    fn eval(&self, State) -> Value;
    fn apply(&self, State, Move) -> State;
}

struct Minimax;

impl Minimax {
    fn best_move
        <State: Copy, Move: Copy, Value: PartialOrd, GameType: Game<State, Move, Value>>
        (depth: i32, game: GameType, root: State) -> Move
    {
        let moves = game.get_moves(root);
        
        let (first, rest) = moves.split_at(1);
        let mut muu : Move = first[0];
        let mut bezt = game.eval(game.apply(root, muu));
        for &m in rest {
            let newState = game.apply(root, m);
            let vallie = game.eval(newState);
            if (vallie > bezt) {
                bezt = vallie;
                muu = m;
            }
        }

        muu
    }
}

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

#[test]
fn it_works() {
    let g = TestGame;
    assert_eq!(2, Minimax::best_move(1, g, 1));
}
