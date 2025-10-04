use crate::board::Colour;

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
    pub(crate) piece_type: PieceType,
    pub(crate) colour: Colour,
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
