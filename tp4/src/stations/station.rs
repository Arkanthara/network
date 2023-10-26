//extern crate rand;

use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Exp;

pub fn station_transmit(lambda: f64) -> f64 {
    let exp = Exp::new(lambda).unwrap();
    return exp.sample(&mut rand::thread_rng());
}

pub fn multiple_stations(lambda: f64, n: i32) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for i in 0..n {
        result.push(station_transmit(lambda));
    }
    result
}

pub fn detect_collision(input: Vec<f64>, t: f64, collision_time: f64) -> f64 {
    let mut collision: f64 = input.iter().reduce(f64::max).unwrap();
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            match input[i] < input[j] {
                true => {
                    if input[i] + t > input[j] {
                        if collision > input[i] {
                            collision = input[i];
                        }
                    }
                }
                false => {
                    if input[i] < input[j] + t {
                        if collision > input[j] {
                            collision = input[j];
                        }
                    }
                }
            }
        }
    }
    return collision;
}

pub fn count_non_collision_tram(input: Vec<f64>, collision: f64) -> i32 {
    let mut result: i32 = 0;
    let mut i = 0;
    while input[i] != collision {
        result += 1;
        i += 1;
    }
    result
}
