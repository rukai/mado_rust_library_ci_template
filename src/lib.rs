pub fn do_something() -> u32 {
    crate_that_i_break_sometimes_for_testing::do_something();
    13
}

mod tests {
    #[test]
    fn it_works() {
        let result = crate::do_something() + 2;
        assert_eq!(result, 15);
    }
}
