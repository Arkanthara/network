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

pub fn detect_collision(input: Vec<f64>, t: f64) -> f64 {
    let mut collision: f64 = input.clone().into_iter().reduce(f64::max).unwrap();
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

pub fn time_retransmit(input: Vec<f64>, collision: f64, t: f64, collision_time: f64) -> f64 {
    for i in input {
        if collision + t > i && collision < i {
            return i + collision_time;
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

pub fn simulate_multiple_stations(n: i32, time: f64, lambda: f64, t: f64, collision_time: f64) {
    let mut current_time: f64 = 0.0;
    let mut collision: i32 = 0;
    let mut no_collision: i32 = 0;
    while current_time < time {
        let result: Vec<f64> = multiple_stations(lambda, n);
        let collision_trame: f64 = detect_collision(result.clone(), t);
        no_collision += count_non_collision_tram(result.clone(), collision_trame);
        collision += 2;
        current_time += time_retransmit(result.clone(), collision_trame, t, collision_time);
    }
}
