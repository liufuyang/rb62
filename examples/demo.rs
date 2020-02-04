extern crate rb62;

use rb62::RB62;

fn main() {
    let mut rb62 = RB62::new();
    let b62 = "6GGODyP2LIdbxIfYxy5UbN";
    let hex_as_u128 = rb62.get_integer(b62).unwrap();
    println!("Input b62 {}, output hex {:032x}", b62, hex_as_u128);

    let hex = "dbc3d5ebe344484da3e2448712a02213";
    let b62 = rb62.get_b62(hex).unwrap();
    println!("Input hex {}, output b62 {}", hex, b62);
}