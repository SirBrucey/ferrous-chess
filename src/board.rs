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

    fn validate_piece_for_move(
        &self,
        position: &Coordinate,
        expected_piece_type: crate::piece::PieceType,
    ) -> Result<Piece, String> {
        match self.get_square(position) {
            Some(piece) if piece.colour != self.turn => {
                Err("Not the turn of the piece at the given position".to_string())
            }
            Some(piece) if piece.piece_type != expected_piece_type => Err(format!(
                "The piece at the given position is not a {}",
                expected_piece_type
            )),
            Some(piece) => Ok(piece),
            None => Err("No piece at the given position".to_string()),
        }
    }

    fn pseudo_knight_moves(
        &self,
        position: &Coordinate,
    ) -> Result<impl Iterator<Item = Coordinate>, String> {
        self.validate_piece_for_move(position, crate::piece::PieceType::Knight)?;
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

    fn pseudo_rook_moves(
        &self,
        position: &Coordinate,
    ) -> Result<impl Iterator<Item = Coordinate>, String> {
        self.validate_piece_for_move(position, crate::piece::PieceType::Rook)?;
        let vectors = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        Ok(vectors.into_iter().flat_map(move |dir| RayIterator {
            board: self,
            current: *position,
            direction: dir,
            stopped: false,
        }))
    }

    fn pseudo_bishop_moves(
        &self,
        position: &Coordinate,
    ) -> Result<impl Iterator<Item = Coordinate>, String> {
        self.validate_piece_for_move(position, crate::piece::PieceType::Bishop)?;
        let vectors = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        Ok(vectors.into_iter().flat_map(move |dir| RayIterator {
            board: self,
            current: *position,
            direction: dir,
            stopped: false,
        }))
    }

    fn pseudo_queen_moves(
        &self,
        position: &Coordinate,
    ) -> Result<impl Iterator<Item = Coordinate>, String> {
        self.validate_piece_for_move(position, crate::piece::PieceType::Queen)?;
        let vectors = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        Ok(vectors.into_iter().flat_map(move |dir| RayIterator {
            board: self,
            current: *position,
            direction: dir,
            stopped: false,
        }))
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

struct RayIterator<'a> {
    board: &'a Board,
    current: Coordinate,
    direction: (i8, i8),
    stopped: bool,
}

impl<'a> Iterator for RayIterator<'a> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }

        match self.current.try_apply_delta(self.direction) {
            Ok(coord) => {
                self.current = coord;
                match self.board.get_square(&coord) {
                    // Current square occupied by a piece of the same colour, stop the ray.
                    Some(piece) if piece.colour == self.board.turn => {
                        self.stopped = true;
                        None
                    }
                    // Current square occupied by an opponent's piece, return the coordinate and
                    // stop the ray.
                    Some(_) => {
                        self.stopped = true;
                        Some(coord)
                    }
                    // Current square is empty, return the coordinate and continue the ray.
                    None => Some(coord),
                }
            }
            Err(_) => {
                self.stopped = true;
                None
            }
        }
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
    fn knight_moves(#[case] start: Coordinate, #[case] expected: Vec<Coordinate>) {
        let board = mk_board(Piece::knight(Colour::White), start);
        let moves: Vec<Coordinate> = board.pseudo_knight_moves(&start).unwrap().collect();
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }

    #[rstest]
    #[case::middle(
        Coordinate::new_unchecked(4, 4),
        vec![
            // Right
            Coordinate::new_unchecked(5, 4),
            Coordinate::new_unchecked(6, 4),
            Coordinate::new_unchecked(7, 4),
            // Left
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(0, 4),
            // Up
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(4, 6),
            Coordinate::new_unchecked(4, 7),
            // Down
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(4, 0),
        ]
    )]
    #[case::bottom_left(
        Coordinate::new_unchecked(0, 0),
        vec![
            // Right
            Coordinate::new_unchecked(1, 0),
            Coordinate::new_unchecked(2, 0),
            Coordinate::new_unchecked(3, 0),
            Coordinate::new_unchecked(4, 0),
            Coordinate::new_unchecked(5, 0),
            Coordinate::new_unchecked(6, 0),
            Coordinate::new_unchecked(7, 0),
            // Up
            Coordinate::new_unchecked(0, 1),
            Coordinate::new_unchecked(0, 2),
            Coordinate::new_unchecked(0, 3),
            Coordinate::new_unchecked(0, 4),
            Coordinate::new_unchecked(0, 5),
            Coordinate::new_unchecked(0, 6),
            Coordinate::new_unchecked(0, 7),
        ]
    )]
    #[case::bottom_edge(
        Coordinate::new_unchecked(4, 0),
        vec![
            // Right
            Coordinate::new_unchecked(5, 0),
            Coordinate::new_unchecked(6, 0),
            Coordinate::new_unchecked(7, 0),
            // Left
            Coordinate::new_unchecked(3, 0),
            Coordinate::new_unchecked(2, 0),
            Coordinate::new_unchecked(1, 0),
            Coordinate::new_unchecked(0, 0),
            // Up
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(4, 6),
            Coordinate::new_unchecked(4, 7),
        ]
    )]
    #[case::bottom_right(
        Coordinate::new_unchecked(7, 0),
        vec![
            // Left
            Coordinate::new_unchecked(6, 0),
            Coordinate::new_unchecked(5, 0),
            Coordinate::new_unchecked(4, 0),
            Coordinate::new_unchecked(3, 0),
            Coordinate::new_unchecked(2, 0),
            Coordinate::new_unchecked(1, 0),
            Coordinate::new_unchecked(0, 0),
            // Up
            Coordinate::new_unchecked(7, 1),
            Coordinate::new_unchecked(7, 2),
            Coordinate::new_unchecked(7, 3),
            Coordinate::new_unchecked(7, 4),
            Coordinate::new_unchecked(7, 5),
            Coordinate::new_unchecked(7, 6),
            Coordinate::new_unchecked(7, 7),
        ]
    )]
    #[case::left_middle(
        Coordinate::new_unchecked(0, 4),
        vec![
            // Right
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(5, 4),
            Coordinate::new_unchecked(6, 4),
            Coordinate::new_unchecked(7, 4),
            // Up
            Coordinate::new_unchecked(0, 5),
            Coordinate::new_unchecked(0, 6),
            Coordinate::new_unchecked(0, 7),
            // Down
            Coordinate::new_unchecked(0, 3),
            Coordinate::new_unchecked(0, 2),
            Coordinate::new_unchecked(0, 1),
            Coordinate::new_unchecked(0, 0),
        ]
    )]
    #[case::right_middle(
        Coordinate::new_unchecked(7, 4),
        vec![
            // Left
            Coordinate::new_unchecked(6, 4),
            Coordinate::new_unchecked(5, 4),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(0, 4),
            // Up
            Coordinate::new_unchecked(7, 5),
            Coordinate::new_unchecked(7, 6),
            Coordinate::new_unchecked(7, 7),
            // Down
            Coordinate::new_unchecked(7, 3),
            Coordinate::new_unchecked(7, 2),
            Coordinate::new_unchecked(7, 1),
            Coordinate::new_unchecked(7, 0),
        ]
    )]
    #[case::top_left(
        Coordinate::new_unchecked(0, 7),
        vec![
            // Right
            Coordinate::new_unchecked(1, 7),
            Coordinate::new_unchecked(2, 7),
            Coordinate::new_unchecked(3, 7),
            Coordinate::new_unchecked(4, 7),
            Coordinate::new_unchecked(5, 7),
            Coordinate::new_unchecked(6, 7),
            Coordinate::new_unchecked(7, 7),
            // Down
            Coordinate::new_unchecked(0, 6),
            Coordinate::new_unchecked(0, 5),
            Coordinate::new_unchecked(0, 4),
            Coordinate::new_unchecked(0, 3),
            Coordinate::new_unchecked(0, 2),
            Coordinate::new_unchecked(0, 1),
            Coordinate::new_unchecked(0, 0),
        ]
    )]
    #[case::top_edge(
        Coordinate::new_unchecked(4, 7),
        vec![
            // Right
            Coordinate::new_unchecked(5, 7),
            Coordinate::new_unchecked(6, 7),
            Coordinate::new_unchecked(7, 7),
            // Left
            Coordinate::new_unchecked(3, 7),
            Coordinate::new_unchecked(2, 7),
            Coordinate::new_unchecked(1, 7),
            Coordinate::new_unchecked(0, 7),
            // Down
            Coordinate::new_unchecked(4, 6),
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(4, 0),
        ]
    )]
    #[case::top_right(
        Coordinate::new_unchecked(7, 7),
        vec![
            // Left
            Coordinate::new_unchecked(6, 7),
            Coordinate::new_unchecked(5, 7),
            Coordinate::new_unchecked(4, 7),
            Coordinate::new_unchecked(3, 7),
            Coordinate::new_unchecked(2, 7),
            Coordinate::new_unchecked(1, 7),
            Coordinate::new_unchecked(0, 7),
            // Down
            Coordinate::new_unchecked(7, 6),
            Coordinate::new_unchecked(7, 5),
            Coordinate::new_unchecked(7, 4),
            Coordinate::new_unchecked(7, 3),
            Coordinate::new_unchecked(7, 2),
            Coordinate::new_unchecked(7, 1),
            Coordinate::new_unchecked(7, 0),
        ]
    )]
    fn rook_moves(#[case] start: Coordinate, #[case] expected: Vec<Coordinate>) {
        let board = mk_board(Piece::rook(Colour::White), start);
        let moves: Vec<Coordinate> = board.pseudo_rook_moves(&start).unwrap().collect();
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }

    #[rstest]
    #[case::middle(
        Coordinate::new_unchecked(4, 4),
        vec![
            // Top-right
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(7, 7),
            // Top-left
            Coordinate::new_unchecked(3, 5),
            Coordinate::new_unchecked(2, 6),
            Coordinate::new_unchecked(1, 7),
            // Bottom-right
            Coordinate::new_unchecked(5, 3),
            Coordinate::new_unchecked(6, 2),
            Coordinate::new_unchecked(7, 1),
            // Bottom-left
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(0, 0),
        ]
    )]
    #[case::bottom_left(
        Coordinate::new_unchecked(0, 0),
        vec![
            // Top-right
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(7, 7),
        ]
    )]
    #[case::bottom_edge(
        Coordinate::new_unchecked(4, 0),
        vec![
            // Top-right
            Coordinate::new_unchecked(5, 1),
            Coordinate::new_unchecked(6, 2),
            Coordinate::new_unchecked(7, 3),
            // Top-left
            Coordinate::new_unchecked(3, 1),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 3),
            Coordinate::new_unchecked(0, 4),
        ]
    )]
    #[case::bottom_right(
        Coordinate::new_unchecked(7, 0),
        vec![
            // Top-left
            Coordinate::new_unchecked(6, 1),
            Coordinate::new_unchecked(5, 2),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(1, 6),
            Coordinate::new_unchecked(0, 7),
        ]
    )]
    #[case::left_middle(
        Coordinate::new_unchecked(0, 4),
        vec![
            // Top-right
            Coordinate::new_unchecked(1, 5),
            Coordinate::new_unchecked(2, 6),
            Coordinate::new_unchecked(3, 7),
            // Bottom-right
            Coordinate::new_unchecked(1, 3),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(3, 1),
            Coordinate::new_unchecked(4, 0),
        ]
    )]
    #[case::right_middle(
        Coordinate::new_unchecked(7, 4),
        vec![
            // Top-left
            Coordinate::new_unchecked(6, 5),
            Coordinate::new_unchecked(5, 6),
            Coordinate::new_unchecked(4, 7),
            // Bottom-left
            Coordinate::new_unchecked(6, 3),
            Coordinate::new_unchecked(5, 2),
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(3, 0),
        ]
    )]
    #[case::top_left(
        Coordinate::new_unchecked(0, 7),
        vec![
            // Bottom-right
            Coordinate::new_unchecked(1, 6),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(5, 2),
            Coordinate::new_unchecked(6, 1),
            Coordinate::new_unchecked(7, 0),
        ]
    )]
    #[case::top_edge(
        Coordinate::new_unchecked(4, 7),
        vec![
            // Top-right
            Coordinate::new_unchecked(5, 6),
            Coordinate::new_unchecked(6, 5),
            Coordinate::new_unchecked(7, 4),
            // Top-left
            Coordinate::new_unchecked(3, 6),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(0, 3),
        ]
    )]
    #[case::top_right(
        Coordinate::new_unchecked(7, 7),
        vec![
            // Top-left
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(0, 0),
        ]
    )]
    fn bishop_moves(#[case] start: Coordinate, #[case] expected: Vec<Coordinate>) {
        let board = mk_board(Piece::bishop(Colour::White), start);
        let moves: Vec<Coordinate> = board.pseudo_bishop_moves(&start).unwrap().collect();
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }

    #[rstest]
    #[case::middle(
        Coordinate::new_unchecked(4, 4),
        vec![
            // Right
            Coordinate::new_unchecked(5, 4),
            Coordinate::new_unchecked(6, 4),
            Coordinate::new_unchecked(7, 4),
            // Left
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(0, 4),
            // Up
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(4, 6),
            Coordinate::new_unchecked(4, 7),
            // Down
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(4, 0),
            // Top-right
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(7, 7),
            // Top-left
            Coordinate::new_unchecked(3, 5),
            Coordinate::new_unchecked(2, 6),
            Coordinate::new_unchecked(1, 7),
            // Bottom-right
            Coordinate::new_unchecked(5, 3),
            Coordinate::new_unchecked(6, 2),
            Coordinate::new_unchecked(7, 1),
            // Bottom-left
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(0, 0),
        ]
    )]
    #[case::bottom_left(
        Coordinate::new_unchecked(0, 0),
        vec![
            // Right
            Coordinate::new_unchecked(1, 0),
            Coordinate::new_unchecked(2, 0),
            Coordinate::new_unchecked(3, 0),
            Coordinate::new_unchecked(4, 0),
            Coordinate::new_unchecked(5, 0),
            Coordinate::new_unchecked(6, 0),
            Coordinate::new_unchecked(7, 0),
            // Up
            Coordinate::new_unchecked(0, 1),
            Coordinate::new_unchecked(0, 2),
            Coordinate::new_unchecked(0, 3),
            Coordinate::new_unchecked(0, 4),
            Coordinate::new_unchecked(0, 5),
            Coordinate::new_unchecked(0, 6),
            Coordinate::new_unchecked(0, 7),
            // Top-right
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(7, 7),
        ]
    )]
    #[case::bottom_edge(
        Coordinate::new_unchecked(4, 0),
        vec![
            // Right
            Coordinate::new_unchecked(5, 0),
            Coordinate::new_unchecked(6, 0),
            Coordinate::new_unchecked(7, 0),
            // Left
            Coordinate::new_unchecked(3, 0),
            Coordinate::new_unchecked(2, 0),
            Coordinate::new_unchecked(1, 0),
            Coordinate::new_unchecked(0, 0),
            // Up
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(4, 6),
            Coordinate::new_unchecked(4, 7),
            // Top-right
            Coordinate::new_unchecked(5, 1),
            Coordinate::new_unchecked(6, 2),
            Coordinate::new_unchecked(7, 3),
            // Top-left
            Coordinate::new_unchecked(3, 1),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 3),
            Coordinate::new_unchecked(0, 4),
        ]
    )]
    #[case::bottom_right(
        Coordinate::new_unchecked(7, 0),
        vec![
            // Left
            Coordinate::new_unchecked(6, 0),
            Coordinate::new_unchecked(5, 0),
            Coordinate::new_unchecked(4, 0),
            Coordinate::new_unchecked(3, 0),
            Coordinate::new_unchecked(2, 0),
            Coordinate::new_unchecked(1, 0),
            Coordinate::new_unchecked(0, 0),
            // Up
            Coordinate::new_unchecked(7, 1),
            Coordinate::new_unchecked(7, 2),
            Coordinate::new_unchecked(7, 3),
            Coordinate::new_unchecked(7, 4),
            Coordinate::new_unchecked(7, 5),
            Coordinate::new_unchecked(7, 6),
            Coordinate::new_unchecked(7, 7),
            // Top-left
            Coordinate::new_unchecked(6, 1),
            Coordinate::new_unchecked(5, 2),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(1, 6),
            Coordinate::new_unchecked(0, 7),
        ]
    )]
    #[case::left_middle(
        Coordinate::new_unchecked(0, 4),
        vec![
            // Right
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(5, 4),
            Coordinate::new_unchecked(6, 4),
            Coordinate::new_unchecked(7, 4),
            // Up
            Coordinate::new_unchecked(0, 5),
            Coordinate::new_unchecked(0, 6),
            Coordinate::new_unchecked(0, 7),
            // Down
            Coordinate::new_unchecked(0, 3),
            Coordinate::new_unchecked(0, 2),
            Coordinate::new_unchecked(0, 1),
            Coordinate::new_unchecked(0, 0),
            // Top-right
            Coordinate::new_unchecked(1, 5),
            Coordinate::new_unchecked(2, 6),
            Coordinate::new_unchecked(3, 7),
            // Bottom-right
            Coordinate::new_unchecked(1, 3),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(3, 1),
            Coordinate::new_unchecked(4, 0),
        ]
    )]
    #[case::right_middle(
        Coordinate::new_unchecked(7, 4),
        vec![
            // Left
            Coordinate::new_unchecked(6, 4),
            Coordinate::new_unchecked(5, 4),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(0, 4),
            // Up
            Coordinate::new_unchecked(7, 5),
            Coordinate::new_unchecked(7, 6),
            Coordinate::new_unchecked(7, 7),
            // Down
            Coordinate::new_unchecked(7, 3),
            Coordinate::new_unchecked(7, 2),
            Coordinate::new_unchecked(7, 1),
            Coordinate::new_unchecked(7, 0),
            // Top-left
            Coordinate::new_unchecked(6, 5),
            Coordinate::new_unchecked(5, 6),
            Coordinate::new_unchecked(4, 7),
            // Bottom-left
            Coordinate::new_unchecked(6, 3),
            Coordinate::new_unchecked(5, 2),
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(3, 0),
        ]
    )]
    #[case::top_left(
        Coordinate::new_unchecked(0, 7),
        vec![
            // Right
            Coordinate::new_unchecked(1, 7),
            Coordinate::new_unchecked(2, 7),
            Coordinate::new_unchecked(3, 7),
            Coordinate::new_unchecked(4, 7),
            Coordinate::new_unchecked(5, 7),
            Coordinate::new_unchecked(6, 7),
            Coordinate::new_unchecked(7, 7),
            // Down
            Coordinate::new_unchecked(0, 6),
            Coordinate::new_unchecked(0, 5),
            Coordinate::new_unchecked(0, 4),
            Coordinate::new_unchecked(0, 3),
            Coordinate::new_unchecked(0, 2),
            Coordinate::new_unchecked(0, 1),
            Coordinate::new_unchecked(0, 0),
            // Bottom-right
            Coordinate::new_unchecked(1, 6),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(5, 2),
            Coordinate::new_unchecked(6, 1),
            Coordinate::new_unchecked(7, 0),
        ]
    )]
    #[case::top_edge(
        Coordinate::new_unchecked(4, 7),
        vec![
            // Right
            Coordinate::new_unchecked(5, 7),
            Coordinate::new_unchecked(6, 7),
            Coordinate::new_unchecked(7, 7),
            // Left
            Coordinate::new_unchecked(3, 7),
            Coordinate::new_unchecked(2, 7),
            Coordinate::new_unchecked(1, 7),
            Coordinate::new_unchecked(0, 7),
            // Down
            Coordinate::new_unchecked(4, 6),
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(4, 0),
            // Top-right
            Coordinate::new_unchecked(5, 6),
            Coordinate::new_unchecked(6, 5),
            Coordinate::new_unchecked(7, 4),
            // Top-left
            Coordinate::new_unchecked(3, 6),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(0, 3),
        ]
    )]
    #[case::top_right(
        Coordinate::new_unchecked(7, 7),
        vec![
            // Left
            Coordinate::new_unchecked(6, 7),
            Coordinate::new_unchecked(5, 7),
            Coordinate::new_unchecked(4, 7),
            Coordinate::new_unchecked(3, 7),
            Coordinate::new_unchecked(2, 7),
            Coordinate::new_unchecked(1, 7),
            Coordinate::new_unchecked(0, 7),
            // Down
            Coordinate::new_unchecked(7, 6),
            Coordinate::new_unchecked(7, 5),
            Coordinate::new_unchecked(7, 4),
            Coordinate::new_unchecked(7, 3),
            Coordinate::new_unchecked(7, 2),
            Coordinate::new_unchecked(7, 1),
            Coordinate::new_unchecked(7, 0),
            // Top-left
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(0, 0),
        ]
    )]
    fn queen_moves(#[case] start: Coordinate, #[case] expected: Vec<Coordinate>) {
        let board = mk_board(Piece::queen(Colour::White), start);
        let moves: Vec<Coordinate> = board.pseudo_queen_moves(&start).unwrap().collect();
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }

    #[rstest]
    #[case::rook_blocked_by_friendly(
        crate::piece::PieceType::Rook,
        Coordinate::new_unchecked(2, 2),
        Coordinate::new_unchecked(5, 2),
        Colour::White,
        vec![
            Coordinate::new_unchecked(3, 2),
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(1, 2),
            Coordinate::new_unchecked(0, 2),
            Coordinate::new_unchecked(2, 3),
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(2, 5),
            Coordinate::new_unchecked(2, 6),
            Coordinate::new_unchecked(2, 7),
            Coordinate::new_unchecked(2, 1),
            Coordinate::new_unchecked(2, 0),
        ]
    )]
    #[case::rook_captures_opponent(
        crate::piece::PieceType::Rook,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(4, 6),
        Colour::Black,
        vec![
            Coordinate::new_unchecked(5, 4),
            Coordinate::new_unchecked(6, 4),
            Coordinate::new_unchecked(7, 4),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(0, 4),
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(4, 6),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(4, 0),
        ]
    )]
    #[case::bishop_blocked_by_friendly(
        crate::piece::PieceType::Bishop,
        Coordinate::new_unchecked(2, 2),
        Coordinate::new_unchecked(4, 4),
        Colour::White,
        vec![
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(1, 3),
            Coordinate::new_unchecked(0, 4),
            Coordinate::new_unchecked(3, 1),
            Coordinate::new_unchecked(4, 0),
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(0, 0),
        ]
    )]
    #[case::bishop_captures_opponent(
        crate::piece::PieceType::Bishop,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(6, 6),
        Colour::Black,
        vec![
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(3, 5),
            Coordinate::new_unchecked(2, 6),
            Coordinate::new_unchecked(1, 7),
            Coordinate::new_unchecked(5, 3),
            Coordinate::new_unchecked(6, 2),
            Coordinate::new_unchecked(7, 1),
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(0, 0),
        ]
    )]
    #[case::queen_blocked_by_friendly(
        crate::piece::PieceType::Queen,
        Coordinate::new_unchecked(3, 3),
        Coordinate::new_unchecked(3, 6),
        Colour::White,
        vec![
            // Left
            Coordinate::new_unchecked(2, 3),
            Coordinate::new_unchecked(1, 3),
            Coordinate::new_unchecked(0, 3),
            // Down
            Coordinate::new_unchecked(3, 2),
            Coordinate::new_unchecked(3, 1),
            Coordinate::new_unchecked(3, 0),
            // Right
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(5, 3),
            Coordinate::new_unchecked(6, 3),
            Coordinate::new_unchecked(7, 3),
            // Up
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(3, 5),
            // Top-left
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(1, 5),
            Coordinate::new_unchecked(0, 6),
            // Bottom-left
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(0, 0),
            // Top-right
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(7, 7),
            // Bottom-right
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(5, 1),
            Coordinate::new_unchecked(6, 0),
        ]
    )]
    #[case::queen_captures_opponent(
        crate::piece::PieceType::Queen,
        Coordinate::new_unchecked(3, 3),
        Coordinate::new_unchecked(3, 6),
        Colour::Black,
        vec![
            // Left
            Coordinate::new_unchecked(2, 3),
            Coordinate::new_unchecked(1, 3),
            Coordinate::new_unchecked(0, 3),
            // Down
            Coordinate::new_unchecked(3, 2),
            Coordinate::new_unchecked(3, 1),
            Coordinate::new_unchecked(3, 0),
            // Right
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(5, 3),
            Coordinate::new_unchecked(6, 3),
            Coordinate::new_unchecked(7, 3),
            // Up
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(3, 5),
            Coordinate::new_unchecked(3, 6),
            // Top-left
            Coordinate::new_unchecked(2, 4),
            Coordinate::new_unchecked(1, 5),
            Coordinate::new_unchecked(0, 6),
            // Bottom-left
            Coordinate::new_unchecked(2, 2),
            Coordinate::new_unchecked(1, 1),
            Coordinate::new_unchecked(0, 0),
            // Top-right
            Coordinate::new_unchecked(4, 4),
            Coordinate::new_unchecked(5, 5),
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(7, 7),
            // Bottom-right
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(5, 1),
            Coordinate::new_unchecked(6, 0),
        ]
    )]
    fn sliding_piece_moves_with_blocking(
        #[case] piece_type: crate::piece::PieceType,
        #[case] piece_position: Coordinate,
        #[case] blocking_piece_position: Coordinate,
        #[case] blocking_piece_colour: Colour,
        #[case] expected: Vec<Coordinate>,
    ) {
        let mut board = Board::empty();

        let player_piece = Piece {
            piece_type,
            colour: Colour::White,
        };
        board.set_square(piece_position, Some(player_piece));

        let blocker = Piece::pawn(blocking_piece_colour);
        board.set_square(blocking_piece_position, Some(blocker));

        let moves: Vec<Coordinate> = match piece_type {
            crate::piece::PieceType::Rook => {
                board.pseudo_rook_moves(&piece_position).unwrap().collect()
            }
            crate::piece::PieceType::Bishop => board
                .pseudo_bishop_moves(&piece_position)
                .unwrap()
                .collect(),
            crate::piece::PieceType::Queen => {
                board.pseudo_queen_moves(&piece_position).unwrap().collect()
            }
            _ => panic!("Piece type not yet supported in test"),
        };

        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }
}
