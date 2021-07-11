#![allow(dead_code)]

mod triangle;
mod rasterizer;
mod utility;

use triangle::*;
use rasterizer::*;

fn main() {
    let t = Triangle::new();
    let b1 = Buffer::COLOR;
    let b2 = Buffer::DEPTH;
    let b3 = b1 | b2;
    println!("Hello, world!");
}
