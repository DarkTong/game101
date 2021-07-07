mod triangle;
mod rasterizer;

use triangle::*;
use rasterizer::*;

fn main() {
    let t = Triangle::new();
    let b1 = Buffer::Color;
    let b2 = Buffer::Depth;
    let b3 = b1 | b2;
    println!("Hello, world!");
}
