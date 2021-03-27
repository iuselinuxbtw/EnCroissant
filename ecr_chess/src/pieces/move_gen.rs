use crate::coordinate::Coordinate;
use crate::pieces::{PieceType, BoardPiece, PieceColor};
use std::convert::TryFrom;

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

/// This macro is used to break the loop of calculating positions when the current square is
/// occupied. Breaks instantly when the square is occupied by a piece of the own color, but not
/// when the piece is the  opponents color in which case it adds the position and then breaks.
/// If it is neither of those it just adds it to the result.
macro_rules! check_square{
    ($x: expr, $y: expr, $team_color: expr, $result: expr) => {
        let possible_square =  coordinate_check($x as &usize, $y as &usize, $team_color);
        // If the square is occupied by a piece
        if possible_square.1{
            // Check if it is our own piece.
            if possible_square.0.is_none() {
                // If it is, we shouldn't add that square to the array since we can't capture our own pieces.
                break;
            }
            // It's safe to use unwrap here since we already know that it's not None.
            // If it is the enemies piece we can capture it.
            // TODO: Change the type to capture if there is an enemy piece on this square.
            $result.push(possible_square.0.unwrap());
            break;
        }
        $result.push(possible_square.0.unwrap());
    }
}


/// Returns the possible linear moves of a piece with the given coordinates as a vector of
/// coordinates, also checks whether there are pieces in the way. An example of a piece that moves
/// this way is a rook.
fn linear_moves(start: Coordinate) -> Vec<Coordinate> {
    // First we initialize a new vector, which we later return
    let mut result: Vec<Coordinate> = Vec::new();

    // Bind the given coordinates to variables because we obviously can
    let from_x = start.get_x() as usize;
    let from_y = start.get_y()  as usize;

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
        // Add the square if we can move there.
        check_square!(&x, &from_y, &team_color, result);
    }
    // Do the same for y coordinates.
    for y in potential_y{
        // Add the square if we can move there.
        check_square!(&from_x, &y, &team_color, result);
    }
    return result;
}

/// Returns the possible diagonal moves of a piece with the given coordinates as a vector of
/// coordinates, also checks whether there are pieces in the way. An example of a piece that moves
/// this way is a bishop.
fn diagonal_moves(start: &Coordinate , team_color: &PieceColor) -> Vec<Coordinate> {
    // Create a vector that will be returned at the end.
    let mut result:Vec<Coordinate> = Vec::new();

    // Bind the starting coordinates to variables
    let from_x = start.get_x() as usize;
    let from_y = start.get_y() as usize;


    // Explore the moves in all directions.
    result.append(&mut explore_diagonal_moves(Directions::NW, &from_x, &from_y, team_color));
    result.append(&mut explore_diagonal_moves(Directions::NE, &from_x, &from_y, team_color));
    result.append(&mut explore_diagonal_moves(Directions::SE, &from_x, &from_y, team_color));
    result.append(&mut explore_diagonal_moves(Directions::SW, &from_x, &from_y, team_color));

    result
}

/// Utility enum for the function explore_diagonal_moves. Assigns each direction a on the chess
/// board a cardinal direction. You can look up the cardinal directions
/// [here](https://en.wikipedia.org/wiki/Cardinal_direction).
enum Directions {
    NW, // upper-left
    NE, // upper-right
    SE, // down-right
    SW,  // down-left
}

/// This function returns all moves into a particular direction
fn explore_diagonal_moves(direction: Directions, from_x: &usize, from_y:&usize, team_color: &PieceColor) -> Vec<Coordinate>{
    let mut x = from_x.clone() as i32;
    let mut y = from_y.clone() as i32;
    let mut result = Vec::new();
    match direction {
        // upper-left
        Directions::NW => {
            while x>0 && y<7{
                // First we modify the coordinates so we can calculate the new possible coordinates
                x-=1;
                y+=1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square!(&usize::try_from(x).unwrap(),&usize::try_from(y).unwrap(), &team_color, result);
            }
        }
        // upper-right
        Directions::NE => {
            while x<7 && y<7 {
                x+=1;
                y+=1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square!(&usize::try_from(x).unwrap(),&usize::try_from(y).unwrap(), &team_color, result);
            }
        }
        // down-right
        Directions::SE => {
            while x<7 && y>0{
                x+=1;
                y-=1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square!(&usize::try_from(x).unwrap(),&usize::try_from(y).unwrap(), &team_color, result);
            }
        }
        // down-left
        Directions::SW => {
            while x>0 && y>0{
                x-=1;
                y-=1;
                // We can safely unwrap here since the variables can't be less than 0
                check_square!(&usize::try_from(x).unwrap(),&usize::try_from(y).unwrap(), &team_color, result);
            }
        }
    }
    result
}

/// Calculates a square and then just calls square_check()
fn coordinate_check(x:&usize, y:&usize , team_color: &PieceColor) -> (Option<Coordinate>, bool) {
    let square = (*x as u8,*y as u8).into();
    square_check(&square, team_color)
}

/// Checks if a square is occupied and if it is checks whether it can be captured
/// or if it is the teams own piece, in which case it returns None. The bool returns true if the
/// square is occupied.
fn square_check(square:&Coordinate, team_color: &PieceColor) -> (Option<Coordinate>, bool) {
    // We need to check if the square is occupied to avoid calculating non-reachable coordinates
    let square_occupied = piece_is_on_square(*square);
    if !square_occupied.is_none() {
        // Check whether it is our own piece.
        if &square_occupied.unwrap().color == team_color {
            return (None, true);
        }
        return (Some(*square), true)
    }
    (Some(*square), false)
}

// Returns the Piece a square is occupied by. If the square is not occupied it returns None
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
    #[test]
    fn test_explore_diagonal_moves(){
        // Calculate the moves in the North-east (upper-right) direction from 3,2(d3)
        let result = explore_diagonal_moves(Directions::NE, &3, &2, &PieceColor::Light);
        let expected: Vec<Coordinate> = vec![(4,3).into(),(5,4).into(),(6,5).into(),(7,6).into()];
        assert_eq!(result, expected);

        // Do the same for the North-west (upper-left) direction from h1
        let result2 = explore_diagonal_moves(Directions::NW, &7, &0, &PieceColor::Dark);
        let expected2: Vec<Coordinate> = vec![(6,1).into(),(5,2).into(),(4,3).into(),(3,4).into(),(2,5).into(), (1,6).into(),(0,7).into()];
        assert_eq!(result2, expected2);
    }

    #[test]
    fn test_diagonal_moves(){
        let result = diagonal_moves(&(4 ,3).into(),&PieceColor::Dark);
        let mut expected :Vec<Coordinate> = vec![
            // North-west (upper left)
            (3,4).into(),(2,5).into(),(1,6).into(),(0,7).into(),
            // North-east (upper right)
            (5,4).into(),(6,5).into(),(7,6).into(),
            // South-east (lower right)
            (5,2).into(),(6,1).into(),(7,0).into(),
            // South-west (lower left)
            (3,2).into(),(2,1).into(),(1,0).into(),
        ];
        assert_eq!(result, expected);
    }
}