use ecr_shared::coordinate::Coordinate;
use crate::board::{ThreatenedState, Board};
use ecr_shared::pieces::PieceColor;

/// Returns the four middle squares
pub(crate) fn get_middle_squares() -> Vec<Coordinate> {
    vec![
        Coordinate { y: 3, x: 3 },
        Coordinate { y: 4, x: 3 },
        Coordinate { y: 3, x: 4 },
        Coordinate { y: 4, x: 4 },
    ]
}

/// Returns the Threatened_states of the given coordinates
pub(crate) fn get_threatened_states(board: &Board, coords: Vec<Coordinate>) -> Vec<ThreatenedState> {
    let mut result = vec![];
    for coord in coords {
        result.push(board.get_threatened_state(coord));
    }
    result
}

/// Gets the threats of a particular team on given squares
pub(crate) fn get_threatened_score(states: Vec<ThreatenedState>, team: PieceColor) -> u64 {
    let mut result: u64 = 0;
    for state in states {
        result += state.get_by_team(team) as u64;
    }
    result
}