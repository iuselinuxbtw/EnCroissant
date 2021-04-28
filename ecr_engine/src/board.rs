use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub use ecr_shared::board::BoardCastleState; // Just exists so we can safely

use ecr_shared::coordinate::Coordinate;
use ecr_formats::fen::{Fen, FenPiecePlacements};
use crate::pieces::{BoardPiece, PieceColor, PieceType};
use crate::pieces::move_gen::{BasicMove, CastleMove, CastleMoveType};
use crate::r#move::{Move, Moves};
use crate::utils::new_rc_refcell;

/// The inner content of a square. Holds a reference-counted pointer to a [`RefCell`] that holds a
/// [`BoardPiece`].
pub type SquareInner = Rc<RefCell<BoardPiece>>;

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
    half_move_amount: u8,
    /// Which castle actions are allowed? Only contains if it would be theoretically allowed, not
    /// representing if the castle would be blocked by another piece or similar.
    castle_state: BoardCastleState,
    /// Specifies the en passant target square that is currently possible. Only contains if it
    /// would be allowed theoretically, not checking if it would actually be possible.
    en_passant_target: Option<Coordinate>,

    /// Specifies how many times each square is threatened by a team.
    threatened_state: Vec<Vec<ThreatenedState>>,
}

/// Consists of two u8s that tell how many times each team threatens a square. Useful for
/// castling.
#[derive(Debug, Clone, PartialEq)]
pub struct ThreatenedState {
    pub threatened_light: u8,
    pub threatened_dark: u8,
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
            threatened_state: vec![
                vec![
                    ThreatenedState {
                        threatened_light: 0,
                        threatened_dark: 0
                    };
                    8
                ];
                8
            ],
        }
    }

    /// This function moves a piece from a given start square to another square, contained in a
    /// BasicMove.
    pub fn r#move(&mut self, start: &Coordinate, basic_move: &BasicMove) {
        // TODO: Tests need to be written for this!
        // We can safely unwrap here since no move is generated without a piece at the start of it.
        let piece = self.get_at(start).unwrap();

        let target_square = basic_move.get_target_square();

        // Update the piece coordinate to the new coordinates.
        piece.borrow_mut().set_coordinate(&target_square);

        // First we remove the piece from the original square on the board.
        self.remove_piece(start);

        if basic_move.capture {
            self.capture_piece(&piece, &target_square);
        }

        let mut piece_to_add: BoardPiece = piece.borrow().deref().clone();
        piece_to_add.set_coordinate(&target_square);
        let piece_type: PieceType = piece.borrow().deref().get_piece().get_type();

        if self.is_pawn_promotion(piece_type, &target_square) {
            // TODO: We need some way to choose a different piece if we can do a promotion. For now every promotion we do is just to the queen.
            piece_to_add = BoardPiece::new_from_type(
                PieceType::Queen,
                target_square,
                piece_to_add.get_color(),
            );
        }
        // Then we add the piece to the target square.
        self.add_piece(piece_to_add);

        // And we of course have to increase the move number
        self.move_number += 1;

        // We have to get the half moves
        self.count_half_moves(&piece_type, basic_move.capture);
    }

    fn is_pawn_promotion(&self, piece_type: PieceType, target: &Coordinate) -> bool {
        if piece_type == PieceType::Pawn {
            // Pawns can't move backwards so checking the color is redundant
            if target.get_y() == 7 || target.get_y() == 0 {
                return true;
            }
        }
        false
    }

    /// This function is called every move and is responsible for increasing/resetting the half move counter.
    fn count_half_moves(&mut self, piece_type: &PieceType, capture: bool) {
        match piece_type {
            PieceType::Pawn => self.half_move_amount = 0,
            _ => self.half_move_amount += 1,
        }
        if capture {
            self.half_move_amount = 0
        }
    }

    /// Removes a piece from a given target square. DOES NOT SET IT OUT OF GAME!
    fn remove_piece(&mut self, target: &Coordinate) {
        // First we get the right column of the piece
        let column = self.board.get_mut(target.get_x() as usize).unwrap();
        // Then we get the row as a range since splice() requires a range, which is totally necessary for changing one variable.
        let column_index_range = target.get_y() as usize..target.get_y() as usize;

        column.splice(column_index_range, None);
    }

    /// This function removes the piece on the given coordinate and sets it out of game.
    fn capture_piece(&mut self, target: &SquareInner, target_square: &Coordinate) {
        target.borrow_mut().set_out_of_game();
        self.remove_piece(target_square);
    }
    /// Returns if the next move should be done by the light color.
    pub fn get_light_to_move(&self) -> bool {
        self.light_to_move
    }

    /// Returns the piece at the supplied coordinate on the board.
    pub fn get_at(&self, coordinate: &Coordinate) -> Option<SquareInner> {
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
        let x_coordinate = piece.get_coordinate().get_x();
        let y_coordinate = piece.get_coordinate().get_y();

        // Get the column (x coordinate) as mutable reference
        let column = self.board.get_mut(x_coordinate as usize).unwrap();

        // Since .splice wants a range but we only want to replace one specific part, we just create
        // a range that consists of the x coordinate
        let column_index_range = y_coordinate as usize..=y_coordinate as usize;

        let square_inner: SquareInner = new_rc_refcell(piece);

        // Replaces the square with the supplied piece
        column.splice(column_index_range, vec![Some(Rc::clone(&square_inner))]);

        // Since we are using a hybrid approach for saving the board and its pieces, we have to add
        // the square to the list of all pieces, too
        self.pieces.push(square_inner);
    }

    // TODO: We need a test for this which should be some mid-game board.
    /// Executes a given CastleMove by moving the king first and then the rook
    pub fn castle(&mut self, castle_move: CastleMove) {
        // First we move the king to the target square.
        // TODO: We don't actually need the to: Coordinate in the CastleMove
        match castle_move.move_type {
            CastleMoveType::LightKingSide => {
                // Move the king
                // TODO: These increase the move counter two times and add two half_moves, which should not happen.
                self.r#move(
                    &(4, 0).into(),
                    &BasicMove {
                        capture: false,
                        to: castle_move.to,
                    },
                );
                // Move the rook
                self.r#move(
                    &(7, 0).into(),
                    &BasicMove {
                        capture: false,
                        to: (4, 0).into(),
                    },
                );
            }
            CastleMoveType::LightQueenSide => {
                self.r#move(
                    &(4, 0).into(),
                    &BasicMove {
                        capture: false,
                        to: castle_move.to,
                    },
                );
                self.r#move(
                    &(0, 0).into(),
                    &BasicMove {
                        capture: false,
                        to: (0, 3).into(),
                    },
                );
            }
            CastleMoveType::DarkKingSide => {
                self.r#move(
                    &(4, 7).into(),
                    &BasicMove {
                        capture: false,
                        to: castle_move.to,
                    },
                );
                self.r#move(
                    &(7, 7).into(),
                    &BasicMove {
                        capture: false,
                        to: (5, 7).into(),
                    },
                );
            }
            CastleMoveType::DarkQueenSide => {
                self.r#move(
                    &(4, 7).into(),
                    &BasicMove {
                        capture: false,
                        to: castle_move.to,
                    },
                );
                self.r#move(
                    &(0, 7).into(),
                    &BasicMove {
                        capture: false,
                        to: (3, 0).into(),
                    },
                );
            }
        }
    }

    /// Returns the current move number.
    pub fn get_move_number(&self) -> usize {
        self.move_number
    }

    /// Returns the amount of half moves done.
    pub fn get_half_move_amount(&self) -> u8 {
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

    /// This function is useful for castling and checking whether a trade would be beneficial.
    pub fn is_threatened(&self, square: Coordinate) -> &ThreatenedState {
        // We assume that the given coordinate is valid.
        let column = self.threatened_state.get(square.get_x() as usize).unwrap();
        let state = column.get(square.get_y() as usize).unwrap();

        state
    }

    /// Sets the target square to the given ThreatenedState
    pub fn set_threatened(&mut self, square: Coordinate, state: &ThreatenedState) {
        // First we need to get the column
        let column = self
            .threatened_state
            .get_mut(square.get_x() as usize)
            .unwrap();
        // Then we have to create the range which we want to replace but since we only want to
        // replace one value we create a range from the start to the start
        let column_index_range = square.get_y() as usize..=square.get_y() as usize;

        // And finally replace it since this function would be pointless otherwise...
        // We need to create a vector since the replace_with needs to be an iterator.
        // This can probably be solved more elegantly than with a range and iterator but it works...
        column.splice(column_index_range, vec![state.clone()]);
    }

    /// This function returns all possible pseudo legal moves (OF BOTH TEAMS!).
    ///
    /// We could also only get one move and bet on it being the best one which would certainly be
    /// interesting...
    pub fn get_all_pseudo_legal_moves(&self) -> Vec<Moves> {
        let mut result: Vec<Moves> = vec![];
        for piece in &self.pieces {
            result.push(Moves {
                from: piece.borrow().deref().get_coordinate(),
                basic_move: piece.borrow().deref().get_piece().get_pseudo_legal_moves(
                    &self,
                    &piece.as_ref().borrow().deref().get_coordinate(),
                    &piece.as_ref().borrow().deref().get_color(),
                    piece.as_ref().borrow().deref().get_has_moved(),
                ),
            });
        }
        result
    }

    /// We should not filter our normal move_gen for legal moves if we are checked, since that would
    /// be inefficient. We can make a special move generator for legal moves during being checked.
    pub fn check_move_gen(&self) -> Vec<BasicMove> {
        todo!()
    }

    /// This function returns a float, which returns a positive value if light is ahead and a
    /// negative value if  dark is ahead(MiniMax Implementation).
    pub fn eval_board(&self) -> f32 {
        // This function will probably be moved to another file as it gets more complex.
        // This currently only considers the value of the pieces on the board and not the positions.
        // TODO: Make this also evaluate the position
        let mut value_light: usize = 0;
        let mut value_dark: usize = 0;
        let light_pieces = self.get_team_pieces(PieceColor::Light);
        let dark_pieces = self.get_team_pieces(PieceColor::Dark);
        for piece in light_pieces {
            value_light += piece.borrow().deref().get_piece().get_value() as usize;
        }
        for piece in dark_pieces {
            value_dark += piece.borrow().deref().get_piece().get_value() as usize;
        }
        (value_light - value_dark) as f32
    }

    /// This function returns the pieces of a team. Useful for the eval function as well as the move_gen function.
    pub fn get_team_pieces(&self, team_color: PieceColor) -> Vec<&RefCell<BoardPiece>> {
        let mut result = vec![];
        for piece in &self.pieces {
            if piece.as_ref().borrow().deref().get_color().clone() == team_color {
                result.push(piece.deref());
            }
        }
        result
    }
}

impl Default for Board {
    /// Returns the board with the default chess pieces placed on it.
    fn default() -> Self {
        let mut board = Board::empty();

        // Pawns
        for i in 0..=7 {
            // Light pawns
            board.add_piece(BoardPiece::new_from_type(
                PieceType::Pawn,
                (i as u8, 1).into(),
                PieceColor::Light,
            ));

            // Dark pawns
            board.add_piece(BoardPiece::new_from_type(
                PieceType::Pawn,
                (i as u8, 6).into(),
                PieceColor::Dark,
            ));
        }

        // Light king
        board.add_piece(BoardPiece::new_from_type(
            PieceType::King,
            (4, 0).into(),
            PieceColor::Light,
        ));
        // Dark king
        board.add_piece(BoardPiece::new_from_type(
            PieceType::King,
            (4, 7).into(),
            PieceColor::Dark,
        ));

        // Light queen
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Queen,
            (3, 0).into(),
            PieceColor::Light,
        ));
        // Dark queen
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Queen,
            (3, 7).into(),
            PieceColor::Dark,
        ));

        // Light rooks
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Rook,
            (0, 0).into(),
            PieceColor::Light,
        ));
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Rook,
            (7, 0).into(),
            PieceColor::Light,
        ));
        // Dark rooks
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Rook,
            (0, 7).into(),
            PieceColor::Dark,
        ));
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Rook,
            (7, 7).into(),
            PieceColor::Dark,
        ));

        // Light knights
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Knight,
            (1, 0).into(),
            PieceColor::Light,
        ));
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Knight,
            (6, 0).into(),
            PieceColor::Light,
        ));
        // Dark knights
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Knight,
            (1, 7).into(),
            PieceColor::Dark,
        ));
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Knight,
            (6, 7).into(),
            PieceColor::Dark,
        ));

        // Light bishops
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Bishop,
            (2, 0).into(),
            PieceColor::Light,
        ));
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Bishop,
            (5, 0).into(),
            PieceColor::Light,
        ));
        // Dark bishops
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Bishop,
            (2, 7).into(),
            PieceColor::Dark,
        ));
        board.add_piece(BoardPiece::new_from_type(
            PieceType::Bishop,
            (5, 7).into(),
            PieceColor::Dark,
        ));

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

impl From<Board> for Fen {
    fn from(board: Board) -> Self {
        let mut fen = Fen {
            piece_placements: FenPiecePlacements { pieces: Vec::new() },
            light_to_move: board.get_light_to_move(),
            castles: *board.get_castle_state(), // Copy is implemented for BoardCastleState
            en_passant: board.get_en_passant_target(),
            half_moves: board.get_half_move_amount(),
            move_number: board.get_move_number(),
        };

        // Add all pieces
        for p in board.get_pieces() {
            fen.piece_placements
                .pieces
                .push((p.borrow().deref()).clone().into());
        }

        fen
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
            assert_eq!(
                BoardCastleState {
                    light_king_side: true,
                    light_queen_side: true,
                    dark_king_side: true,
                    dark_queen_side: true,
                },
                b.castle_state
            );
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

            assert_eq!(None, b.get_at(&(0 as u8, 0 as u8).into()));
            let square_from_board = b.get_at(&(2 as u8, 1 as u8).into()).unwrap();
            let piece_from_board = square_from_board.borrow_mut();
            assert_eq!(p, *piece_from_board);
        }

        #[test]
        fn test_add_piece() {
            let mut b = Board::empty();
            let pawn1 =
                BoardPiece::new_from_type(PieceType::Pawn, (2, 1).into(), PieceColor::Light);
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

            b.half_move_amount = 42;
            assert_eq!(42, b.get_half_move_amount());
        }

        #[test]
        fn test_get_castle_state() {
            let mut b = Board::empty();
            assert_eq!(
                &BoardCastleState {
                    light_king_side: true,
                    light_queen_side: true,
                    dark_king_side: true,
                    dark_queen_side: true,
                },
                b.get_castle_state()
            );

            b.castle_state.dark_king_side = false;
            b.castle_state.dark_queen_side = false;
            assert_eq!(
                &BoardCastleState {
                    light_king_side: true,
                    light_queen_side: true,
                    dark_king_side: false,
                    dark_queen_side: false,
                },
                b.get_castle_state()
            );
        }

        #[test]
        fn test_get_all_pseudo_legal_moves() {
            let default_board = Board::default();
            let result = default_board.get_all_pseudo_legal_moves().len();
        }

        #[test]
        fn test_eval_board() {
            let default_board: Board = board::Board::default();
            let result = default_board.eval_board();
            assert_eq!(0.0, result);
        }

        #[test]
        fn test_get_en_passant_target() {
            let mut b = Board::empty();
            assert_eq!(None, b.en_passant_target);

            b.en_passant_target = Some((3, 4).into());
            assert_eq!(Some((3, 4).into()), b.get_en_passant_target());
        }

        /*
                #[test]
                fn test_move(){
                    todo!()
                }*/

        #[test]
        fn test_from_fen() {
            let fen: Fen = "2k5/8/8/8/8/4R3/8/2K5 b - - 3 6".parse().unwrap();
            let board: Board = fen.into();

            assert_eq!(3, board.pieces.len());
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::King, (2, 0).into(), PieceColor::Light),
                board
                    .get_at(&(2 as u8, 0 as u8).into())
                    .unwrap()
                    .borrow()
                    .deref(),
            );
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::Rook, (4, 2).into(), PieceColor::Light),
                board
                    .get_at(&(4 as u8, 2 as u8).into())
                    .unwrap()
                    .borrow()
                    .deref(),
            );
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::King, (2, 7).into(), PieceColor::Dark),
                board
                    .get_at(&(2 as u8, 7 as u8).into())
                    .unwrap()
                    .borrow()
                    .deref(),
            );

            assert_eq!(false, board.light_to_move);
            assert_eq!(None, board.en_passant_target);
            assert_eq!(3, board.half_move_amount);
            assert_eq!(6, board.move_number);
            assert_eq!(
                BoardCastleState {
                    light_king_side: false,
                    light_queen_side: false,
                    dark_king_side: false,
                    dark_queen_side: false,
                },
                board.castle_state
            );
        }

        #[test]
        fn test_fen_from_board() {
            let mut b = Board::empty();
            b.add_piece(BoardPiece::new_from_type(
                PieceType::Pawn,
                (5, 3).into(),
                PieceColor::Light,
            ));
            b.add_piece(BoardPiece::new_from_type(
                PieceType::King,
                (4, 0).into(),
                PieceColor::Light,
            ));
            b.add_piece(BoardPiece::new_from_type(
                PieceType::King,
                (4, 7).into(),
                PieceColor::Dark,
            ));

            assert_eq!(
                Fen {
                    piece_placements: FenPiecePlacements {
                        pieces: vec![
                            ((5, 3).into(), PieceColor::Light, PieceType::Pawn).into(),
                            ((4, 0).into(), PieceColor::Light, PieceType::King).into(),
                            ((4, 7).into(), PieceColor::Dark, PieceType::King).into(),
                        ],
                    },
                    light_to_move: true,
                    castles: BoardCastleState {
                        light_king_side: true,
                        light_queen_side: true,
                        dark_king_side: true,
                        dark_queen_side: true,
                    },
                    en_passant: None,
                    half_moves: 0,
                    move_number: 1,
                },
                b.into()
            );
        }

        #[test]
        fn test_get_pieces() {
            let b = Board::default();
            assert_eq!(32, b.pieces.len());
            assert_eq!(32, Board::default().get_pieces().len());

            let mut b = Board::from(Fen::from_str("2k5/8/8/8/8/4R3/8/2K5 b - - 3 6").unwrap());
            assert_eq!(3, b.pieces.len());
            assert_eq!(3, b.get_pieces().len());

            b.add_piece(BoardPiece::new_from_type(
                PieceType::Pawn,
                (1, 1).into(),
                PieceColor::Light,
            ));
            assert_eq!(4, b.pieces.len());
            assert_eq!(4, b.get_pieces().len());
        }

        #[test]
        fn test_default() {
            let b = Board::default();
            let f: Fen = b.into();
            assert_eq!(
                String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
                f.to_string()
            );
        }
        #[test]
        fn test_threatened_state() {
            let mut empty_board = Board::empty();
            let square = (5, 6).into();
            let state = &ThreatenedState {
                threatened_light: 1,
                threatened_dark: 3,
            };
            empty_board.set_threatened(square, state);
            let result = empty_board.is_threatened(square);
            let expected = &ThreatenedState {
                threatened_light: 1,
                threatened_dark: 3,
            };
            assert_eq!(result, expected);

            let state = empty_board.is_threatened((0, 0).into());
            let expected2 = &ThreatenedState {
                threatened_light: 0,
                threatened_dark: 0,
            };
            assert_eq!(state, expected2);
        }
    }
}
