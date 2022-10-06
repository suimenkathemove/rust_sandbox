mod env_logger_;
mod tracing;

fn main() {
    env_logger_::main();
    tracing::main();
}
