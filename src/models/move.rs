use crate::common::MoveData::{precomputed_move_data, DIRECTION_OFFSET};
use crate::common::MoveData::{
    NORTH,SOUTH,WEST,EAST,
    NORTH_WEST,NORTH_EAST,
    SOUTH_WEST,SOUTH_EAST
};
use crate::common::Misc;
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
    CASTLE,
}

impl Move {
    pub fn new(start: usize, end: usize) -> Move {
        Move {
            start,
            end
        }
    }

    pub fn is_valid(start: usize, end: usize, board: &mut Board,
                    piece: &mut Piece,
                    possible_moves: &HashMap<usize, Vec<Move>>) -> MoveAction {
        if !Move::is_in_list(start,end,&possible_moves[&start]) {
            return MoveAction::INCORRECT;
        }

        let selected = board.get_square(end);
        let (y,x): (usize, usize) = (start / 8, start % 8);
        let (i,j): (usize, usize) = (end / 8, end % 8);

        let delta_x : i8 = x as i8 - j as i8;
        let delta_y : i8 = y as i8 - i as i8;
        if piece.can_castle  && Misc::abs(delta_x as isize) == 2 {
            piece.can_castle = false;
            match piece.r#type {
                PieceType::KING => {
                    match delta_x {
                    2 => { //we castled left side
                        board.set(y, j + 1, 
                                board.get(y,0));

                        board.set(y,0,None);
                    }
                    -2 => { //we castled right side
                        board.set(y, j - 1, 
                                board.get(y,7));

                        board.set(y,7,None);
                    }
                    a => {println!("Wtf, help me I cannot make a simple addition.
                                   I got a delta of {} when castling in x",a);}
                }
                    board.set(i, j,Some(*piece));
                    return MoveAction::CASTLE;
                }
                _ => {}
            }
        }
        else if (piece.can_castle) {
            piece.can_castle = false;
        }
        else if piece.is_type(PieceType::PAWN) {
            if Misc::abs(delta_y as isize) == 2 {

                let mut tmp: Vec<(Option<Piece>, i8)> = Vec::new();
                tmp.push((board.get(i, j + 1), 1));
                if j != 0 {
                    tmp.push((board.get(i, j - 1), -1));
                }

                for (adj,abs) in tmp {
                    match adj {
                        None => {}
                        Some(mut p) => {
                            if p.is_type(PieceType::PAWN) && piece.is_ennemy(adj) {
                                p.can_en_passant = match abs {
                                    -1 => {EAST},
                                    1 => {WEST},
                                    _ => {0}
                                };
                                //we update the pawn states
                                board.set(i, (j as i8 + abs) as usize, Some(p));
                            }
                        }
                    }
                }
            }
            else if Misc::abs(delta_y as isize) == 1 &&
                Misc::abs(delta_x as isize) ==1 && selected == None {
                
                board.set(y,j,None);
                board.set(i, j, Some(*piece)); 
                return MoveAction::TAKE;
            }
        }
        board.set(i, j,Some(*piece));


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
    pub fn GenerateMoves(&self, board: &mut Board, player_color: PColor) 
        -> HashMap<usize,Vec<Move>> {
        let mut hash = HashMap::new();
        for square in 0 .. 64 {
            let mut moves: Vec<Move> = Vec::new();
            let mut piece = match board.get_square(square) {
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
                    PieceType::PAWN => {
                        if self.GeneratePawnMove(
                            &mut moves, &mut piece, square, board) {

                            board.set_square(square, Some(piece));
                        }
                    },
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
                           piece: &mut Piece, 
                           square: usize, board: &Board) -> bool {
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
            {(DIRECTION_OFFSET[NORTH], self.precomputed[square][NORTH],
                4, 6)} 
            else {(DIRECTION_OFFSET[SOUTH], self.precomputed[square][SOUTH],
                7, 5)};

        if range_column == 0 {
            return false;
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
        return self.__generate_en_passant_move(moves, piece, direction,
                                               square, board);      
    }

    fn __generate_en_passant_move(&self, moves: &mut Vec<Move>, 
                           piece: &mut Piece, direction: i8,
                           square: usize, board: &Board) -> bool {
    
        if piece.can_en_passant == 0 {return false;}
        
        match piece.can_en_passant {
            WEST => {
                moves.push(Move::new(
                        square,
                        (square as i8 + direction - 1) as usize
                        ));
            }
            EAST => {
                moves.push(Move::new(
                        square,
                        (square as i8 + direction + 1) as usize
                        ));
            }
            _ => {}
        }
        piece.can_en_passant = 0;
        return true;
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
        self.__generate_castling_move(moves, piece,
                                      square, board);
    }

    fn __generate_castling_move(&self, moves: &mut Vec<Move>, 
                           piece: &Piece, square: usize, board: &Board) {

        if !piece.can_castle {
            return;
        }
        for direction in [WEST, EAST] {
            let mut can_castle: bool = true;
            let sign: i8 = match direction {
                    WEST => {-1}
                    EAST => {1}
                    _ => {0}
                };
            for index in 0 .. (self.precomputed[square][direction] -1) {
                let target: i8 = square as i8 + sign * (index as i8 + 1);
                if board.get_square(target as usize) != None {
                    can_castle = false;
                    break;
                }
            }
            let y: usize = square / 8;
            let pair = board.get(
                y, 
                if direction == WEST {0} else {7});
            if can_castle && Piece::can_castle(pair){
                let end: usize = (square as i8 + (sign * 2)) as usize;
                moves.push(Move::new(square,end));
            }
        }
                


    }

    fn GenerateKnightMove(&self, moves: &mut Vec<Move>, 
                           piece: &Piece, square: usize, board: &Board) {
        for row in [-1, 1] {
            for column in [-8, 8] {
                let target1: i32 = (square as i32) + row + (column * 2);
                let target2: i32 = (square as i32) + (row * 2) + column;
                self.__generate_knight_move(
                    moves,piece, square,target1,
                    board);
                self.__generate_knight_move(
                    moves, piece, square, target2,
                    board);
            }
        }
    }
    fn __generate_knight_move(&self, moves: &mut Vec<Move>, piece: &Piece,
                              square: usize, target: i32, board: &Board) {

        if target >= 0 && target < (board.size * board.size) as i32 {

            let t: usize = target as usize;
            let col_start: usize = square % 8;
            let col_end: usize = t % 8;

            if (col_start < 2 && col_end > 5) ||
                (col_start > 5 && col_end < 2) {return;}

            if !piece.is_ally(board.get_square(t)) {
                    moves.push(Move::new(square, t));
                }

        }
    }

}
