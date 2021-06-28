use ecr_shared::coordinate::Coordinate;
use std::cell::RefCell;
use std::rc::Rc;

/// Returns the supplied value wrapped inside a [`Rc`] that contains a [`RefCell`] with the value.
pub fn new_rc_refcell<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}

pub fn get_en_passant_actual(target_square: Coordinate) -> Coordinate {
    match target_square.get_y() {
        3 => (target_square.get_x(), 3).into(),
        4 => (target_square.get_x(), 5).into(),
        // This only happens when the given coordinate is invalid, so we're going to give the same coordinate back.
        _ => target_square,
    }
}

// Returns all Squares on a default board as Coordinates
pub fn get_all_squares() -> Vec<Coordinate> {
    let mut result = vec![];
    for x in 0..=7 {
        for y in 0..=7 {
            result.push((x, y).into());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::any;

    use super::*;

    fn get_type_name<T>(_: &T) -> String {
        format!("{}", any::type_name::<T>())
    }

    #[test]
    fn test_new_rc_refcell() {
        let r = new_rc_refcell(String::from("Test"));
        assert_eq!(
            "alloc::rc::Rc<core::cell::RefCell<alloc::string::String>>",
            get_type_name(&r)
        );
    }

    #[test]
    fn test_get_all_squares() {
        let all_squares: Vec<Coordinate> = get_all_squares();
        assert_eq!(64, all_squares.len());
        assert_eq!(Coordinate { y: 1, x: 0 }, all_squares[1]);
        assert_eq!(Coordinate { y: 7, x: 7 }, all_squares[63]);
    }
}
