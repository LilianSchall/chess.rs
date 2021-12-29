use crate::common::MoveData::{precomputed_move_data, DIRECTION_OFFSET};

use super::piece::{Piece, PieceType, PColor};
use super::board::Board;

use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub start: usize,
    pub end: usize
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum MoveAction {
    INCORRECT,
    MOVE,
    TAKE,
}

impl Move {
    pub fn new(start: usize, end: usize) -> Move {
        Move {
            start,
            end
        }
    }

    pub fn is_valid(start: usize, end: usize, board: &mut Board,
                    piece: Piece,
                    possible_moves: &HashMap<usize, Vec<Move>>) -> MoveAction {
        if !Move::is_in_list(start,end,&possible_moves[&start]) {
            return MoveAction::INCORRECT;
        }

        let selected = board.get_square(end);
        
        board.set(end / 8, end % 8,Some(piece));

        match selected {
            None => {
                MoveAction::MOVE
            },
            Some(_) => {
                MoveAction::TAKE
            }
        }
    }

    fn is_in_list(start: usize, end: usize, moves: &Vec<Move>) -> bool {
        for r#move in moves {
            println!("Move: from {} to {}", r#move.start, r#move.end);
            if r#move.start == start && r#move.end == end {
                return true;
            }
        }
        false
    }
}

pub struct MoveGenerator {
    precomputed: [[i8;8]; 64]
}

impl MoveGenerator {
    pub fn new () -> MoveGenerator {
        let precomputed = precomputed_move_data();

        MoveGenerator {
            precomputed,
        }
    }
    pub fn GenerateMoves(&self, board: &Board, player_color: PColor) 
        -> HashMap<usize,Vec<Move>> {
        let mut hash = HashMap::new();
        for square in 0 .. 64 {
            let mut moves: Vec<Move> = Vec::new();
            let piece = match board.get_square(square) {
                None => {continue}
                Some(p) => {p}
            };
                
            if !piece.is_color(player_color) {continue;}

            if piece.is_sliding_piece() {
                self.GenerateSlidingMove(&mut moves, &piece,
                                         square, board);
            }
            else {
               match piece.r#type {
                    PieceType::PAWN => {self.GeneratePawnMove(
                            &mut moves, &piece, square, board)},
                    PieceType::KING => {self.GenerateKingMove(
                            &mut moves, &piece, square, board)},
                    PieceType::KNIGHT => {self.GenerateKnightMove(
                            &mut moves, &piece, square, board)}
                    _ => {}
               }
            }
            hash.insert(square, moves);
        }    
        hash
    }
    
    fn GenerateSlidingMove(&self, moves: &mut Vec<Move>, 
                           piece: &Piece, square: usize, board: &Board) {

        let start_index: i32 = if piece.r#type == PieceType::BISHOP {4} else {0};
        let end_index: i32 = if piece.r#type == PieceType::ROOK {4} else {8};

        for index in start_index .. end_index {
            for n in 0 .. self.precomputed[square][index as usize] {

                let target = (square as i8
                    + (DIRECTION_OFFSET[index as usize] * (n+1))) as usize;
                let s = board.get_square(target as usize);
                if s != None {
                    if piece.is_ennemy(s) {
                        moves.push(Move::new(square,target as usize));
                    }
                    break;
                }
                else
                {
                    moves.push(Move::new(square,target as usize));
                }
                                   
            }
        }
    }
    fn GeneratePawnMove(&self, moves: &mut Vec<Move>, 
                           piece: &Piece, square: usize, board: &Board) {
        let nb_moves = match piece.color {
            PColor::WHITE => {
                if square / 8 == 6 { // if it is on the seventh row
                    2}
                else {1}
            }
            PColor::BLACK => {
                if square / 8 == 1 { //if it is on the second row
                    2}
                else {1}
            }
        };
        let (direction,range_column,diag_left,diag_right) =
            if piece.is_color(PColor::WHITE)
            {(DIRECTION_OFFSET[0], self.precomputed[square][0],
                4, 6)} 
            else {(DIRECTION_OFFSET[1], self.precomputed[square][1],
                7, 5)};

        if range_column == 0 {
            return;
        }

        for n in 0..nb_moves{
            let target = square as i8 + direction * (n + 1);
            if board.get_square(target as usize) != None {
                break;
            }
            moves.push(Move::new(square, target as usize));
        }
        for diag in [diag_left, diag_right] {
            if self.precomputed[square][diag] > 0 {
                let pos: usize = (square as i8 + 
                                  DIRECTION_OFFSET[diag]) as usize;

                if piece.is_ennemy(board.get_square(pos)){
                    moves.push(Move::new(square,pos))
                }
            }
        }
                    
    }
    
    fn GenerateKingMove(&self, moves: &mut Vec<Move>, 
                           piece: &Piece, square: usize, board: &Board) {
        
        for index in 0..8 {
            if self.precomputed[square][index] != 0 {
                let end: usize = (square as i8 + DIRECTION_OFFSET[index]) as usize;
                if !piece.is_ally(board.get_square(end)) {
                    moves.push(Move::new(square, end));
                }
            }
        }
    }

    fn GenerateKnightMove(&self, moves: &mut Vec<Move>, 
                           piece: &Piece, square: usize, board: &Board) {
    }

}
