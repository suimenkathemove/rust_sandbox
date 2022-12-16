struct A(String);

struct B(String);

impl From<B> for A {
    fn from(b: B) -> Self {
        Self(b.0)
    }
}

impl Into<B> for A {
    fn into(self) -> B {
        B(self.0)
    }
}

fn main() {
    let a = A("a".to_string());
    let b: B = a.into();
    let a = A::from(b);
}
