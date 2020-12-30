pub mod tests_com {

    extern crate ewin;
    use ewin::model::*;

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn test_add() {
        assert_eq!(add(3, 2), 5);
    }
}
