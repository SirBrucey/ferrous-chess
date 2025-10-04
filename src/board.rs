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

impl<'a> IntoIterator for &'a Board {
    type Item = Option<(Coordinate, Piece)>;
    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator {
            board: self,
            row: 0,
            col: 0,
        }
    }
}

pub(crate) struct BoardIterator<'a> {
    board: &'a Board,
    row: usize,
    col: usize,
}

impl Iterator for BoardIterator<'_> {
    type Item = Option<(Coordinate, Piece)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= 8 {
            return None;
        }
        let current_col = self.col;
        let current_row = self.row;
        let maybe_piece = self.board.squares[current_row][current_col];

        self.col += 1;
        if self.col >= 8 {
            self.col = 0;
            self.row += 1;
        }

        Some(maybe_piece.map(|piece| {
            (
                Coordinate::new_unchecked(current_col as u8, current_row as u8),
                piece,
            )
        }))
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

// Direction constants for move generation
const ORTHOGONAL_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const DIAGONAL_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const KING_DIRS: [(i8, i8); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const KNIGHT_DELTAS: [(i8, i8); 8] = [
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
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

    // TODO: This method should validate that moves are for pieces matching self.turn.
    // Pseudo move methods only validate piece type, not turn order.
    fn get_legal_moves(&self) -> Vec<Move> {
        todo!()
    }

    fn validate_piece_type(
        &self,
        position: &Coordinate,
        expected_piece_type: crate::piece::PieceType,
    ) -> Result<Piece, String> {
        match self.get_square(position) {
            Some(piece) if piece.piece_type != expected_piece_type => Err(format!(
                "The piece at the given position is not a {}",
                expected_piece_type
            )),
            Some(piece) => Ok(piece),
            None => Err("No piece at the given position".to_string()),
        }
    }

    fn is_valid_destination(&self, coord: &Coordinate, moving_piece_colour: Colour) -> bool {
        !self
            .get_square(coord)
            .is_some_and(|piece| piece.colour == moving_piece_colour)
    }

    fn sliding_piece_moves(
        &self,
        position: &Coordinate,
        piece_type: crate::piece::PieceType,
        directions: &[(i8, i8)],
    ) -> Result<Vec<Coordinate>, String> {
        let piece = self.validate_piece_type(position, piece_type)?;
        Ok(directions
            .iter()
            .flat_map(move |&dir| RayIterator {
                board: self,
                current: *position,
                direction: dir,
                moving_piece_colour: piece.colour,
                stopped: false,
            })
            .collect())
    }

    fn step_piece_moves(
        &self,
        position: &Coordinate,
        piece_type: crate::piece::PieceType,
        deltas: &[(i8, i8)],
    ) -> Result<Vec<Coordinate>, String> {
        let piece = self.validate_piece_type(position, piece_type)?;
        Ok(position
            .apply_deltas(deltas.iter().copied())
            .filter(|coord| self.is_valid_destination(coord, piece.colour))
            .collect())
    }

    fn pseudo_knight_moves(&self, position: &Coordinate) -> Result<Vec<Coordinate>, String> {
        self.step_piece_moves(position, crate::piece::PieceType::Knight, &KNIGHT_DELTAS)
    }

    fn pseudo_rook_moves(&self, position: &Coordinate) -> Result<Vec<Coordinate>, String> {
        self.sliding_piece_moves(position, crate::piece::PieceType::Rook, &ORTHOGONAL_DIRS)
    }

    fn pseudo_bishop_moves(&self, position: &Coordinate) -> Result<Vec<Coordinate>, String> {
        self.sliding_piece_moves(position, crate::piece::PieceType::Bishop, &DIAGONAL_DIRS)
    }

    fn pseudo_queen_moves(&self, position: &Coordinate) -> Result<Vec<Coordinate>, String> {
        let piece = self.validate_piece_type(position, crate::piece::PieceType::Queen)?;
        Ok(ORTHOGONAL_DIRS
            .iter()
            .chain(DIAGONAL_DIRS.iter())
            .flat_map(move |&dir| RayIterator {
                board: self,
                current: *position,
                direction: dir,
                moving_piece_colour: piece.colour,
                stopped: false,
            })
            .collect())
    }

    fn pseudo_king_moves(&self, position: &Coordinate) -> Result<Vec<Coordinate>, String> {
        self.step_piece_moves(position, crate::piece::PieceType::King, &KING_DIRS)
    }

    fn pseudo_pawn_moves(&self, position: &Coordinate) -> Result<Vec<Coordinate>, String> {
        let piece = self.validate_piece_type(position, crate::piece::PieceType::Pawn)?;
        let direction = match piece.colour {
            Colour::White => 1,
            Colour::Black => -1,
        };
        let on_starting_rank = match piece.colour {
            Colour::White => position.y == 1,
            Colour::Black => position.y == 6,
        };

        let forward_one = position.try_apply_delta((0, direction)).ok();
        let forward_two = position.try_apply_delta((0, 2 * direction)).ok();

        Ok(
            // Move one square
            forward_one
                .filter(|coord| self.get_square(coord).is_none())
                .into_iter()
                // Move two squares forward from starting position
                .chain(
                    forward_two
                        .filter(|coord| {
                            on_starting_rank
                                && forward_one
                                    .map(|f| self.get_square(&f).is_none())
                                    .unwrap_or(false)
                                && self.get_square(coord).is_none()
                        })
                        .into_iter(),
                )
                .chain(
                    // Captures
                    position
                        .apply_deltas([(1, direction), (-1, direction)].into_iter())
                        .filter(move |coord| {
                            self.get_square(coord)
                                .is_some_and(|p| p.colour != piece.colour)
                        }),
                )
                .chain(
                    // FIXME: En passant captures would go here
                    std::iter::empty(),
                )
                .collect(),
        )
    }

    fn pseudo_moves_by_type(
        &self,
        position: &Coordinate,
        piece_type: crate::piece::PieceType,
    ) -> Result<Vec<Coordinate>, String> {
        match piece_type {
            crate::piece::PieceType::Pawn => self.pseudo_pawn_moves(position),
            crate::piece::PieceType::Knight => self.pseudo_knight_moves(position),
            crate::piece::PieceType::Bishop => self.pseudo_bishop_moves(position),
            crate::piece::PieceType::Rook => self.pseudo_rook_moves(position),
            crate::piece::PieceType::Queen => self.pseudo_queen_moves(position),
            crate::piece::PieceType::King => self.pseudo_king_moves(position),
        }
    }

    fn is_board_legal(&self) -> bool {
        todo!()
    }

    fn is_in_check(&self, colour: Colour) -> Result<bool, String> {
        let (king_pos, opponent_pieces) = self.into_iter().fold(
            (None, Vec::new()),
            |(king_pos, mut opponent_pieces), square| {
                if let Some((coord, piece)) = square {
                    if piece.colour != colour {
                        opponent_pieces.push((coord, piece));
                    } else if piece.piece_type == crate::piece::PieceType::King {
                        return (Some(coord), opponent_pieces);
                    }
                }
                (king_pos, opponent_pieces)
            },
        );
        let king_pos = match king_pos {
            Some(pos) => pos,
            None => return Err("No king found for the given colour".to_string()),
        };
        for (coord, piece) in opponent_pieces {
            let pseudo_moves = self.pseudo_moves_by_type(&coord, piece.piece_type)?;
            if pseudo_moves.contains(&king_pos) {
                return Ok(true);
            }
        }
        Ok(false)
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
    moving_piece_colour: Colour,
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
                    Some(piece) if piece.colour == self.moving_piece_colour => {
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

impl Colour {
    pub(crate) fn opposite(&self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
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
        let moves = board.pseudo_knight_moves(&start).unwrap();
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
        let moves = board.pseudo_rook_moves(&start).unwrap();
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
        let moves = board.pseudo_bishop_moves(&start).unwrap();
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
        let moves = board.pseudo_queen_moves(&start).unwrap();
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }

    #[rstest]
    #[case::middle(
        Coordinate::new_unchecked(4, 4),
        vec![
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(3, 5),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(5, 3),
            Coordinate::new_unchecked(5, 4),
            Coordinate::new_unchecked(5, 5),
        ]
    )]
    #[case::bottom_left(
        Coordinate::new_unchecked(0, 0),
        vec![
            Coordinate::new_unchecked(0, 1),
            Coordinate::new_unchecked(1, 0),
            Coordinate::new_unchecked(1, 1),
        ]
    )]
    #[case::bottom_edge(
        Coordinate::new_unchecked(4, 0),
        vec![
            Coordinate::new_unchecked(3, 0),
            Coordinate::new_unchecked(3, 1),
            Coordinate::new_unchecked(4, 1),
            Coordinate::new_unchecked(5, 0),
            Coordinate::new_unchecked(5, 1),
        ]
    )]
    #[case::bottom_right(
        Coordinate::new_unchecked(7, 0),
        vec![
            Coordinate::new_unchecked(6, 0),
            Coordinate::new_unchecked(6, 1),
            Coordinate::new_unchecked(7, 1),
        ]
    )]
    #[case::left_middle(
        Coordinate::new_unchecked(0, 4),
        vec![
            Coordinate::new_unchecked(0, 3),
            Coordinate::new_unchecked(0, 5),
            Coordinate::new_unchecked(1, 3),
            Coordinate::new_unchecked(1, 4),
            Coordinate::new_unchecked(1, 5),
        ]
    )]
    #[case::right_middle(
        Coordinate::new_unchecked(7, 4),
        vec![
            Coordinate::new_unchecked(6, 3),
            Coordinate::new_unchecked(6, 4),
            Coordinate::new_unchecked(6, 5),
            Coordinate::new_unchecked(7, 3),
            Coordinate::new_unchecked(7, 5),
        ]
    )]
    #[case::top_left(
        Coordinate::new_unchecked(0, 7),
        vec![
            Coordinate::new_unchecked(0, 6),
            Coordinate::new_unchecked(1, 6),
            Coordinate::new_unchecked(1, 7),
        ]
    )]
    #[case::top_edge(
        Coordinate::new_unchecked(4, 7),
        vec![
            Coordinate::new_unchecked(3, 6),
            Coordinate::new_unchecked(3, 7),
            Coordinate::new_unchecked(4, 6),
            Coordinate::new_unchecked(5, 6),
            Coordinate::new_unchecked(5, 7),
        ]
    )]
    #[case::top_right(
        Coordinate::new_unchecked(7, 7),
        vec![
            Coordinate::new_unchecked(6, 6),
            Coordinate::new_unchecked(6, 7),
            Coordinate::new_unchecked(7, 6),
        ]
    )]
    fn king_moves(#[case] start: Coordinate, #[case] expected: Vec<Coordinate>) {
        let board = mk_board(Piece::king(Colour::White), start);
        let moves = board.pseudo_king_moves(&start).unwrap();
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }

    #[rstest]
    #[case::white_start(
        Coordinate::new_unchecked(4, 1),
        Colour::White,
        vec![
            Coordinate::new_unchecked(4, 2),
            Coordinate::new_unchecked(4, 3),
        ]
    )]
    #[case::black_start(
        Coordinate::new_unchecked(4, 6),
        Colour::Black,
        vec![
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(4, 4),
        ]
    )]
    fn pawn_moves(
        #[case] start: Coordinate,
        #[case] colour: Colour,
        #[case] expected: Vec<Coordinate>,
    ) {
        let mut board = Board {
            turn: colour,
            ..Default::default()
        };
        board.set_square(start, Some(Piece::pawn(colour)));

        let moves = board.pseudo_pawn_moves(&start).unwrap();
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }

    #[rstest]
    #[case::white_left_capture(
        Coordinate::new_unchecked(4, 4),
        Colour::White,
        Coordinate::new_unchecked(3, 5)
    )]
    #[case::white_right_capture(
        Coordinate::new_unchecked(4, 4),
        Colour::White,
        Coordinate::new_unchecked(5, 5)
    )]
    #[case::black_left_capture(
        Coordinate::new_unchecked(4, 4),
        Colour::Black,
        Coordinate::new_unchecked(3, 3)
    )]
    #[case::black_right_capture(
        Coordinate::new_unchecked(4, 4),
        Colour::Black,
        Coordinate::new_unchecked(5, 3)
    )]
    fn pawn_move_captures(
        #[case] start: Coordinate,
        #[case] colour: Colour,
        #[case] target: Coordinate,
    ) {
        use crate::piece::PieceType;

        let mut board = Board {
            turn: colour,
            ..Default::default()
        };
        board.set_square(start, Some(Piece::pawn(colour)));
        board.set_square(
            target,
            Some(Piece {
                piece_type: PieceType::Rook,
                colour: colour.opposite(),
            }),
        );

        let moves = board.pseudo_pawn_moves(&start).unwrap();
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&target));
    }

    #[rstest]
    #[case::white_blocked_ahead(
        Coordinate::new_unchecked(4, 4),
        Colour::White,
        Coordinate::new_unchecked(4, 5)
    )]
    #[case::black_blocked_ahead(
        Coordinate::new_unchecked(4, 4),
        Colour::Black,
        Coordinate::new_unchecked(4, 3)
    )]
    fn pawn_move_blocked(
        #[case] start: Coordinate,
        #[case] colour: Colour,
        #[case] blocker: Coordinate,
    ) {
        let mut board = Board {
            turn: colour,
            ..Default::default()
        };
        board.set_square(start, Some(Piece::pawn(colour)));
        board.set_square(
            blocker,
            Some(Piece {
                piece_type: crate::piece::PieceType::Rook,
                colour,
            }),
        );

        let moves = board.pseudo_pawn_moves(&start).unwrap();
        assert_eq!(moves.len(), 0);
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
    #[case::king_blocked_by_friendly(
        crate::piece::PieceType::King,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(5, 5),
        Colour::White,
        vec![
            Coordinate::new_unchecked(3, 3),
            Coordinate::new_unchecked(3, 4),
            Coordinate::new_unchecked(3, 5),
            Coordinate::new_unchecked(4, 3),
            Coordinate::new_unchecked(4, 5),
            Coordinate::new_unchecked(5, 3),
            Coordinate::new_unchecked(5, 4),
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

        let moves = match piece_type {
            crate::piece::PieceType::Rook => board.pseudo_rook_moves(&piece_position).unwrap(),
            crate::piece::PieceType::Bishop => board.pseudo_bishop_moves(&piece_position).unwrap(),
            crate::piece::PieceType::Queen => board.pseudo_queen_moves(&piece_position).unwrap(),
            crate::piece::PieceType::King => board.pseudo_king_moves(&piece_position).unwrap(),
            _ => panic!("Piece type not yet supported in test"),
        };

        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }

    fn board_iteration() {
        assert_eq!(
            Board::default().into_iter().flatten().collect::<Vec<_>>(),
            vec![
                (Coordinate::new_unchecked(0, 0), Piece::rook(Colour::White)),
                (
                    Coordinate::new_unchecked(1, 0),
                    Piece::knight(Colour::White)
                ),
                (
                    Coordinate::new_unchecked(2, 0),
                    Piece::bishop(Colour::White)
                ),
                (Coordinate::new_unchecked(3, 0), Piece::queen(Colour::White)),
                (Coordinate::new_unchecked(4, 0), Piece::king(Colour::White)),
                (
                    Coordinate::new_unchecked(5, 0),
                    Piece::bishop(Colour::White)
                ),
                (
                    Coordinate::new_unchecked(6, 0),
                    Piece::knight(Colour::White)
                ),
                (Coordinate::new_unchecked(7, 0), Piece::rook(Colour::White)),
                (Coordinate::new_unchecked(0, 1), Piece::pawn(Colour::White)),
                (Coordinate::new_unchecked(1, 1), Piece::pawn(Colour::White)),
                (Coordinate::new_unchecked(2, 1), Piece::pawn(Colour::White)),
                (Coordinate::new_unchecked(3, 1), Piece::pawn(Colour::White)),
                (Coordinate::new_unchecked(4, 1), Piece::pawn(Colour::White)),
                (Coordinate::new_unchecked(5, 1), Piece::pawn(Colour::White)),
                (Coordinate::new_unchecked(6, 1), Piece::pawn(Colour::White)),
                (Coordinate::new_unchecked(7, 1), Piece::pawn(Colour::White)),
                (Coordinate::new_unchecked(0, 6), Piece::pawn(Colour::Black)),
                (Coordinate::new_unchecked(1, 6), Piece::pawn(Colour::Black)),
                (Coordinate::new_unchecked(2, 6), Piece::pawn(Colour::Black)),
                (Coordinate::new_unchecked(3, 6), Piece::pawn(Colour::Black)),
                (Coordinate::new_unchecked(4, 6), Piece::pawn(Colour::Black)),
                (Coordinate::new_unchecked(5, 6), Piece::pawn(Colour::Black)),
                (Coordinate::new_unchecked(6, 6), Piece::pawn(Colour::Black)),
                (Coordinate::new_unchecked(7, 6), Piece::pawn(Colour::Black)),
                (Coordinate::new_unchecked(0, 7), Piece::rook(Colour::Black)),
                (
                    Coordinate::new_unchecked(1, 7),
                    Piece::knight(Colour::Black)
                ),
                (
                    Coordinate::new_unchecked(2, 7),
                    Piece::bishop(Colour::Black)
                ),
                (Coordinate::new_unchecked(3, 7), Piece::queen(Colour::Black)),
                (Coordinate::new_unchecked(4, 7), Piece::king(Colour::Black)),
                (
                    Coordinate::new_unchecked(5, 7),
                    Piece::bishop(Colour::Black)
                ),
                (
                    Coordinate::new_unchecked(6, 7),
                    Piece::knight(Colour::Black)
                ),
                (Coordinate::new_unchecked(7, 7), Piece::rook(Colour::Black))
            ]
        );
    }

    #[rstest]
    #[case::white_king_checked_by_black_pawn_left(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(3, 5),
        crate::piece::PieceType::Pawn
    )]
    #[case::white_king_checked_by_black_pawn_right(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(5, 5),
        crate::piece::PieceType::Pawn
    )]
    #[case::black_king_checked_by_white_pawn_left(
        Colour::Black,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(3, 3),
        crate::piece::PieceType::Pawn
    )]
    #[case::black_king_checked_by_white_pawn_right(
        Colour::Black,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(5, 3),
        crate::piece::PieceType::Pawn
    )]
    #[case::white_king_checked_by_knight(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(5, 6),
        crate::piece::PieceType::Knight
    )]
    #[case::black_king_checked_by_knight(
        Colour::Black,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(6, 5),
        crate::piece::PieceType::Knight
    )]
    #[case::white_king_checked_by_bishop_diagonal(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(6, 6),
        crate::piece::PieceType::Bishop
    )]
    #[case::black_king_checked_by_bishop_diagonal(
        Colour::Black,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(2, 2),
        crate::piece::PieceType::Bishop
    )]
    #[case::white_king_checked_by_rook_horizontal(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(7, 4),
        crate::piece::PieceType::Rook
    )]
    #[case::white_king_checked_by_rook_vertical(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(4, 0),
        crate::piece::PieceType::Rook
    )]
    #[case::black_king_checked_by_rook_horizontal(
        Colour::Black,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(0, 4),
        crate::piece::PieceType::Rook
    )]
    #[case::white_king_checked_by_queen_diagonal(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(7, 7),
        crate::piece::PieceType::Queen
    )]
    #[case::white_king_checked_by_queen_horizontal(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(4, 7),
        crate::piece::PieceType::Queen
    )]
    #[case::black_king_checked_by_queen_diagonal(
        Colour::Black,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(1, 1),
        crate::piece::PieceType::Queen
    )]
    #[case::white_king_checked_by_king(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        Coordinate::new_unchecked(5, 5),
        crate::piece::PieceType::King
    )]
    fn is_in_check_returns_true(
        #[case] king_colour: Colour,
        #[case] king_pos: Coordinate,
        #[case] attacker_pos: Coordinate,
        #[case] attacker_type: crate::piece::PieceType,
    ) {
        let mut board = Board::empty();
        board.set_square(king_pos, Some(Piece::king(king_colour)));

        let attacker = Piece {
            piece_type: attacker_type,
            colour: king_colour.opposite(),
        };
        board.set_square(attacker_pos, Some(attacker));

        assert!(board.is_in_check(king_colour).unwrap());
    }

    #[rstest]
    #[case::king_alone(Colour::White, Coordinate::new_unchecked(4, 4), vec![])]
    #[case::king_with_friendly_pieces(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        vec![
            (Coordinate::new_unchecked(3, 3), Piece::rook(Colour::White)),
            (Coordinate::new_unchecked(5, 5), Piece::bishop(Colour::White)),
        ]
    )]
    #[case::enemy_pawn_wrong_diagonal(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        vec![
            (Coordinate::new_unchecked(3, 3), Piece::pawn(Colour::Black)),
        ]
    )]
    #[case::enemy_pawn_in_front(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        vec![
            (Coordinate::new_unchecked(4, 5), Piece::pawn(Colour::Black)),
        ]
    )]
    #[case::enemy_knight_not_in_range(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        vec![
            (Coordinate::new_unchecked(4, 6), Piece::knight(Colour::Black)),
        ]
    )]
    #[case::enemy_rook_blocked(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        vec![
            (Coordinate::new_unchecked(4, 7), Piece::rook(Colour::Black)),
            (Coordinate::new_unchecked(4, 6), Piece::pawn(Colour::Black)),
        ]
    )]
    #[case::enemy_bishop_blocked(
        Colour::White,
        Coordinate::new_unchecked(4, 4),
        vec![
            (Coordinate::new_unchecked(7, 7), Piece::bishop(Colour::Black)),
            (Coordinate::new_unchecked(5, 5), Piece::pawn(Colour::White)),
        ]
    )]
    #[case::enemy_queen_blocked(
        Colour::Black,
        Coordinate::new_unchecked(4, 4),
        vec![
            (Coordinate::new_unchecked(0, 4), Piece::queen(Colour::White)),
            (Coordinate::new_unchecked(2, 4), Piece::knight(Colour::Black)),
        ]
    )]
    fn is_in_check_returns_false(
        #[case] king_colour: Colour,
        #[case] king_pos: Coordinate,
        #[case] pieces: Vec<(Coordinate, Piece)>,
    ) {
        let mut board = Board::empty();
        board.set_square(king_pos, Some(Piece::king(king_colour)));

        for (pos, piece) in pieces {
            board.set_square(pos, Some(piece));
        }

        assert!(!board.is_in_check(king_colour).unwrap());
    }

    #[test]
    fn is_in_check_errors_when_no_king() {
        let board = Board::empty();
        let result = board.is_in_check(Colour::White);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No king found for the given colour");
    }

    #[test]
    fn is_in_check_with_multiple_attackers() {
        let mut board = Board::empty();
        let king_pos = Coordinate::new_unchecked(4, 4);
        board.set_square(king_pos, Some(Piece::king(Colour::White)));

        // Two black pieces both attacking the king
        board.set_square(Coordinate::new_unchecked(4, 7), Some(Piece::rook(Colour::Black)));
        board.set_square(Coordinate::new_unchecked(6, 6), Some(Piece::bishop(Colour::Black)));

        assert!(board.is_in_check(Colour::White).unwrap());
    }
}
