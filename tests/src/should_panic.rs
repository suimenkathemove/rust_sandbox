#[test]
#[should_panic]
fn test() {
    panic!();
}

#[test]
#[should_panic(expected = "foo")]
fn test_with_expected() {
    panic!("fooo");
}
