mod math;
mod compare;

use std::fs;
use std::env;

fn main() {
    let flags = compare::cmp(5, 5);
    println!("eq: {}, lt: {}, gt: {}", flags[0], flags[1], flags[2]);
}