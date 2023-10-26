mod stations;

use crate::stations::station::*;

fn main() {
    println!("Hello, world!");
    let result: f64 = station_transmit(2.0, 10000, 1.0);
    println!("My Vec: {:?}", &result);
}
