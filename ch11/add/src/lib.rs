mod rectangle;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_test() {
        assert_eq!("hello", "hello");
    }

    use crate::rectangle::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{height: 7, width: 4};
        let smaller = Rectangle{height: 4, width:2};

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
}
