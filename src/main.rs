#![feature(std_misc)]
extern crate rand;

use std::time::Duration;
use rand::{Rng, XorShiftRng};

fn fill_buffer() -> Vec<u8> {
    let mut fast_rng: XorShiftRng = rand::thread_rng().gen();
    let testsize = 1000000;

    let mut buf = vec![0; testsize];
    fast_rng.fill_bytes(&mut buf[..]);
    buf
}



fn main() {
    println!("took {} to fill buffer", Duration::span(|| {
        let _ = fill_buffer();
    }));
}
