use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use ecr_formats::fen::{Fen, FenPiecePlacements};
pub use ecr_shared::board::BoardCastleState;
use ecr_shared::coordinate::Coordinate;

use crate::pieces::{BoardPiece, PieceColor, PieceType};
use crate::r#move::Move;
use crate::utils::{get_en_passant_actual, new_rc_refcell};
use std::fmt::Formatter;

// Just exists so we can safely

/// The inner content of a square. Holds a reference-counted pointer to a [`RefCell`] that holds a
/// [`BoardPiece`].
pub type SquareInner = Rc<RefCell<BoardPiece>>;

/// A [`Board`] contains the current game of chess.
#[derive(Debug)]
pub struct Board {
    /// The representation of the board. A board consists of 8x8 squares. The first array is for the
    /// x, the second for the y coordinate. Since the board has 8 squares on each axis, an index of
    /// `0` to `7` is possible. Contains an [`Option<BoardPiece>`] since a square can be empty, which
    /// means that squares with [`None`] as value will be empty.
    board: Vec<Vec<Option<SquareInner>>>,
    /// Since a hybrid solution for saving the pieces is used, we save all pieces as well as
    pub(crate) pieces: Vec<SquareInner>,
    /// All moves that were played. Can be empty if the board gets created from e.g. the FEN
    /// notation.
    moves: Vec<Move>,

    /// The color that does the next move.
    pub(crate) to_move: PieceColor,

    /// The number of moves already done. Will be increased when a move occurs and light_to_move is
    /// `false`.
    pub(crate) move_number: usize,

    /// The amount of half moves done. A half move is any move where nothing gets captured and no
    /// pawn is moved. Resets to `0` if a non-half move occurs.
    pub(crate) half_move_amount: u8,

    /// Which castle actions are allowed? Only contains if it would be theoretically allowed, not
    /// representing if the castle would be blocked by another piece or similar.
    pub(crate) castle_state: BoardCastleState,

    /// Specifies the en passant target square that is currently possible. Only contains if it
    /// would be allowed theoretically, not checking if it would actually be possible.
    en_passant: Option<EnPassant>,

    /// Specifies how many times each square is threatened by a team.
    threatened_state: Vec<Vec<ThreatenedState>>,
}

/// If an en_passant is possible gives the target square where the pawn can also be captured and the
/// actual square the pawn is on.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct EnPassant {
    pub target_square: Coordinate,
    pub actual_square: Coordinate,
}

impl EnPassant {
    pub fn new_from_target_square(target_square: Coordinate) -> EnPassant {
        EnPassant {
            target_square,
            actual_square: get_en_passant_actual(target_square),
        }
    }
    pub fn from_option(square: Option<Coordinate>) -> Option<EnPassant> {
        if let Some(target_square) = square {
            return Some(EnPassant::new_from_target_square(target_square));
        }
        None
    }
}

/// Consists of two u8s that tell how many times each team threatens a square. Useful for
/// castling.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ThreatenedState {
    pub threatened_light: u8,
    pub threatened_dark: u8,
}

impl ThreatenedState {
    pub fn clear(&mut self) {
        self.threatened_dark = 0;
        self.threatened_light = 0;
    }
    pub fn get_by_team(&self, team: PieceColor) -> u8 {
        match team {
            PieceColor::Light => self.threatened_light,
            PieceColor::Dark => self.threatened_dark,
        }
    }
}

impl Clone for Board {
    /// Since we use RefCells we can't just clone a board but have to replace the references inside.
    fn clone(&self) -> Self {
        // First we create an empty board to clone to.
        let mut board_clone = Board::empty();
        // We need to replace the pieces inside the array
        let mut cloned_pieces = vec![];
        for inner in &self.pieces {
            let board_piece_to_add: BoardPiece = inner.borrow().deref().clone();
            let piece = Rc::new(RefCell::new(board_piece_to_add));
            cloned_pieces.push(piece);
            // TODO: This doesn't clone the Piece
        }
        board_clone.castle_state = *self.get_castle_state();
        board_clone.pieces = cloned_pieces;
        board_clone.fill_board_from_pieces();
        board_clone.en_passant = self.en_passant;
        board_clone.half_move_amount = self.get_half_move_amount();
        board_clone.move_number = self.get_move_number();
        board_clone.moves = self.moves.clone();
        board_clone.to_move = self.to_move;

        board_clone
    }
}

impl Board {
    /// Returns an empty board.
    pub fn empty() -> Board {
        Board {
            board: vec![vec![None; 8]; 8],
            pieces: vec![],
            moves: vec![],
            to_move: PieceColor::Light,
            move_number: 1,
            half_move_amount: 0,
            castle_state: BoardCastleState::default(),
            en_passant: None,
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

    /// Removes a piece from a given target square. DOES NOT SET IT OUT OF GAME!
    pub(crate) fn remove_piece(&mut self, target: Coordinate) {
        // First we get the right column of the piece
        let column = self.board.get_mut(target.get_x() as usize).unwrap();
        // Then we get the row as a range since splice() requires a range, which is totally necessary for changing one variable.
        let column_index_range = target.get_y() as usize..target.get_y() as usize;

        column.splice(column_index_range, vec![None]);
    }

    /// Returns if the next move should be done by the light color.
    pub fn get_light_to_move(&self) -> bool {
        match self.to_move {
            PieceColor::Light => true,
            PieceColor::Dark => false,
        }
    }

    fn fill_board_from_pieces(&mut self) {
        self.remove_all_board_pieces();
        for piece in &self.pieces.clone() {
            self.add_piece_to_board(piece.borrow().deref().clone());
        }
    }

    fn remove_all_board_pieces(&mut self) {
        let none_vector: Vec<Option<SquareInner>> =
            vec![None, None, None, None, None, None, None, None];
        self.board.fill(none_vector);
    }

    /// Returns the piece at the supplied coordinate on the board.
    pub fn get_at(&self, coordinate: Coordinate) -> Option<SquareInner> {
        // ? -> column not found
        let column = self.board.get(coordinate.get_x() as usize)?;
        // ? -> square not found
        let square = column.get(coordinate.get_y() as usize)?;
        // If it was found, clone the BoardPiece for future access
        square.as_ref().map(|v| Rc::clone(v))
    }

    /// Adds a piece to the board. Since a hybrid solution for saving the board is used, the piece
    /// gets added into the board array as well as the piece list.
    pub fn add_piece(&mut self, piece: BoardPiece) {
        self.add_piece_to_board(piece.clone());

        let square_inner = new_rc_refcell(piece);
        // Since we are using a hybrid approach for saving the board and its pieces, we have to add
        // the square to the list of all pieces, too
        self.pieces.push(square_inner);
    }

    /// Adds a piece to the board.
    pub fn add_piece_to_board(&mut self, piece: BoardPiece) {
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
    }

    pub fn get_pieces_by_type(&self, piece_type: PieceType) -> Vec<SquareInner> {
        let mut result: Vec<SquareInner> = vec![];
        for inner in self.pieces.clone() {
            if inner.deref().borrow().get_piece().get_type() == piece_type {
                result.push(inner);
            }
        }
        result
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
    pub fn get_en_passant_target(&self) -> Option<EnPassant> {
        self.en_passant
    }

    pub fn get_en_passant_target_option(&self) -> Option<Coordinate> {
        if let Some(en_passant) = self.get_en_passant_target() {
            return Some(en_passant.target_square);
        }
        None
    }

    /// Returns all pieces that are on the [`Board`].
    pub fn get_pieces(&self) -> &Vec<SquareInner> {
        &self.pieces
    }

    /// This function is useful for castling and checking whether a trade would be beneficial.
    pub fn get_threatened_state(&self, square: Coordinate) -> ThreatenedState {
        // We assume that the given coordinate is valid.
        let column = self.threatened_state.get(square.get_x() as usize).unwrap();
        *column.get(square.get_y() as usize).unwrap()
    }

    /// Sets the target square to the given ThreatenedState
    pub fn set_threatened(&mut self, square: Coordinate, state: ThreatenedState) {
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
        column.splice(column_index_range, vec![state]);
    }

    /// Adds a threat to the square by the given team.
    pub fn add_threat(&mut self, square: Coordinate, team: PieceColor) {
        let mut current_state = self.get_threatened_state(square).clone();
        match team {
            PieceColor::Light => {
                current_state.threatened_light += 1;
                self.set_threatened(square, current_state);
            }
            PieceColor::Dark => {
                current_state.threatened_dark += 1;
                self.set_threatened(square, current_state);
            }
        }
    }

    pub fn remove_all_threats(&mut self) {
        let to_replace = ThreatenedState {
            threatened_light: 0,
            threatened_dark: 0,
        };
        let range = 0_usize..7;
        for i in 0..=7 {
            let column = self.threatened_state.get_mut(i).unwrap();
            column.splice(
                range.clone(),
                vec![
                    to_replace, to_replace, to_replace, to_replace, to_replace, to_replace,
                    to_replace, to_replace,
                ],
            );
        }
    }

    /// This function returns a float, which returns a positive value if light is ahead and a
    /// negative value if  dark is ahead(MiniMax Implementation).
    pub fn eval_board(&self) -> f32 {
        self.eval()
    }

    /// This function returns the pieces of a team. Useful for the eval function as well as the move_gen function.
    pub fn get_team_pieces(&self, team_color: PieceColor) -> Vec<&RefCell<BoardPiece>> {
        let mut result = vec![];
        for piece in &self.pieces {
            if piece.as_ref().borrow().deref().get_color() == team_color {
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
        board.en_passant = EnPassant::from_option(f.en_passant);
        board.castle_state = f.castles;
        match f.light_to_move {
            true => board.to_move = PieceColor::Light,
            false => board.to_move = PieceColor::Dark,
        }

        // Add all pieces to the board
        for piece in f.piece_placements {
            board.add_piece(piece.into());
        }

        board.calculate_threatened_states();

        board
    }
}

impl From<Board> for Fen {
    fn from(board: Board) -> Self {
        let mut fen = Fen {
            piece_placements: FenPiecePlacements { pieces: Vec::new() },
            light_to_move: board.get_light_to_move(),
            castles: *board.get_castle_state(), // Copy is implemented for BoardCastleState
            en_passant: board.get_en_passant_target_option(),
            half_moves: board.get_half_move_amount(),
            move_number: board.get_move_number(),
        };

        // Add all pieces
        for p in board.get_pieces() {
            fen.piece_placements
                .pieces
                .push((p.deref().borrow().deref()).clone().into());
        }

        fen
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fen: {}, Eval: {}",
            Fen::from(self.clone()).to_string(),
            self.eval()
        )
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

            assert!(b.get_light_to_move());
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
            assert_eq!(None, b.en_passant);

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
        fn test_clone() {
            let default_board = Board::default();
            let cloned_board = default_board.clone();
            assert_eq!(cloned_board.pieces.len(), default_board.pieces.len());
            default_board.pieces[2].borrow().get_has_moved();
            cloned_board.pieces[2].borrow().get_has_moved();
            //assert_eq!(Fen::from(default_board), Fen::from(cloned_board));
        }

        #[test]
        fn test_remove_piece() {
            let mut default_board = Board::default();
            default_board.remove_piece((0, 1).into());
            assert_eq!(None, default_board.get_at((0, 1).into()));
        }

        #[test]
        fn test_get_light_to_move() {
            let mut b = Board::empty();
            assert!(b.get_light_to_move());

            b.to_move = PieceColor::Light;
            assert!(b.get_light_to_move());

            b.to_move = PieceColor::Dark;
            assert!(!b.get_light_to_move());
        }

        #[test]
        fn test_get_at() {
            let mut b = Board::empty();
            let p = BoardPiece::new_from_type(PieceType::Pawn, (2, 1).into(), PieceColor::Light);
            let column = b.board.get_mut(2).unwrap();
            column.insert(1, Some(Rc::new(RefCell::new(p.clone()))));

            assert_eq!(None, b.get_at((0 as u8, 0 as u8).into()));
            let square_from_board = b.get_at((2 as u8, 1 as u8).into()).unwrap();
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
                assert_eq!(&pawn1, pieces_piece.deref().borrow().deref());
                assert_eq!(pieces_piece, board_piece);
            }

            // Pawn 2
            {
                let pieces_piece = b.pieces.get(1).unwrap();
                let board_piece = b.board.get(5).unwrap().get(6).unwrap().as_ref().unwrap();
                assert_eq!(&pawn2, pieces_piece.deref().borrow().deref());
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
        fn test_eval_board() {
            let default_board: Board = board::Board::default();
            let result = default_board.eval_board();
            assert_eq!(0.0, result);
        }

        #[test]
        fn test_get_en_passant_target() {
            let mut b = Board::empty();
            assert_eq!(None, b.en_passant);

            b.en_passant = Some(EnPassant::new_from_target_square((3, 4).into()));
            assert_eq!(
                Some(EnPassant {
                    target_square: (3, 4).into(),
                    actual_square: (3, 5).into()
                }),
                b.get_en_passant_target()
            );
        }

        #[test]
        fn test_from_fen() {
            let fen: Fen = "2k5/8/8/8/8/4R3/8/2K5 b - - 3 6".parse().unwrap();
            let board: Board = fen.into();

            assert_eq!(3, board.pieces.len());
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::King, (2, 0).into(), PieceColor::Light),
                board
                    .get_at((2 as u8, 0 as u8).into())
                    .unwrap()
                    .deref()
                    .borrow()
                    .deref(),
            );
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::Rook, (4, 2).into(), PieceColor::Light),
                board
                    .get_at((4 as u8, 2 as u8).into())
                    .unwrap()
                    .deref()
                    .borrow()
                    .deref(),
            );
            assert_eq!(
                &BoardPiece::new_from_type(PieceType::King, (2, 7).into(), PieceColor::Dark),
                board
                    .get_at((2 as u8, 7 as u8).into())
                    .unwrap()
                    .deref()
                    .borrow()
                    .deref(),
            );

            assert_eq!(PieceColor::Dark, board.to_move);
            assert_eq!(None, board.en_passant);
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
            assert_eq!(
                1,
                board.get_threatened_state((4, 0).into()).threatened_light
            );
            assert_eq!(
                1,
                board.get_threatened_state((4, 1).into()).threatened_light
            );
            assert_eq!(
                1,
                board.get_threatened_state((4, 3).into()).threatened_light
            );
            assert_eq!(
                1,
                board.get_threatened_state((4, 4).into()).threatened_light
            );
            assert_eq!(
                1,
                board.get_threatened_state((3, 0).into()).threatened_light
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
            let state = ThreatenedState {
                threatened_light: 1,
                threatened_dark: 3,
            };
            empty_board.set_threatened(square, state);
            let result = empty_board.get_threatened_state(square);
            let expected = ThreatenedState {
                threatened_light: 1,
                threatened_dark: 3,
            };
            assert_eq!(result, expected);

            let state = empty_board.get_threatened_state((0, 0).into());
            let expected2 = ThreatenedState {
                threatened_light: 0,
                threatened_dark: 0,
            };
            assert_eq!(expected2, state);
        }
        #[test]
        fn test_get_team_pieces() {
            let default_board = Board::default();
            let light_pieces = default_board.get_team_pieces(PieceColor::Light);
            let dark_pieces = default_board.get_team_pieces(PieceColor::Dark);
            assert_eq!(16, light_pieces.len());
            assert_eq!(16, dark_pieces.len());
        }
    }
}
