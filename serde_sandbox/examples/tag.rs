use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "c")]
struct A {
    b: String,
}

fn main() {
    let a = A { b: "b".to_string() };
    println!("{}", serde_json::to_string(&a).unwrap());
}
