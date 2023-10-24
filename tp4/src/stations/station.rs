//extern crate rand;

use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Exp;

pub fn station_transmit(lambda: f64, n: i32, t: f64) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    let exp = Exp::new(lambda).unwrap();
    result.push(exp.sample(&mut rand::thread_rng()));
    for i in 1..n {
        result.push(result.last().unwrap() + t + exp.sample(&mut rand::thread_rng()));
    }
    result
}
