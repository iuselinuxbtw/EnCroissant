use std::cell::RefCell;
use std::rc::Rc;

/// Returns the supplied value wrapped inside a [`Rc`] that contains a [`RefCell`] with the value.
pub fn new_rc_refcell<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
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
