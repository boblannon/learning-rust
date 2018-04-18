//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain calculations more
//! convenient.


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
///
/// # Panics
///
/// Scenarios where function could panic!
///
/// # Errors
///
/// if function returns a Result, describe errors that might occur and be returned
///
/// # Safety
///
/// if the function is unsafe, explain why. also explain the invariants that the function expects
/// callers to uphold
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
