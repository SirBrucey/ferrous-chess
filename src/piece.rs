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
    position.apply_deltas(deltas.into_iter())
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
        let piece = Piece::knight(Colour::White);
        let moves = pseudo_knight_moves(&board, &piece, start);
        assert_eq!(moves.len(), expected.len());
        for m in moves {
            assert!(expected.contains(&m));
        }
    }
}
