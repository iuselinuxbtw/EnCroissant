use regex::Regex;
use lazy_static::lazy_static;
use std::convert::TryFrom;
use thiserror::Error;
use std::num::ParseIntError;

lazy_static! {
    static ref FEN_REGEX: Regex = Regex::new(r#"^(?P<positions>((?:[rnbqkpRNBQKP1-8]{1,8}/){7})[rnbqkpRNBQKP1-8]{1,8})\s(?P<to_move>[b|w])\s(?P<castles>-|K?Q?k?q?)\s(?P<en_passant>-|[a-h][3|6])\s(?P<half_moves>\d+)\s(?P<move_number>\d+)$"#).unwrap();
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
    positions: String,
    light_to_move: bool,
    castles: Option<String>,
    en_passant: Option<String>,
    half_moves: usize,
    move_number: usize,
}

impl TryFrom<&str> for Fen{
    type Error = FenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let caps = match FEN_REGEX.captures(value){
                None => Err(FenError::InvalidFenString),
                Some(v) => Ok(v)
            }?;
        Ok(Fen {
            positions: String::from(&caps["positions"]),
            light_to_move: match &caps["to_move"]{
                "w" => true,
                "b" => false,
                _ => false
            },
            castles: match &caps["castles"]{
                "-" => None,
                v => Some(String::from(v)),
            },
            en_passant: match &caps["en_passant"]{
                "-"=> None,
                v=> Some(String::from(v)),
            },
            half_moves: (&caps["half_moves"]).parse()?,
            move_number: (&caps["move_number"]).parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_regex() {
        let caps = FEN_REGEX.captures("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", &caps["positions"]);
        assert_eq!("w", &caps["to_move"]);
        assert_eq!("KQkq", &caps["castles"]);
        assert_eq!("-", &caps["en_passant"]);
        assert_eq!("0", &caps["half_moves"]);
        assert_eq!("1", &caps["move_number"]);


        let example_string = "r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R b - - 3 17";
        let gotc = FEN_REGEX.captures(example_string).unwrap();
        assert_eq!("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R", &gotc["positions"]);
        assert_eq!("b", &gotc["to_move"]);
        assert_eq!("-", &gotc["castles"]);
        assert_eq!("-", &gotc["en_passant"]);
        assert_eq!("3", &gotc["half_moves"]);
        assert_eq!("17", &gotc["move_number"]);

        //Invalid FENs
        let mut  invalid_caps = FEN_REGEX.captures("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 1");
        assert!(invalid_caps.is_none());
        invalid_caps = FEN_REGEX.captures("r3r1k1/pp3pbp/1qp3p1/2B5/2BP2b1/Q1n2N2/P4PPP/3R1K1R z - - 3 17");
        assert!(invalid_caps.is_none());
    }

    #[test]
    fn test_try_from_string(){
        let fen = Fen::try_from(r#"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"#).unwrap();
        assert_eq!(Fen {
            positions: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
            light_to_move: true,
            castles: Some("KQkq".to_string()),
            en_passant: None,
            half_moves: 0,
            move_number: 1,
        }, fen);
    }
}