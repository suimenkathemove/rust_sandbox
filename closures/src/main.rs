mod cacher;
mod capture_env;
mod move_;

fn main() {
    cacher::main();
    capture_env::main();
    move_::main();
}
