use std::cell::RefCell;
use std::rc::Rc;

use crate::coordinate::Coordinate;
use crate::formats::fen::Fen;
use crate::pieces::{BoardPiece, PieceColor, PieceType};
use crate::r#move::Move;
use crate::utils::new_rc_refcell;

/// The inner content of a square. Holds a reference-counted pointer to a [`RefCell`] that holds a
/// [`BoardPiece`].
pub type SquareInner = Rc<RefCell<BoardPiece>>;

/// Holds information whether castling is allowed on the specific sides.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardCastleState {
    /// Can light castle on king side?
    pub light_king_side: bool,
    /// Can light castle on queen side?
    pub light_queen_side: bool,
    /// Can dark castle on king side?
    pub dark_king_side: bool,
    /// Can dark castle on queen side?
    pub dark_queen_side: bool,
}

impl Default for BoardCastleState {
    /// By default, every castle action is possible.
    fn default() -> Self {
        BoardCastleState {
            light_king_side: true,
            light_queen_side: true,
            dark_king_side: true,
            dark_queen_side: true,
        }
    }
}

impl BoardCastleState {
    /// Returns if any castle action is still allowed.
    pub fn is_any_possible(&self) -> bool {
        self.light_king_side || self.light_queen_side || self.dark_king_side || self.dark_queen_side
    }
}

/// A [`Board`] contains the current game of chess.
#[derive(Debug, Clone)]
pub struct Board {
    /// The representation of the board. A board consists of 8x8 squares. The first array is for the
    /// x, the second for the y coordinate. Since the board has 8 squares on each axis, an index of
    /// `0` to `7` is possible. Contains an [`Option<BoardPiece>`] since a square can be empty, which
    /// means that squares with [`None`] as value will be empty.
    board: Vec<Vec<Option<SquareInner>>>,
    /// Since a hybrid solution for saving the pieces is used, we save all pieces as well as
    pieces: Vec<SquareInner>,
    /// All moves that were played. Can be empty if the board gets created from e.g. the FEN
    /// notation.
    moves: Vec<Move>,

    /// If the next move should be done by the light color.
    light_to_move: bool,
    /// The number of moves already done. Will be increased when a move occurs and light_to_move is
    /// `false`.
    move_number: usize,
    /// The amount of half moves done. A half move is any move where nothing gets captured and no
    /// pawn is moved. Resets to `0` if a non-half move occurs.
    half_move_amount: usize,
    /// Which castle actions are allowed? Only contains if it would be theoretically allowed, not
    /// representing if the castle would be blocked by another piece or similar.
    castle_state: BoardCastleState,
    /// Specifies the en passant target square that is currently possible. Only contains if it
    /// would be allowed theoretically, not checking if it would actually be possible.
    en_passant_target: Option<Coordinate>,
}

impl Board {
    /// Returns an empty board.
    pub fn empty() -> Board {
        Board {
            board: vec![vec![None; 8]; 8],
            pieces: vec![],
            moves: vec![],
            light_to_move: true,
            move_number: 1,
            half_move_amount: 0,
            castle_state: BoardCastleState::default(),
            en_passant_target: None,
        }
    }

    /// Returns if the next move should be done by the light color.
    pub fn get_light_to_move(&self) -> bool {
        self.light_to_move
    }

    /// Returns the piece at the supplied coordinate on the board.
    pub fn get_at(&self, coordinate: Coordinate) -> Option<SquareInner> {
        // ? -> column not found
        let column = self.board.get(coordinate.get_x() as usize)?;
        // ? -> square not found
        let square = column.get(coordinate.get_y() as usize)?;
        // If it was found, clone the BoardPiece for future access
        match square {
            Some(v) => Some(Rc::clone(v)),
            None => None,
        }
    }

    /// Adds a piece to the board. Since a hybrid solution for saving the board is used, the piece
    /// gets added into the board array as well as the piece list.
    pub fn add_piece(&mut self, piece: BoardPiece) {
        let x_coordinate = piece.get_coordinate().get_x() as usize;
        let y_coordinate = piece.get_coordinate().get_y() as usize;

        // Get the column (x coordinate) as mutable reference
        let column = self.board.get_mut(x_coordinate).unwrap();
        // Since .splice wants a range but we only want to replace one specific part, we just create
        // a range that consists of the x coordinate
        let column_index_range = y_coordinate..=y_coordinate;

        let square_inner = new_rc_refcell(piece);

        // Replaces the square with the supplied piece
        column.splice(column_index_range, vec![Some(Rc::clone(&square_inner))]);

        // Since we are using a hybrid approach for saving the board and its pieces, we have to add
        // the square to the list of all pieces, too
        self.pieces.push(square_inner);
    }

    /// Returns the current move number.
    pub fn get_move_number(&self) -> usize {
        self.move_number
    }

    /// Returns the amount of half moves done.
    pub fn get_half_move_amount(&self) -> usize {
        self.half_move_amount
    }

    /// Returns the castle moves that are still allowed.
    pub fn get_castle_state(&self) -> &BoardCastleState {
        &self.castle_state
    }

    /// Returns the currently possible en passant target square.
    pub fn get_en_passant_target(&self) -> Option<Coordinate> {
        self.en_passant_target
    }

    /// Returns all pieces that are on the [`Board`].
    pub fn get_pieces(&self) -> &Vec<SquareInner> {
        &self.pieces
    }
}

impl Default for Board {
    /// Returns the board with the default chess pieces placed on it.
    fn default() -> Self {
        let mut board = Board::empty();

        // Pawns
        for i in 0..=7 {
            // Light pawns
            board.add_piece(
                BoardPiece::new_from_type(
                    PieceType::Pawn, (i as u8, 1).into(), PieceColor::Light,
                )
            );

            // Dark pawns
            board.add_piece(
                BoardPiece::new_from_type(
                    PieceType::Pawn, (i as u8, 6).into(), PieceColor::Dark,
                )
            );
        }

        // Light king
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::King, (4, 0).into(), PieceColor::Light,
            )
        );
        // Dark king
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::King, (4, 7).into(), PieceColor::Dark,
            )
        );

        // Light queen
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Queen, (3, 0).into(), PieceColor::Light,
            )
        );
        // Dark queen
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Queen, (3, 7).into(), PieceColor::Dark,
            )
        );

        // Light rooks
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Rook, (0, 0).into(), PieceColor::Light,
            )
        );
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Rook, (7, 0).into(), PieceColor::Light,
            )
        );
        // Dark rooks
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Rook, (0, 7).into(), PieceColor::Dark,
            )
        );
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Rook, (7, 7).into(), PieceColor::Dark,
            )
        );

        // Light knights
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Knight, (1, 0).into(), PieceColor::Light,
            )
        );
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Knight, (6, 0).into(), PieceColor::Light,
            )
        );
        // Dark knights
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Knight, (1, 7).into(), PieceColor::Dark,
            )
        );
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Knight, (6, 7).into(), PieceColor::Dark,
            )
        );

        // Light bishops
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Bishop, (2, 0).into(), PieceColor::Light,
            )
        );
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Bishop, (5, 0).into(), PieceColor::Light,
            )
        );
        // Dark bishops
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Bishop, (2, 7).into(), PieceColor::Dark,
            )
        );
        board.add_piece(
            BoardPiece::new_from_type(
                PieceType::Bishop, (5, 7).into(), PieceColor::Dark,
            )
        );

        board
    }
}

impl From<Fen> for Board {
    fn from(f: Fen) -> Self {
        let mut board = Board::empty();

        // Set the attributes of the board state
        board.move_number = f.move_number;
        board.half_move_amount = f.half_moves;
        board.en_passant_target = f.en_passant;
        board.castle_state = f.castles;
        board.light_to_move = f.light_to_move;

        // Add all pieces to the board
        for piece in f.piece_placements {
            board.add_piece(piece.into());
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod board {
        use std::ops::Deref;
        use std::str::FromStr;

        use crate::pieces::PieceType;

        use super::*;

        #[test]
        fn test_empty() {
            let b = Board::empty();

            assert!(b.light_to_move);
            assert_eq!(1, b.move_number);
            assert_eq!(0, b.half_move_amount);
            assert_eq!(BoardCastleState {
                light_king_side: true,
                light_queen_side: true,
                dark_king_side: true,
                dark_queen_side: true,
            }, b.castle_state);
            assert_eq!(None, b.en_passant_target);

            assert_eq!(0, b.moves.len());
            assert_eq!(0, b.pieces.len());

            assert_eq!(8, b.board.len());
            for i in 0..=7 {
                let elements = b.board.get(i).unwrap();

                for j in 0..=7 {
                    // Some(None) consists of Some for element found and None for no piece on the board
                    assert_eq!(Some(&None), elements.get(j));
                }
            }
        }

        #[test]
        fn test_get_light_to_move() {
            let mut b = Board::empty();
            assert!(b.get_light_to_move());

            b.light_to_move = true;
            assert!(b.get_light_to_move());

            b.light_to_move = false;
            assert!(!b.get_light_to_move());
        }

        #[test]
        fn test_get_at() {
            let mut b = Board::empty();
            let p = BoardPiece::new_from_type(PieceType::Pawn, (2, 1).into(), PieceColor::Light);
            let column = b.board.get_mut(2).unwrap();
            column.insert(1, Some(Rc::new(RefCell::new(p.clone()))));

            assert_eq!(None, b.get_at((0, 0).into()));
            let square_from_board = b.get_at((2, 1).into()).unwrap();
            let piece_from_board = square_from_board.borrow_mut();
            assert_eq!(p, *piece_from_board);
        }

        #[test]
        fn test_add_piece() {
            let mut b = Board::empty();
            let pawn1 = BoardPiece::new_from_type(PieceType::Pawn, (2, 1).into(), PieceColor::Light);
            let pawn2 = BoardPiece::new_from_type(PieceType::Pawn, (5, 6).into(), PieceColor::Dark);

            b.add_piece(pawn1.clone());
            b.add_piece(pawn2.clone());

            // Pawn 1
            {
                let pieces_piece = b.pieces.get(0).unwrap();
                let board_piece = b.board.get(2).unwrap().get(1).unwrap().as_ref().unwrap();
                assert_eq!(&pawn1, pieces_piece.borrow().deref());
                assert_eq!(pieces_piece, board_piece);
            }

            // Pawn 2
            {
                let pieces_piece = b.pieces.get(1).unwrap();
                let board_piece = b.board.get(5).unwrap().get(6).unwrap().as_ref().unwrap();
                assert_eq!(&pawn2, pieces_piece.borrow().deref());
                assert_eq!(pieces_piece, board_piece);
            }

            // When adding 1 to the y coordinate, nothing should be there (checking for the range of
            // splice)
            {
                let board_piece = b.board.get(2).unwrap().get(2).unwrap().as_ref();
                assert_eq!(None, board_piece);
            }
        }

        #[test]
        fn test_get_move_number() {
            let mut b = Board::empty();
            assert_eq!(1, b.get_move_number());

            b.move_number = 1337;
            assert_eq!(1337, b.get_move_number());
        }

        #[test]
        fn test_get_half_move_amount() {
            let mut b = Board::empty();
            assert_eq!(0, b.get_half_move_amount());

            b.half_move_amount = 420;
            assert_eq!(420, b.get_half_move_amount());
        }

        #[test]
        fn test_get_castle_state() {
            let mut b = Board::empty();
            assert_eq!(&BoardCastleState {
                light_king_side: true,
                light_queen_side: true,
                dark_king_side: true,
                dark_queen_side: true,
            }, b.get_castle_state());

            b.castle_state.dark_king_side = false;
            b.castle_state.dark_queen_side = false;
            assert_eq!(&BoardCastleState {
                light_king_side: true,
                light_queen_side: true,
                dark_king_side: false,
                dark_queen_side: false,
            }, b.get_castle_state());
        }

        #[test]
        fn test_get_en_passant_target() {
            let mut b = Board::empty();
            assert_eq!(None, b.en_passant_target);

            b.en_passant_target = Some((3, 4).into());
            assert_eq!(Some((3, 4).into()), b.get_en_passant_target());
        }

        #[test]
        fn test_from_fen() {
            let fen: Fen = "2k5/8/8/8/8/4R3/8/2K5 b - - 3 6".parse().unwrap();
            let board: Board = fen.into();

            assert_eq!(3, board.pieces.len());
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::King, (2, 0).into(), PieceColor::Light),
                board.get_at((2, 0).into()).unwrap().borrow().deref(),
            );
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::Rook, (4, 2).into(), PieceColor::Light),
                board.get_at((4, 2).into()).unwrap().borrow().deref(),
            );
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::King, (2, 7).into(), PieceColor::Dark),
                board.get_at((2, 7).into()).unwrap().borrow().deref(),
            );

            assert_eq!(false, board.light_to_move);
            assert_eq!(None, board.en_passant_target);
            assert_eq!(3, board.half_move_amount);
            assert_eq!(6, board.move_number);
            assert_eq!(BoardCastleState {
                light_king_side: false,
                light_queen_side: false,
                dark_king_side: false,
                dark_queen_side: false,
            }, board.castle_state);
        }

        #[test]
        fn test_get_pieces() {
            let b = Board::default();
            assert_eq!(32, b.pieces.len());
            assert_eq!(32, Board::default().get_pieces().len());

            let mut b = Board::from(Fen::from_str("2k5/8/8/8/8/4R3/8/2K5 b - - 3 6").unwrap());
            assert_eq!(3, b.pieces.len());
            assert_eq!(3, b.get_pieces().len());

            b.add_piece(BoardPiece::new_from_type(PieceType::Pawn, (1, 1).into(), PieceColor::Light));
            assert_eq!(4, b.pieces.len());
            assert_eq!(4, b.get_pieces().len());
        }

        #[test]
        fn test_default() {
            let b = Board::default();
            let f: Fen = b.into();
            assert_eq!(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"), f.to_string());
        }
    }

    mod board_castle_state {
        use super::*;

        #[test]
        fn test_is_any_possible() {
            assert!(!BoardCastleState {
                light_king_side: false,
                light_queen_side: false,
                dark_king_side: false,
                dark_queen_side: false,
            }.is_any_possible());
            assert!(BoardCastleState {
                light_king_side: true,
                light_queen_side: false,
                dark_king_side: false,
                dark_queen_side: false,
            }.is_any_possible());
            assert!(BoardCastleState {
                light_king_side: false,
                light_queen_side: true,
                dark_king_side: false,
                dark_queen_side: false,
            }.is_any_possible());
            assert!(BoardCastleState {
                light_king_side: false,
                light_queen_side: false,
                dark_king_side: true,
                dark_queen_side: false,
            }.is_any_possible());
            assert!(BoardCastleState {
                light_king_side: false,
                light_queen_side: false,
                dark_king_side: false,
                dark_queen_side: true,
            }.is_any_possible());
            assert!(BoardCastleState {
                light_king_side: true,
                light_queen_side: false,
                dark_king_side: true,
                dark_queen_side: false,
            }.is_any_possible());
            assert!(BoardCastleState {
                light_king_side: true,
                light_queen_side: true,
                dark_king_side: true,
                dark_queen_side: true,
            }.is_any_possible());
        }

        #[test]
        fn test_default() {
            assert_eq!(BoardCastleState {
                light_king_side: true,
                light_queen_side: true,
                dark_king_side: true,
                dark_queen_side: true,
            }, BoardCastleState::default());
        }
    }
}