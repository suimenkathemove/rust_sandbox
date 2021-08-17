pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    pub fn x_abs(&self) -> i32 {
        self.x.abs()
    }
}
