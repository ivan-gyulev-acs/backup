#![feature(generic_const_exprs)]
mod vector;
mod line;
use vector::Vector;
use line::Line;

fn main() {
    let a = vector![4.0, -9.0, 36.0];
    let _ = a.rotate(2.0, a);
}
