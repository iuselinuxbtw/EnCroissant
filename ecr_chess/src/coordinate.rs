/// A `Coordinate` represents a square on the chess board.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    /// Returns a new instance of `Coordinate` with the supplied x and y coordinates set.
    pub fn new(x: u8, y: u8) -> Coordinate {
        Coordinate {
            x,
            y,
        }
    }

    /// Returns the x coordinate.
    pub fn get_x(&self) -> u8 {
        self.x
    }

    /// Returns the y coordinate.
    pub fn get_y(&self) -> u8 {
        self.y
    }

    /// Returns the x coordinate as a char.
    pub fn get_x_as_char(&self) -> char {
        match self.x {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => ' ',
        }
    }
}

impl From<(u8, u8)> for Coordinate {
    fn from(v: (u8, u8)) -> Self {
        Coordinate::new(v.0, v.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_coordinate() -> Coordinate {
        Coordinate {
            x: 3,
            y: 4,
        }
    }

    #[test]
    fn test_new() {
        assert_eq!(get_coordinate(), Coordinate::new(3, 4));
    }

    #[test]
    fn test_get_x() {
        assert_eq!(3, get_coordinate().get_x());
    }

    #[test]
    fn test_get_x_as_char() {
        assert_eq!('a', Coordinate::new(0, 0).get_x_as_char());
        assert_eq!('b', Coordinate::new(1, 0).get_x_as_char());
        assert_eq!('c', Coordinate::new(2, 0).get_x_as_char());
        assert_eq!('d', Coordinate::new(3, 0).get_x_as_char());
        assert_eq!('e', Coordinate::new(4, 0).get_x_as_char());
        assert_eq!('f', Coordinate::new(5, 0).get_x_as_char());
        assert_eq!('g', Coordinate::new(6, 0).get_x_as_char());
        assert_eq!('h', Coordinate::new(7, 0).get_x_as_char());
        assert_eq!(' ', Coordinate::new(8, 0).get_x_as_char());
        assert_eq!(' ', Coordinate::new(42, 0).get_x_as_char());
    }

    #[test]
    fn test_get_y() {
        assert_eq!(4, get_coordinate().get_y());
    }

    #[test]
    fn test_from_tuple() {
        assert_eq!(Coordinate::from((3, 4)), Coordinate::new(3, 4));
    }
}