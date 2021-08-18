mod debug;
mod eq;
mod partial_eq;
mod partial_ord;

pub fn main() {
    debug::main();
    partial_eq::main();
    eq::main();
    partial_ord::main();
}
