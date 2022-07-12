use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "あいうえお";

    assert_eq!(s.len(), 15);

    assert_eq!(s.graphemes(true).count(), 5);
}
