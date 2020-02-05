use rb62::RB62;

fn main() {
    let rb62 = RB62::new();

    let b62 = "6GGODyP2LIdbxIfYxy5UbN";
    let hex_as_u128 = rb62.get_integer(b62).unwrap();
    let hex = format!("{:032x}", hex_as_u128);
    println!("Input b62 {}, output hex {}", b62, hex);

    let hex = "dbc3d5ebe344484da3e2448712a02213";
    let b62 = rb62.get_b62(hex).unwrap();
    println!("Input hex {}, output b62 {}", hex, b62);
}