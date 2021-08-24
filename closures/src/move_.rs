pub fn main() {
    let v = vec![1, 2, 3];

    let _closure = move || v;

    // error
    // println!("{:?}", v);
}
