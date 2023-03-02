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

    #[test]
    fn intersection() {
        let set1 = HashSet::from([0, 1]);
        let set2 = HashSet::from([1, 2]);
        let set3 = set1.intersection(&set2).map(|&i| i).collect();
        assert_eq!(HashSet::from([1]), set3);
    }
}
