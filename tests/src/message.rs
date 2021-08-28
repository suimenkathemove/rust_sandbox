fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[test]
fn greeting_contains_name() {
    let name = "John Doe";
    let greeting = greet(name);
    // 必須引数の後に渡される値はformat!マクロに渡される
    assert!(greeting.contains(name), "greeting is `{}`", greeting);
}
