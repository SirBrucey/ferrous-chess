use crate::board::{Colour, Coordinate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Piece {
    piece_type: PieceType,
    colour: Colour,
}

impl Piece {
    pub(crate) const fn pawn(colour: Colour) -> Self {
        Self {
            piece_type: PieceType::Pawn,
            colour,
        }
    }

    pub(crate) const fn knight(colour: Colour) -> Self {
        Self {
            piece_type: PieceType::Knight,
            colour,
        }
    }

    pub(crate) const fn bishop(colour: Colour) -> Self {
        Self {
            piece_type: PieceType::Bishop,
            colour,
        }
    }

    pub(crate) const fn rook(colour: Colour) -> Self {
        Self {
            piece_type: PieceType::Rook,
            colour,
        }
    }

    pub(crate) const fn queen(colour: Colour) -> Self {
        Self {
            piece_type: PieceType::Queen,
            colour,
        }
    }

    pub(crate) const fn king(colour: Colour) -> Self {
        Self {
            piece_type: PieceType::King,
            colour,
        }
    }
}

trait PieceMovement {
    fn generate_pseudo_legal_moves(&self, position: Coordinate) -> Vec<Coordinate>;
}

impl PieceMovement for PieceType {
    fn generate_pseudo_legal_moves(&self, position: Coordinate) -> Vec<Coordinate> {
        match self {
            PieceType::Pawn => psuedo_pawn_moves(position),
            PieceType::Knight => psuedo_knight_moves(position),
            PieceType::Bishop => psuedo_bishop_moves(position),
            PieceType::Rook => psuedo_rook_moves(position),
            PieceType::Queen => psuedo_queen_moves(position),
            PieceType::King => psuedo_king_moves(position),
        }
    }
}

fn psuedo_pawn_moves(position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn psuedo_knight_moves(position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn psuedo_bishop_moves(position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn psuedo_rook_moves(position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn psuedo_queen_moves(position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn psuedo_king_moves(position: Coordinate) -> Vec<Coordinate> {
    todo!()
}
