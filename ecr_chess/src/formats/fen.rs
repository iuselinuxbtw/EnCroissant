//! Contains an implementation for the Forsyth-Edwards Notation (FEN). More information about it can
//! be found in [chess programming wiki](https://www.chessprogramming.org/Forsyth-Edwards_Notation).


use std::convert::TryFrom;
use std::num::ParseIntError;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::coordinate::{char_to_x_coordinate, Coordinate};
use crate::pieces::{PieceColor, PieceType};

lazy_static! {
    static ref FEN_REGEX: Regex = Regex::new(r#"^(?P<piece_placements>((?:[rnbqkpRNBQKP1-8]{1,8}/){7})[rnbqkpRNBQKP1-8]{1,8})\s(?P<to_move>[b|w])\s(?P<castles>-|K?Q?k?q?)\s(?P<en_passant>-|[a-h][3|6])\s(?P<half_moves>\d+)\s(?P<move_number>\d+)$"#).unwrap();
}

#[derive(Debug, Error)]
pub enum FenError {
    #[error("invalid FEN string")]
    InvalidFenString,
    #[error("cannot parse as int: {0}")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fen {
    piece_placements: FenPiecePlacements,
    light_to_move: bool,
    castles: Option<String>,
    en_passant: Option<Coordinate>,
    half_moves: usize,
    move_number: usize,
}


impl TryFrom<&str> for Fen {
    type Error = FenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let caps = match FEN_REGEX.captures(value) {
            None => Err(FenError::InvalidFenString),
            Some(v) => Ok(v)
        }?;
        Ok(Fen {
            piece_placements: (&caps["piece_placements"]).into(),
            light_to_move: match &caps["to_move"] {
                "w" => true,
                "b" => false,
                _ => false
            },
            castles: match &caps["castles"] {
                "-" => None,
                v => Some(String::from(v)),
            },
            en_passant: match &caps["en_passant"] {
                "-" => None,
                v => Some({
                    let coordinates: Vec<char> = v.chars().collect();
                    // We can unwrap the parsed number since we already checked that it is valid
                    // with regex.
                    (char_to_x_coordinate(coordinates[0]), coordinates[1] as u8).into()
                }),
            },
            half_moves: (&caps["half_moves"]).parse()?,
            move_number: (&caps["move_number"]).parse()?,
        })
    }
}

/// Stores all the pieces with their corresponding colors and coordinates.
#[derive(Debug, PartialEq, Clone)]
pub struct FenPiecePlacements {
    pieces: Vec<(Coordinate, PieceColor, PieceType)>
}

/// Used to parse the FEN positions string into actual chess pieces with positions.
impl From<&str> for FenPiecePlacements {
    fn from(value: &str) -> Self {
        let as_chars: Vec<char> = value.chars().collect();
        let mut result: Vec<(Coordinate, PieceColor, PieceType)> = Vec::new();
        let rows:Vec<&str> = value.split("/").collect();
        for i in 0..rows.len() {
            let mut x: usize = 0;
            let row = &rows[i];
            let chars: Vec<char> = row.chars().collect();
            for c in chars{
                if c.is_ascii_digit() {
                    // If we get a number we have to skip that amount of squares. We have to subtract 1
                    // since we already add one per loop.
                    // We already checked this is a ascii digit, so we can safely unwrap the parsed
                    // value.
                    x += c.to_string().parse::<usize>().unwrap();
                } else {
                    // Since FEN piece placement starts from the top of the board, we have to
                    // subtract the index (y coordinate) from 7 to also start from the top.
                    result.push(resolve_piececode(x, 7 - i, c));
                    x+=1;
                }
            }
        };
        FenPiecePlacements {
            pieces: result
        }
    }
}

/// Used to resolve a Piece from coordinates and a fen piececode.
fn resolve_piececode(x: usize, y: usize, code: char) -> (Coordinate, PieceColor, PieceType) {
    let coordinates: Coordinate = (x as u8, y as u8).into();
    // By default dark but if its uppercase it's light
    let mut color: PieceColor = PieceColor::Dark;
    if code.is_uppercase() {
        color = PieceColor::Light;
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_regex() {
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

        //Invalid FENs
        let mut invalid_caps = FEN_REGEX.captures("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 1");
        assert!(invalid_caps.is_none());
        invalid_caps = FEN_REGEX.captures("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R z - - 3 17");
        assert!(invalid_caps.is_none());
    }

    #[test]
    fn test_try_from_string() {
        let fen = Fen::try_from(r#"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"#).unwrap();
        assert_eq!(Fen {
            piece_placements: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".into(),
            light_to_move: true,
            castles: Some("KQkq".to_string()),
            en_passant: None,
            half_moves: 0,
            move_number: 1,
        }, fen);
    }

    #[test]
    fn test_resolve_piececode() {
        let piece1 = resolve_piececode(0,2,'k');
        assert_eq!(((0,2).into(), PieceColor::Dark, PieceType::King), piece1);
        let piece2 = resolve_piececode(5,7,'P');
        assert_eq!(((5,7).into(), PieceColor::Light, PieceType::Pawn),piece2);
    }

    #[test]
    fn test_from_fen_piece_placements(){
        let positions:FenPiecePlacements = "r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R".into();
        let mut expected = FenPiecePlacements {
            pieces: Vec::new(),
        };
        // Now we have to create the pieces manually
        // Eighth row
        expected.pieces.push(((0,7).into(),PieceColor::Dark, PieceType::Rook));
        expected.pieces.push(((4,7).into(),PieceColor::Dark, PieceType::Rook));
        expected.pieces.push(((6,7).into(),PieceColor::Dark, PieceType::King));
        // Seventh row
        expected.pieces.push(((0,6).into(),PieceColor::Dark, PieceType::Pawn));
        expected.pieces.push(((1,6).into(),PieceColor::Dark, PieceType::Pawn));
        expected.pieces.push(((5,6).into(),PieceColor::Dark, PieceType::Pawn));
        expected.pieces.push(((6,6).into(),PieceColor::Dark, PieceType::Bishop));
        expected.pieces.push(((7,6).into(),PieceColor::Dark, PieceType::Pawn));
        // Sixth row
        expected.pieces.push(((1,5).into(),PieceColor::Dark, PieceType::Queen));
        expected.pieces.push(((2,5).into(),PieceColor::Dark, PieceType::Pawn));
        expected.pieces.push(((6,5).into(),PieceColor::Dark, PieceType::Pawn));
        // Fifth row
        expected.pieces.push(((2,4).into(),PieceColor::Light, PieceType::Bishop));
        // Fourth row
        expected.pieces.push(((2,3).into(),PieceColor::Light, PieceType::Bishop));
        expected.pieces.push(((3,3).into(),PieceColor::Light, PieceType::Pawn));
        expected.pieces.push(((6,3).into(),PieceColor::Dark, PieceType::Bishop));
        // Third row
        expected.pieces.push(((0,2).into(),PieceColor::Light, PieceType::Queen));
        expected.pieces.push(((2,2).into(),PieceColor::Dark, PieceType::Knight));
        expected.pieces.push(((5,2).into(),PieceColor::Light, PieceType::Knight));
        // Second row
        expected.pieces.push(((0,1).into(),PieceColor::Light, PieceType::Pawn));
        expected.pieces.push(((5,1).into(),PieceColor::Light, PieceType::Pawn));
        expected.pieces.push(((6,1).into(),PieceColor::Light, PieceType::Pawn));
        expected.pieces.push(((7,1).into(),PieceColor::Light, PieceType::Pawn));
        expected.pieces.push(((0,1).into(),PieceColor::Light, PieceType::Pawn));
        // First row
        expected.pieces.push(((3,0).into(),PieceColor::Light, PieceType::Rook));
        expected.pieces.push(((5,0).into(),PieceColor::Light, PieceType::King));
        expected.pieces.push(((7,0).into(),PieceColor::Light, PieceType::Rook));

        assert_eq!(expected, positions);
    }
}