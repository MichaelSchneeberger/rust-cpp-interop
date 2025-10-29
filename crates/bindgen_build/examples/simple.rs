use bindgen_build::{compress, decompress};

fn main() {
    let input = include_str!("../futurama-quotes.txt").as_bytes();

    let compressed = compress(input);
    let decompressed = decompress(&compressed[..]);

    assert_eq!(input, &decompressed[..]);
}
