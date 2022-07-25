pub mod string {
    use unicode_segmentation::UnicodeSegmentation;

    fn len(val: &str) -> usize {
        val.graphemes(true).count()
    }

    pub fn min(val: &str, min_len: usize) -> bool {
        min_len <= len(val)
    }

    pub fn max(val: &str, max_len: usize) -> bool {
        len(val) <= max_len
    }
}

#[cfg(test)]
mod tests {
    use crate::validation;

    #[test]
    fn min() {
        let val = "✋";
        assert!(validation::string::min(&val, 1));
    }

    #[test]
    fn max() {
        let val = "✋";
        assert!(validation::string::max(&val, 1));
    }
}
