use crate::{game::{Game, GameState}, chess_move::{ChessMove, self}, board::Board};


pub fn negation_max(game: &mut Game, depth_left: u8) -> (ChessMove, i32) {
    if depth_left == 0 {
        return (chess_move::NULL_MOVE, static_eval(game));
    }
    
    let mut best_value = i32::MIN;
    let mut best_move = chess_move::NULL_MOVE;

    for m in game.get_legal_moves() {
        game.make_move(m);

        let value = -negation_max(game,  depth_left - 1).1;
        
        if value > best_value {
            best_value = value;
            best_move = m;
        }
        
        game.undo_move();
    }    

    return (best_move, best_value);
}
//                              Pawn, Knight, Bishop, Rook, Queen
const PIECE_VALUES: [i32; 5] = [1000, 2800, 3200, 5000, 9000];
const CHECK_MATE_VALUE: i32 = 1_000_000_000;
pub fn static_eval(game: &mut Game) -> i32 {
    
    match game.get_game_state() {
        GameState::Checkmate => return CHECK_MATE_VALUE,
        GameState::Draw => return 0,
        GameState::Undecided => ()
    }
    
    //whites perspective
    let board = game.get_board();
    let mut sum: i32 = 0;

    for i in 0..5 {
        sum += (board.piece_lists[i * 2 + 0].count() as i32 - board.piece_lists[i * 2 + 1].count() as i32) 
            * PIECE_VALUES[i];
    }
    
    return  sum * if game.is_whites_turn() { 1 } else { -1 };
}

