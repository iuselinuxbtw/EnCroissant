use ecr_shared::coordinate::Coordinate;

use crate::board::Board;
use crate::pieces::move_gen::{diagonal_moves, linear_moves, BasicMove};
use crate::pieces::{PieceColor, PieceType};

use super::Piece;

#[derive(Debug, PartialEq, Clone)]
pub struct Queen {}

impl Piece for Queen {
    fn get_type(&self) -> PieceType {
        PieceType::Queen
    }
    fn get_pseudo_legal_moves(
        &self,
        board: &Board,
        piece_coordinate: &Coordinate,
        piece_color: &PieceColor,
        #[allow(unused_variables)] has_moved: bool,
    ) -> Vec<BasicMove> {
        let mut result: Vec<BasicMove> = vec![];
        result.append(&mut linear_moves(piece_coordinate, board, piece_color));
        result.append(&mut diagonal_moves(piece_coordinate, board, piece_color));
        result
    }

    fn get_value(&self) -> u8 {
        90
    }
}

#[cfg(test)]
mod tests {
    use crate::board;
    use crate::pieces::move_gen::Capture;

    use super::*;

    fn get_piece() -> Queen {
        Queen {}
    }

    #[test]
    fn test_get_shortcode_algebraic() {
        assert_eq!("Q", get_piece().get_shortcode_algebraic());
    }

    #[test]
    fn test_get_type() {
        assert_eq!(PieceType::Queen, get_piece().get_type());
    }

    #[test]
    fn test_get_pseudo_legal_moves() {
        let default_board = board::Board::default();
        let piece = Queen {};
        let result =
            piece.get_pseudo_legal_moves(&default_board, &(3, 4).into(), &PieceColor::Dark, true);
        let expected: Vec<BasicMove> = vec![
            // North
            BasicMove {
                to: (3, 5).into(),
                capture: None,
            },
            // East
            BasicMove {
                to: (4, 4).into(),
                capture: None,
            },
            BasicMove {
                to: (5, 4).into(),
                capture: None,
            },
            BasicMove {
                to: (6, 4).into(),
                capture: None,
            },
            BasicMove {
                to: (7, 4).into(),
                capture: None,
            },
            // South
            BasicMove {
                to: (3, 3).into(),
                capture: None,
            },
            BasicMove {
                to: (3, 2).into(),
                capture: None,
            },
            BasicMove {
                to: (3, 1).into(),
                capture: Some(Capture {
                    piece_type: PieceType::Pawn,
                    target: (3, 1).into(),
                }),
            },
            // West
            BasicMove {
                to: (2, 4).into(),
                capture: None,
            },
            BasicMove {
                to: (1, 4).into(),
                capture: None,
            },
            BasicMove {
                to: (0, 4).into(),
                capture: None,
            },
            // North-west
            BasicMove {
                to: (2, 5).into(),
                capture: None,
            },
            // North-east
            BasicMove {
                to: (4, 5).into(),
                capture: None,
            },
            // South-east
            BasicMove {
                to: (4, 3).into(),
                capture: None,
            },
            BasicMove {
                to: (5, 2).into(),
                capture: None,
            },
            BasicMove {
                to: (6, 1).into(),
                capture: Some(Capture {
                    piece_type: PieceType::Pawn,
                    target: (6, 1).into(),
                }),
            },
            // South-west
            BasicMove {
                to: (2, 3).into(),
                capture: None,
            },
            BasicMove {
                to: (1, 2).into(),
                capture: None,
            },
            BasicMove {
                to: (0, 1).into(),
                capture: Some(Capture {
                    piece_type: PieceType::Pawn,
                    target: (0, 1).into(),
                }),
            },
        ];
        assert_eq!(expected, result);
    }
}
