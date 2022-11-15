pub fn main() {
    let foo = option_env!("FOO").expect("FOO is not set");
    println!("FOO: {}", foo);
}
