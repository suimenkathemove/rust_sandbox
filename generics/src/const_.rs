// 定数を受け取るジェネリック型
struct Buffer<const S: usize> {
    buf: [u8; S],
}

pub fn main() {
    let _buf = Buffer::<128> { buf: [0; 128] };
}
