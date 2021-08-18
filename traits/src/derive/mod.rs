mod clone;
mod copy;
mod debug;
mod eq;
mod ord;
mod partial_eq;
mod partial_ord;

pub fn main() {
    clone::main();
    copy::main();
    debug::main();
    eq::main();
    ord::main();
    partial_eq::main();
    partial_ord::main();
}
