mod clone;
mod debug;
mod eq;
mod ord;
mod partial_eq;
mod partial_ord;

pub fn main() {
    debug::main();
    partial_eq::main();
    clone::main();
    ord::main();
    eq::main();
    partial_ord::main();
}
