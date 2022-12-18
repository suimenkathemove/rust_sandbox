struct A(i32);

// B -> A
impl From<B> for A {
    fn from(b: B) -> Self {
        Self(b.0)
    }
}

struct B(i32);

fn main() {
    let b = B(0);
    let a: A = b.into();
}
