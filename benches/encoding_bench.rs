#![feature(test)]

extern crate test;
use test::Bencher;
use rust3270::encoding::{encode_ascii_to, decode_to_ascii, Encoding};

fn sample_ascii_string() -> String {
    // 10,000 ASCII characters
    (0x20u8..=0x7Eu8).cycle().take(10_000).map(|b| b as char).collect()
}

#[bench]
fn bench_encode_ascii_to_cp037(b: &mut Bencher) {
    let encoding = Encoding::CP037;
    let input = sample_ascii_string();
    b.iter(|| {
        let _result: Vec<u8> = encode_ascii_to(input.chars(), &encoding).collect();
    });
}

#[bench]
fn bench_decode_ascii_from_cp037(b: &mut Bencher) {
    let encoding = Encoding::CP037;
    let input = sample_ascii_string();
    let encoded: Vec<u8> = encode_ascii_to(input.chars(), &encoding).collect();
    b.iter(|| {
        let _result: Vec<char> = decode_to_ascii(encoded.iter().copied(), &encoding).collect();
    });
}
