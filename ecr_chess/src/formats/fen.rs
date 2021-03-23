//! Contains an implementation for the Forsyth-Edwards Notation (FEN). More information about it can
//! be found in [chess programming wiki](https://www.chessprogramming.org/Forsyth-Edwards_Notation).

use std::num::ParseIntError;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::coordinate::{char_to_x_coordinate, Coordinate};
use crate::pieces::{PieceColor, PieceType};
use crate::board::BoardCastleState;
use std::str::FromStr;

lazy_static! {
    /// This is the regex pattern that we use to split the string. What may be a bit confusing is
    /// that we have 'rnbqkpRNBQKP1-8' twice in the string. This is because the last line has no `/`
    /// at the end.
    /// # Example
    /// It splits this string `r4rk1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3RK2R b K - 1 16` into
    /// the following parts:
    /// ```text
    ///              piece_placements                     to_move castles en_passant half_moves move_number
    /// r4rk1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3RK2R    b       K        -          1          16
    /// ```
    pub static ref FEN_REGEX: Regex = Regex::new(r#"^(?P<piece_placements>((?:[rnbqkpRNBQKP1-8]{1,8}/){7})[rnbqkpRNBQKP1-8]{1,8})\s(?P<to_move>[b|w])\s(?P<castles>-|K?Q?k?q?)\s(?P<en_passant>-|[a-h][3|6])\s(?P<half_moves>\d+)\s(?P<move_number>\d+)$"#).unwrap();

    /// Parses the piece placement part of the FEN.
    pub static ref FEN_PIECE_PLACEMENT_REGEX: Regex = Regex::new(r#"^(((?:[rnbqkpRNBQKP1-8]{1,8}/){7})[rnbqkpRNBQKP1-8]{1,8})$"#).unwrap();
}

/// An error that occurred while doing actions related to the FEN.
#[derive(Debug, Error, PartialEq)]
pub enum FenError {
    #[error("invalid FEN string")]
    InvalidFenString,

    #[error("invalid FEN piece placement string")]
    InvalidFenPiecePlacementString,

    #[error("cannot parse as int: {0}")]
    ParseIntError(#[from] ParseIntError),
}

/// Holds the information a FEN represents.
#[derive(Debug, PartialEq, Clone)]
pub struct Fen {
    pub piece_placements: FenPiecePlacements,
    pub light_to_move: bool,
    pub castles: BoardCastleState,
    pub en_passant: Option<Coordinate>,
    pub half_moves: usize,
    pub move_number: usize,
}

impl FromStr for Fen {
    type Err = FenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First we split the string with regex
        let caps = match FEN_REGEX.captures(s) {
            None => Err(FenError::InvalidFenString),
            Some(v) => Ok(v)
        }?;
        Ok(Fen {
            // Unwrapping is safe here since the FEN string got already validated so this does not
            // return an error
            piece_placements: (&caps["piece_placements"]).parse().unwrap(),
            light_to_move: match &caps["to_move"] {
                "w" => true,
                _ => false, // Includes "b"
            },
            castles: resolve_board_castle_state(String::from(&caps["castles"])),
            en_passant: match &caps["en_passant"] {
                "-" => None,
                v => Some({
                    // If there are en_passant options in the string, then we just save those as a
                    // string (for now).
                    let coordinates: Vec<char> = v.chars().collect();
                    (char_to_x_coordinate(coordinates[0]), coordinates[1] as u8).into()
                }),
            },
            half_moves: (&caps["half_moves"]).parse()?,
            move_number: (&caps["move_number"]).parse()?,
        })
    }
}

/// Contains information about a piece that is stored inside Fen. This is their [`Coordinate`],
/// their [`PieceColor`] and their [`PieceType`].
pub type FenPiece = (Coordinate, PieceColor, PieceType);

/// Stores all pieces notated in the FEN.
#[derive(Debug, PartialEq, Clone)]
pub struct FenPiecePlacements {
    pub pieces: Vec<FenPiece>,
}

impl IntoIterator for FenPiecePlacements {
    type Item = FenPiece;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Just returns the [`IntoIter<FenPiece>`](struct@std::vec::IntoIter) of the pieces [`Vec`]
    /// that is stored inside the [`FenPiecePlacements`] struct.
    fn into_iter(self) -> Self::IntoIter {
        self.pieces.into_iter()
    }
}

impl FromStr for FenPiecePlacements {
    type Err = FenError;

    /// Parses the FEN positions string into actual chess pieces with positions.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FEN_PIECE_PLACEMENT_REGEX.captures(s).ok_or(FenError::InvalidFenPiecePlacementString)?;

        let mut result: Vec<FenPiece> = Vec::new();

        let rows: Vec<&str> = s.split("/").collect();
        for i in 0..rows.len() {
            let row = &rows[i];
            let chars: Vec<char> = row.chars().collect();

            let mut x: u8 = 0;
            for c in chars {
                if c.is_ascii_digit() {
                    // If we get a number we have to skip that amount of squares. We have to subtract 1
                    // since we already add one per loop.
                    // We already checked this is an ascii digit, so we can safely unwrap the parsed
                    // value.
                    x += c.to_string().parse::<u8>().unwrap();
                } else {
                    // Since FEN piece placement starts from the top of the board, we have to
                    // subtract the index (y coordinate) from 7 to also start from the top.
                    result.push(resolve_piece_code(x, (7 - i) as u8, c));
                    x += 1;
                }
            }
        };

        Ok(FenPiecePlacements {
            pieces: result,
        })
    }
}

/// Resolves a piece from coordinates and a FEN piece code.
fn resolve_piece_code(x: u8, y: u8, code: char) -> FenPiece {
    let coordinates: Coordinate = (x, y).into();

    // By default dark but if its uppercase it's light
    // lowercase -> dark, uppercase -> light
    let mut color: PieceColor = PieceColor::Dark;
    if code.is_uppercase() {
        color = PieceColor::Light;
    };

    // Match the char code to the corresponding 'PieceType'
    let piece_type: PieceType = match code.to_ascii_lowercase() {
        'r' => PieceType::Rook,
        'n' => PieceType::Knight,
        'b' => PieceType::Bishop,
        'q' => PieceType::Queen,
        'k' => PieceType::King,
        'p' => PieceType::Pawn,
        // Can't happen because of the applied regex pattern
        _ => PieceType::King
    };

    (coordinates, color, piece_type)
}

/// Resolves a Fen Castling ability string and returns a [`BoardCastleState`].
/// # Example
/// Parsing the string `Qkq`:
/// ```
/// # use ecr_chess::board::BoardCastleState;
/// # use ecr_chess::formats::fen;
/// #
/// assert_eq!(BoardCastleState {
///     light_king_side: false,
///     light_queen_side: true,
///     dark_king_side: true,
///     dark_queen_side: true,
/// }, fen::resolve_board_castle_state(String::from("Qkq")));
/// ```
pub fn resolve_board_castle_state(state: String) -> BoardCastleState {
    let mut bcs = BoardCastleState {
        light_king_side: false,
        light_queen_side: false,
        dark_king_side: false,
        dark_queen_side: false,
    };

    if state.contains("q") {
        bcs.dark_queen_side = true;
    }
    if state.contains("k") {
        bcs.dark_king_side = true;
    }
    if state.contains("K") {
        bcs.light_king_side = true;
    }
    if state.contains("Q") {
        bcs.light_queen_side = true;
    }

    bcs
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fen_regex {
        use super::*;

        #[test]
        fn test_input() {
            let caps = FEN_REGEX.captures("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
            assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", &caps["piece_placements"]);
            assert_eq!("w", &caps["to_move"]);
            assert_eq!("KQkq", &caps["castles"]);
            assert_eq!("-", &caps["en_passant"]);
            assert_eq!("0", &caps["half_moves"]);
            assert_eq!("1", &caps["move_number"]);


            let example_string = "r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R b - - 3 17";
            let gotc = FEN_REGEX.captures(example_string).unwrap();
            assert_eq!("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R", &gotc["piece_placements"]);
            assert_eq!("b", &gotc["to_move"]);
            assert_eq!("-", &gotc["castles"]);
            assert_eq!("-", &gotc["en_passant"]);
            assert_eq!("3", &gotc["half_moves"]);
            assert_eq!("17", &gotc["move_number"]);

            // Invalid FENs
            let mut invalid_caps = FEN_REGEX.captures("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 1");
            assert!(invalid_caps.is_none());
            invalid_caps = FEN_REGEX.captures("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R z - - 3 17");
            assert!(invalid_caps.is_none());
        }

        #[test]
        #[should_panic]
        fn test_invalid_input() {
            FEN_REGEX.captures("r3r1k1/pp3pbp/1Bp1b1p1/8/2BP4/Q1n2N2/P4PPP/3R1K1R/ b - - 0 18").unwrap();
        }
    }

    mod fen_piece_placement_regex {
        use super::*;

        #[test]
        fn test_valid_input() {
            FEN_PIECE_PLACEMENT_REGEX.captures("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
            FEN_PIECE_PLACEMENT_REGEX.captures("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R").unwrap();
        }

        #[test]
        fn test_invalid_input() {
            assert!(FEN_PIECE_PLACEMENT_REGEX.captures("r3r1k1/pp3pbp/1Bp1b1p1/8/2BP4/Q1n2N2/P4PPP/3R1K1R/").is_none());
            assert!(FEN_PIECE_PLACEMENT_REGEX.captures("").is_none());
            assert!(FEN_PIECE_PLACEMENT_REGEX.captures("r/r").is_none());
        }
    }

    #[test]
    fn test_resolve_piece_code() {
        let piece1 = resolve_piece_code(0, 2, 'k');
        assert_eq!(((0, 2).into(), PieceColor::Dark, PieceType::King), piece1);

        let piece2 = resolve_piece_code(5, 7, 'P');
        assert_eq!(((5, 7).into(), PieceColor::Light, PieceType::Pawn), piece2);

        let piece3 = resolve_piece_code(7,0, 'n');
        assert_eq!(((7,0).into(), PieceColor::Dark, PieceType::Knight), piece3);

        let piece4 = resolve_piece_code(2,5, 'B');
        assert_eq!(((2,5).into(), PieceColor::Light, PieceType::Bishop), piece4);
    }

    mod fen {
        use super::*;

        #[test]
        fn test_from_str_string() {
            let fen = Fen::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
            assert_eq!(Fen {
                piece_placements: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".parse().unwrap(),
                light_to_move: true,
                castles: BoardCastleState {
                    light_king_side: true,
                    light_queen_side: true,
                    dark_king_side: true,
                    dark_queen_side: true
                },
                en_passant: None,
                half_moves: 0,
                move_number: 1,
            }, fen);
        }
    }

    mod fen_piece_placements {
        use super::*;

        #[test]
        fn test_from_str_valid_input() {
            let mut expected = FenPiecePlacements {
                pieces: Vec::new(),
            };
            // We have to implement the entire board manually. You can view the position here:
            // https://lichess.org/study/UZlSqSLA/Ku9M59je#31
            // Eighth row
            expected.pieces.push(((0, 7).into(), PieceColor::Dark, PieceType::Rook));
            expected.pieces.push(((4, 7).into(), PieceColor::Dark, PieceType::Rook));
            expected.pieces.push(((6, 7).into(), PieceColor::Dark, PieceType::King));
            // Seventh row
            expected.pieces.push(((0, 6).into(), PieceColor::Dark, PieceType::Pawn));
            expected.pieces.push(((1, 6).into(), PieceColor::Dark, PieceType::Pawn));
            expected.pieces.push(((5, 6).into(), PieceColor::Dark, PieceType::Pawn));
            expected.pieces.push(((6, 6).into(), PieceColor::Dark, PieceType::Bishop));
            expected.pieces.push(((7, 6).into(), PieceColor::Dark, PieceType::Pawn));
            // Sixth row
            expected.pieces.push(((1, 5).into(), PieceColor::Dark, PieceType::Queen));
            expected.pieces.push(((2, 5).into(), PieceColor::Dark, PieceType::Pawn));
            expected.pieces.push(((6, 5).into(), PieceColor::Dark, PieceType::Pawn));
            // Fifth row
            expected.pieces.push(((2, 4).into(), PieceColor::Light, PieceType::Bishop));
            // Fourth row
            expected.pieces.push(((2, 3).into(), PieceColor::Light, PieceType::Bishop));
            expected.pieces.push(((3, 3).into(), PieceColor::Light, PieceType::Pawn));
            expected.pieces.push(((6, 3).into(), PieceColor::Dark, PieceType::Bishop));
            // Third row
            expected.pieces.push(((0, 2).into(), PieceColor::Light, PieceType::Queen));
            expected.pieces.push(((2, 2).into(), PieceColor::Dark, PieceType::Knight));
            expected.pieces.push(((5, 2).into(), PieceColor::Light, PieceType::Knight));
            // Second row
            expected.pieces.push(((0, 1).into(), PieceColor::Light, PieceType::Pawn));
            expected.pieces.push(((5, 1).into(), PieceColor::Light, PieceType::Pawn));
            expected.pieces.push(((6, 1).into(), PieceColor::Light, PieceType::Pawn));
            expected.pieces.push(((7, 1).into(), PieceColor::Light, PieceType::Pawn));
            // First row
            expected.pieces.push(((3, 0).into(), PieceColor::Light, PieceType::Rook));
            expected.pieces.push(((5, 0).into(), PieceColor::Light, PieceType::King));
            expected.pieces.push(((7, 0).into(), PieceColor::Light, PieceType::Rook));

            assert_eq!(expected, "r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R".parse().unwrap());
        }

        #[test]
        fn test_from_str_invalid_input() {
            assert_eq!(Err(FenError::InvalidFenPiecePlacementString), FenPiecePlacements::from_str(""));
            assert_eq!(Err(FenError::InvalidFenPiecePlacementString), FenPiecePlacements::from_str("asdfjknasdfjkndasjknf"));
            assert_eq!(Err(FenError::InvalidFenPiecePlacementString), FenPiecePlacements::from_str("0/0/0/0/0/0/0/0"));
            assert_eq!(Err(FenError::InvalidFenPiecePlacementString), FenPiecePlacements::from_str("a/b/c/d/e"));
            assert_eq!(Err(FenError::InvalidFenPiecePlacementString), FenPiecePlacements::from_str("aaaaaa/AAAA4A/b6B"));
        }

        #[test]
        fn test_into_iterator() {
            let p1 = FenPiecePlacements::from_str("2k5/8/8/8/8/4R3/8/2K5").unwrap();
            let p2 = p1.clone();

            let mut p1_iter = p1.into_iter();
            let mut p2_iter = p2.pieces.into_iter();

            // They should return the same values
            for _ in 0..=2 {
                let p1_iter_next = p1_iter.next();
                assert_eq!(p1_iter_next, p2_iter.next());
                assert_ne!(None, p1_iter_next); // Implies p2_iter_next != None
            }

            assert_eq!(None, p1_iter.next());
            assert_eq!(None, p2_iter.next());
            assert_eq!(p1_iter.next(), p2_iter.next());
        }
    }

    #[test]
    fn test_resolve_board_castle_state(){
        let castle_state = resolve_board_castle_state(String::from("KQkq"));
        let expected = BoardCastleState{
            light_king_side: true,
            light_queen_side: true,
            dark_king_side: true,
            dark_queen_side: true
        };
        assert_eq!(castle_state, expected);

        let castle_state2 = resolve_board_castle_state(String::from("Kq"));
        let expected2 = BoardCastleState{
            light_king_side: true,
            light_queen_side: false,
            dark_king_side: false,
            dark_queen_side: true
        };
        assert_eq!(castle_state2, expected2);

        let castle_state3 = resolve_board_castle_state(String::from("Qq"));
        let expected3 = BoardCastleState{
            light_king_side: false,
            light_queen_side: true,
            dark_king_side: false,
            dark_queen_side: true
        };
        assert_eq!(castle_state3, expected3);

        let castle_state4 = resolve_board_castle_state(String::from("-"));
        let expected4 = BoardCastleState{
            light_king_side: false,
            light_queen_side: false,
            dark_king_side: false,
            dark_queen_side: false
        };
        assert_eq!(castle_state4, expected4);
    }
}