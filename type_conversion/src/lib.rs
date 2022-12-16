mod from_into;

// Option<&T> -> Option<T>
pub fn option_ref_to_option<T: Clone>(arg: Option<&T>) -> Option<T> {
    arg.map(|a| a.clone())
}
pub fn option_ref_to_option2<T: Clone>(arg: Option<&T>) -> Option<T> {
    let a = arg?;
    Some(a.clone())
}
