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

    pub(crate) fn get_square(&self, coord: &Coordinate) -> Option<Piece> {
        self.squares[coord.y as usize][coord.x as usize]
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

    fn pseudo_knight_moves(
        &self,
        position: &Coordinate,
    ) -> Result<impl Iterator<Item = Coordinate>, &'static str> {
        match self.get_square(position) {
            Some(piece) if piece.colour != self.turn => {
                return Err("Not the turn of the piece at the given position");
            }
            Some(piece) if piece.piece_type != crate::piece::PieceType::Knight => {
                return Err("The piece at the given position is not a knight");
            }
            None => return Err("No piece at the given position"),
            _ => {}
        };
        let deltas = [
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ];
        Ok(position.apply_deltas(deltas.into_iter()).filter(|&coord| {
            !self
                .get_square(&coord)
                .is_some_and(|piece| piece.colour == self.turn)
        }))
    }

    fn pseudo_pawn_moves(&self, position: &Coordinate) -> Vec<Coordinate> {
        todo!()
    }

    fn pseudo_bishop_moves(&self, position: &Coordinate) -> Vec<Coordinate> {
        todo!()
    }

    fn pseudo_rook_moves(&self, position: &Coordinate) -> Vec<Coordinate> {
        todo!()
    }

    fn pseudo_queen_moves(&self, position: &Coordinate) -> Vec<Coordinate> {
        todo!()
    }

    fn pseudo_king_moves(&self, position: &Coordinate) -> Vec<Coordinate> {
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

    pub(crate) fn apply_deltas<T>(&self, deltas: T) -> impl Iterator<Item = Coordinate> + use<'_, T>
    where
        T: Iterator<Item = (i8, i8)>,
    {
        deltas.filter_map(|delta| self.try_apply_delta(delta).ok())
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn mk_board(piece: Piece, position: Coordinate) -> Board {
        let mut board = Board::empty();
        board.set_square(position, Some(piece));
        board
    }

    #[rstest]
    #[case::middle(
        Coordinate::new_unchecked(4, 4),
        vec![
            Coordinate::new_unchecked(5, 6),
            Coordinate::new_unchecked(5, 2),
            Coordinate::new_unchecked(3, 6),
            Coordinate::new_unchecked(3, 2),
            Coordinate::new_unchecked(6, 5),
            Coordinate::new_unchecked(6, 3),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(2, 3)
        ]
    )]
    #[case::bottom_left(
        Coordinate::new_unchecked(0, 0),
        vec![Coordinate::new_unchecked(1, 2), Coordinate::new_unchecked(2, 1)]
    )]
    #[case::bottom_edge(
        Coordinate::new_unchecked(4, 0),
        vec![
            Coordinate::new_unchecked(5, 2),
            Coordinate::new_unchecked(3, 2),
            Coordinate::new_unchecked(6, 1),
            Coordinate::new_unchecked(2, 1)
        ]
    )]
    #[case::bottom_right(
        Coordinate::new_unchecked(7, 0),
        vec![Coordinate::new_unchecked(6, 2), Coordinate::new_unchecked(5, 1)]
    )]
    #[case::left_middle(
        Coordinate::new_unchecked(0, 4),
        vec![
            Coordinate::new_unchecked(1, 6),
            Coordinate::new_unchecked(1, 2),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(2, 3)
        ]
    )]
    #[case::right_middle(
        Coordinate::new_unchecked(7, 4),
        vec![
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(6, 2),
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(5, 3)
        ]
    )]
    #[case::top_left(
        Coordinate::new_unchecked(0, 7),
        vec![Coordinate::new_unchecked(1, 5), Coordinate::new_unchecked(2, 6)]
    )]
    #[case::top_edge(
        Coordinate::new_unchecked(4, 7),
        vec![
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(3, 5),
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(2, 6)
        ]
    )]
    #[case::top_right(
        Coordinate::new_unchecked(7, 7),
        vec![Coordinate::new_unchecked(6, 5), Coordinate::new_unchecked(5, 6)]
    )]
    fn knight_moves_edge(#[case] start: Coordinate, #[case] expected: Vec<Coordinate>) {
        let board = mk_board(Piece::knight(Colour::White), start);
        let moves: Vec<Coordinate> = board.pseudo_knight_moves(&start).unwrap().collect();
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }
}
