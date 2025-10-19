struct A(i32);

struct B(i32);

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
    let a = A(0);
    let b: B = a.into();
    let a = A::from(b);
}

fn vec() {
    let a_vec = vec![A(0)];
    let b_vec = a_vec.into_iter().map(|a| a.into()).collect::<Vec<B>>();
}
