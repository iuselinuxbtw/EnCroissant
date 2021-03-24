
use crate::coordinate::Coordinate;
use crate::pieces::{PieceType, BoardPiece, PieceColor};

/// Defines a move.
struct Move {
    from: Coordinate,
    to: Coordinate,
    piece_type: PieceType,
    //move_type: Vec<MoveType>,
}
enum MoveType {
    Check,
    Capture,
    Evasion,
    Book,
    Sacrifice,
    Promotion,
    Castle,
}

/// Returns the possible linear moves of a given coordinate as a vector of coordinates and also
/// checks whether there are pieces in the way.
fn linear_moves(start: Coordinate) -> Vec<Coordinate> {
    // First we initialize a new vector, which we later return
    let mut result: Vec<Coordinate> = Vec::new();

    // Bind the given coordinates to variables because we obviously can
    let from_x = start.get_x();
    let from_y = start.get_y();

    // Initialize two vectors to through the board coordinates.
    let mut potential_x:Vec<usize> = vec![0,1,2,3,4,5,6,7];
    let mut potential_y:Vec<usize> = potential_x.clone();

    // We need to remove the given coordinates from the vector, so that we can't output the start
    // coordinates as move coordinates.
    potential_x.remove(from_x as usize);
    potential_y.remove(from_y as usize);

    // TODO: We need the actual team color here.
    // The team color is necessary because we don't wanna capture our own pieces.
    let team_color = PieceColor::Light;

    // Iterate through the possible x coordinates.
    for x in potential_x{
        let square:Coordinate = (x as u8,*&from_y as u8).into();
        let square_checked = square_check(&square, &team_color);
        if square_checked.is_none(){
            break;
        }
        &result.push(square);
    }
    // Do the same for y coordinates.
    for y in potential_y{
        let square:Coordinate = (*&from_x as u8,y as u8).into();
        let square_checked = square_check(&square, &team_color);
        if square_checked.is_none(){
            break;
        }
        &result.push(square);
    }
    return result;
}

/// This function checks if a square is occupied and if it is, it checks whether it can be captured
/// or if it is the teams own piece, in which case it returns None.
fn square_check(square:&Coordinate, team_color: &PieceColor) -> Option<Coordinate> {
    // We need to check if the square is occupied to avoid calculating non-reachable coordinates
    let square_occupied = piece_is_on_square(*square);
    if !square_occupied.is_none() {
        // TODO: Error handling (In reality this unwrap should never fail but we all know that that's not how it works).
        // Check whether it is our own piece.
        if &square_occupied.unwrap().color == team_color {
            return None;
        }
    }
    Some(*square)
}

// Returns the Piece a square is occupied by.
fn piece_is_on_square(square: Coordinate) -> Option<BoardPiece> {
    // TODO: Get access to the board here.
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linear_moves(){
        let result = linear_moves((3,2).into());
        // Make a new Vector and fill it with all possible Coordinates
        let mut expected :Vec<Coordinate> = Vec::new();
        for x in 0..=7{
            &expected.push((x, 2).into());
        }
        for y in 0..=7{
            &expected.push((3, y).into());
        }
        // Remove the orininal position in the Vector
        expected.retain(|&x| x !=(3,2).into());
        assert_eq!(result, expected);
    }
}