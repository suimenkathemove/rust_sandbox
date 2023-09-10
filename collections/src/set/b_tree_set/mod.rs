#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn main() {
        let mut set = BTreeSet::<i32>::new();
        set.insert(0);
        set.insert(1);
        dbg!(set);
    }
}
