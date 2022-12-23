use std::{any::Any, fmt::{Display, Formatter}};

use colored::Colorize;

use crate::{game::Game, rook::Rook, piece::{Construct, Move, Piece, DynClone}, player::Player};

#[derive(Clone, Debug)]
pub struct King {
    player: Player,
    //set has_moved to false after moved
    pub has_moved: bool
}


impl Piece for King {
    fn get_legal_moves(&self, position: (u8,u8), game: &Game) -> Vec<(u8, u8)> {
        let mut moves = Vec::new();
        for (x,y) in [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)]{
            let new_pos = (position.0 as i8 + x, position.1 as i8 + y);
            if new_pos.0 < 0 || new_pos.0 > 7 || new_pos.1 < 0 || new_pos.1 > 7 {
                continue;
            }
            let new_pos = (new_pos.0 as u8, new_pos.1 as u8);
            if game.is_not_ally(new_pos) {
                // check for in_check here?
                moves.push(new_pos);
            }
        }    
        if self.has_moved || game.in_check(position) {
            return moves;
        }
        if self.can_castle_left(position, game) {
            moves.push((2,position.1));
        }
        if self.can_castle_right(position, game) {
            moves.push((6,position.1));
        }
        moves
    }

    //doesn't  handle friendly fire or moving into check
    fn valid_move(&self, from: (u8,u8), to: (u8,u8), game: &Game) -> Move {
        let (x, y) = (to.0 as i8 - from.0 as i8, to.1 as i8 - from.1 as i8);
        if x.abs() < 2 && y.abs() < 2 {
            Move::Normal
        } else if game.can_castle(from, to) {
            // use new function to check if castling is valid?
            Move::Castle
        } else {
            Move::Invalid
        }
    }

    fn player(&self) -> Player {
        self.player
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        "king"
    }
}

impl King {
    fn can_castle_left(&self, position: (u8,u8), game: &Game) -> bool {
        let y = match self.player {
            Player::One => 0,
            Player::Two => 7
        };
        if self.has_moved || position != (4,y) || game.in_check((4,y)) {
            return false;
        }
        if game.square_is_none((3,y)) && game.square_is_none((2,y)) && game.square_is_none((1,y)) {
            if let Some(rook) = game.get((0,y)) {
                // if matches!(rook, Rook) {
                // if rook.is_type::<Rook>() {
                //     let rook = rook.get_piece::<Rook>().unwrap();
                if let Some(rook) = rook.get_piece::<Rook>() {
                    if !rook.has_moved {
                        return true;
                    }
                }
            }
            // if rook.is_some() && matches!(rook.unwrap(), Rook) {
            //     let rook = rook.unwrap().as_any().downcast_ref::<Rook>().unwrap();
            //     if !rook.has_moved {
            //         return true;
            //     }
            // }
        }
        false
    }

    fn can_castle_right(&self , position: (u8,u8), game: &Game) -> bool {
        let y = match self.player {
            Player::One => 0,
            Player::Two => 7
        };
        if self.has_moved || position != (4,y) || game.in_check((4,y)) {
            return false;
        }
        if game.square_is_none((5,y)) && game.square_is_none((6,y)) {
            if let Some(rook) = game.get((7,y)) {
                // if matches!(rook, Rook) {
                // // if rook.is_type::<Rook>() {
                //     let rook = rook.get_piece::<Rook>().unwrap();
                if let Some(rook) = rook.get_piece::<Rook>() {
                    if !rook.has_moved {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl Construct for King {
    fn new(player: Player) -> Self {
        Self {
            player,
            has_moved: false
        }
    }
}

impl DynClone for King {
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Display for King {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self.player() {
            Player::One => "♚".white().bold(),
            Player::Two => "♔".black()
        };
        write!(f, "{}", c)
    }
}