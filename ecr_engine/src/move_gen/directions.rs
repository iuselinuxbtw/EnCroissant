/// Utility enum for the function explore_diagonal_moves. Assigns each diagonal direction a on the
/// chess board a cardinal direction. You can look up the cardinal directions
/// [here](https://en.wikipedia.org/wiki/Cardinal_direction).
pub(crate) enum DiagonalDirections {
    // upper-left
    NW,
    // upper-right
    NE,
    // down-right
    SE,
    // down-left
    SW,
}

/// Utility enum for the function explore_linear_moves. Assigns each linear direction a on the chess
/// board a cardinal direction. You can look up the cardinal directions
/// [here](https://en.wikipedia.org/wiki/Cardinal_direction).
pub(crate) enum LinearDirections {
    // up
    N,
    // right
    E,
    // down
    S,
    // left
    W,
}

/// This enum combines LinearDirections and DiagonalDirections. Useful for the explore_knight_moves.
/// The first direction always refers to the direction where the knight jumps further. These are
/// cardinal directions, which you can look up [here](https://en.wikipedia.org/wiki/Cardinal_direction).
pub(crate) enum KnightDirections {
    // First the linear directions.
    // left-then-up
    WN,
    // right-then-up
    EN,
    // right-then-down
    ES,
    // left-then-down
    WS,
    // And the diagonal ones as well.
    // up-then-left
    NW,
    // up-then-right
    NE,
    // down-then-right
    SE,
    // down-then-left
    SW,
}

/// This enum holds the combined directions of LinearDirections and DiagonalDirections. Used for
/// e.g. KingDirections
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Directions {
    // Linear Directions
    // up
    N,
    // right
    E,
    // down
    S,
    // left
    W,
    // Diagonal Directions
    // upper-left
    NW,
    // upper-right
    NE,
    // down-right
    SE,
    // down-left
    SW,
}

impl Directions{
    pub fn get_direction(x: i16, y: i16) -> Option<Directions>{
        match (x, y){
            // West and East
            (-8..=-1, 0) => Some(Directions::W),
            (1..=8, 0) => Some(Directions::E),
            // South and North
            (0, 1..=8) => Some(Directions::S),
            (0,-8..=-1) => Some(Directions::N),
            // South-east
            (1..=8, 1..=8) => Some(Directions::SE),
            // South-west
            (-8..=-1, 1..=8) => Some(Directions::SW),
            // North-east
            (1..=8, -8..=-1) => Some(Directions::NE),
            // North-west
            (-8..=-1, -8..=-1) => Some(Directions::NW),
            (_, _) => None
        }
    }
}