// Option<&T> -> Option<T>
fn option_ref_to_option<T: Clone>(arg: Option<&T>) -> Option<T> {
    arg.map(|a| a.clone())
}
fn option_ref_to_option2<T: Clone>(arg: Option<&T>) -> Option<T> {
    let a = arg?;
    Some(a.clone())
}

fn main() {}
