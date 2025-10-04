use crate::board::{Board, Colour, Coordinate};

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
    fn generate_pseudo_legal_moves(
        &self,
        board: &Board,
        piece: &Piece,
        position: Coordinate,
    ) -> Vec<Coordinate>;
}

impl PieceMovement for PieceType {
    fn generate_pseudo_legal_moves(
        &self,
        board: &Board,
        piece: &Piece,
        position: Coordinate,
    ) -> Vec<Coordinate> {
        match self {
            PieceType::Pawn => pseudo_pawn_moves(board, piece, position),
            PieceType::Knight => pseudo_knight_moves(board, piece, position),
            PieceType::Bishop => pseudo_bishop_moves(board, piece, position),
            PieceType::Rook => pseudo_rook_moves(board, piece, position),
            PieceType::Queen => pseudo_queen_moves(board, piece, position),
            PieceType::King => pseudo_king_moves(board, piece, position),
        }
    }
}

fn pseudo_pawn_moves(board: &Board, piece: &Piece, position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn pseudo_knight_moves(board: &Board, piece: &Piece, position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn pseudo_bishop_moves(board: &Board, piece: &Piece, position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn pseudo_rook_moves(board: &Board, piece: &Piece, position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn pseudo_queen_moves(board: &Board, piece: &Piece, position: Coordinate) -> Vec<Coordinate> {
    todo!()
}

fn pseudo_king_moves(board: &Board, piece: &Piece, position: Coordinate) -> Vec<Coordinate> {
    todo!()
}
