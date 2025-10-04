use crate::piece::Piece;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Board {
    squares: [[Option<Piece>; 8]; 8],
    turn: Colour,
    white_castling: CastlingRights,
    black_castling: CastlingRights,
    move_list: Vec<(usize, Move)>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

const DEFAULT_BOARD: [[Option<Piece>; 8]; 8] = [
    [
        Some(Piece::rook(Colour::White)),
        Some(Piece::knight(Colour::White)),
        Some(Piece::bishop(Colour::White)),
        Some(Piece::queen(Colour::White)),
        Some(Piece::king(Colour::White)),
        Some(Piece::bishop(Colour::White)),
        Some(Piece::knight(Colour::White)),
        Some(Piece::rook(Colour::White)),
    ],
    [Some(Piece::pawn(Colour::White)); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [Some(Piece::pawn(Colour::Black)); 8],
    [
        Some(Piece::rook(Colour::Black)),
        Some(Piece::knight(Colour::Black)),
        Some(Piece::bishop(Colour::Black)),
        Some(Piece::queen(Colour::Black)),
        Some(Piece::king(Colour::Black)),
        Some(Piece::bishop(Colour::Black)),
        Some(Piece::knight(Colour::Black)),
        Some(Piece::rook(Colour::Black)),
    ],
];

impl Board {
    const fn new() -> Self {
        Self {
            squares: DEFAULT_BOARD,
            turn: Colour::White,
            white_castling: CastlingRights {
                kingside: true,
                queenside: true,
            },
            black_castling: CastlingRights {
                kingside: true,
                queenside: true,
            },
            move_list: Vec::new(),
        }
    }

    fn import_from_fen(&mut self, fen: &str) -> Result<(), &'static str> {
        todo!()
    }

    fn get_legal_moves(&self) -> Vec<(Piece, Coordinate)> {
        todo!()
    }

    fn is_board_legal(&self) -> bool {
        todo!()
    }

    fn is_in_check(&self, colour: Colour) -> bool {
        todo!()
    }

    // FIXME: Should this be infallible, we should be able to leverage get_legal_moves.
    fn make_move(&mut self, from: Coordinate, to: Coordinate) -> Result<(), &'static str> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Colour {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Move {
    piece: Piece,
    from: Coordinate,
    to: Coordinate,
    captured: Option<Piece>,
    promotion: Option<Piece>,
    is_castling: bool,
    is_en_passant: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Coordinate {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CastlingRights {
    kingside: bool,
    queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            kingside: true,
            queenside: true,
        }
    }
}

const fn is_within_bounds(coord: Coordinate) -> bool {
    coord.x < 8 && coord.y < 8
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Coordinate { x: 0, y: 0 }, true)]
    #[case(Coordinate { x: 7, y: 7 }, true)]
    #[case(Coordinate { x: 8, y: 0 }, false)]
    #[case(Coordinate { x: 0, y: 8 }, false)]
    fn coord_within_bounds(#[case] coord: Coordinate, #[case] expected: bool) {
        assert_eq!(is_within_bounds(coord), expected);
    }
}
