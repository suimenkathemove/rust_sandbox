//! string
//!
//! char型はどの文字でも1文字で4バイト消費するが、String型や&str型は文字による。例えば`a`は1バイト消費し、`あ`は3バイト消費する。

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "あいうえお";

    assert_eq!(s.len(), 15);

    assert_eq!(s.graphemes(true).count(), 5);
}
