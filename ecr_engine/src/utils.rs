use std::cell::RefCell;
use std::rc::Rc;
use ecr_shared::coordinate::Coordinate;

/// Returns the supplied value wrapped inside a [`Rc`] that contains a [`RefCell`] with the value.
pub fn new_rc_refcell<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}

pub fn get_en_passant_actual(target_square: Coordinate) -> Coordinate{
    match target_square.get_y(){
        3 => (target_square.get_x(), 3).into(),
        4 => (target_square.get_x(), 5).into(),
        // This only happens when the given coordinate is invalid, so we're going to give the same coordinate back.
        _ => target_square
    }
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
}
