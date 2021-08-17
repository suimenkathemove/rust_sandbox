use std::fmt::{Debug, Display};

trait A {
    fn a1<T: Clone>(&self, t: &T);
    fn a2(&self, t: &impl Clone);

    fn b1<T: Clone + Copy, U: Debug + Display>(t: &T, u: &U);
    fn b2<T, U>(t: &T, u: &U)
    where
        T: Clone + Copy,
        U: Debug + Display;
}
