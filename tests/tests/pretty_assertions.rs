use pretty_assertions::assert_eq;

#[test]
#[should_panic]
fn test() {
    assert_eq!("abc", "bcd");
}
