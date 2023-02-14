#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn main() {
        let mut set = HashSet::<i32>::new();
        set.insert(0);
        set.insert(1);
        dbg!(set);
    }
}
