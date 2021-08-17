use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }
}

struct Circle {
    radius: u32,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * (self.radius.pow(2) as f64)
    }
}
