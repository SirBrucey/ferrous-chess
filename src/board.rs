use crate::piece::Piece;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Board {
    squares: [[Option<Piece>; 8]; 8],
    turn: Colour,
    white_castling: CastlingRights,
    black_castling: CastlingRights,
    move_list: Vec<Move>,
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

    #[cfg(test)]
    pub(crate) const fn empty() -> Self {
        Self {
            squares: [[None; 8]; 8],
            turn: Colour::White,
            white_castling: CastlingRights {
                kingside: false,
                queenside: false,
            },
            black_castling: CastlingRights {
                kingside: false,
                queenside: false,
            },
            move_list: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn set_square(&mut self, coord: Coordinate, piece: Option<Piece>) {
        self.squares[coord.y as usize][coord.x as usize] = piece;
    }

    fn import_from_fen(&mut self, fen: &str) -> Result<(), &'static str> {
        todo!()
    }

    fn get_legal_moves(&self) -> Vec<Move> {
        todo!()
    }

    fn is_board_legal(&self) -> bool {
        todo!()
    }

    fn is_in_check(&self, colour: Colour) -> bool {
        todo!()
    }

    // FIXME: Should this be infallible, we should be able to leverage get_legal_moves.
    fn make_move(&mut self, mv: Move) -> Result<(), &'static str> {
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
    x: u8,
    y: u8,
}

impl Coordinate {
    pub(crate) const fn new(x: u8, y: u8) -> Result<Self, &'static str> {
        if x < 8 && y < 8 {
            Ok(Self::new_unchecked(x, y))
        } else {
            Err("Coordinates out of bounds")
        }
    }

    pub(crate) const fn new_unchecked(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub(crate) fn try_apply_delta(&self, (dx, dy): (i8, i8)) -> Result<Coordinate, &'static str> {
        let new_x = self.x as i8 + dx;
        let new_y = self.y as i8 + dy;
        if (0..8).contains(&new_x) && (0..8).contains(&new_y) {
            Ok(Coordinate {
                x: new_x as u8,
                y: new_y as u8,
            })
        } else {
            Err("Resulting coordinates out of bounds")
        }
    }

    pub(crate) fn apply_deltas(&self, deltas: impl Iterator<Item = (i8, i8)>) -> Vec<Coordinate> {
        let mut results = Vec::new();
        for (dx, dy) in deltas {
            if let Ok(new_coord) = self.try_apply_delta((dx, dy)) {
                results.push(new_coord);
            }
        }
        results
    }
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
