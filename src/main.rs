extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const TOTAL_RUNS: i32 = 1_000_000;
const TOTAL_BLOG_POSTS: i32 = 60;

const DURATIONS: [i32; 10] = [2, 3, 4, 6, 6, 6, 8, 8, 9, 10];

fn main() {
    let mut rng = thread_rng();
    let time_to_completion = Uniform::from(0..DURATIONS.len());

    let mut successes = 0;

    for _ in 0..TOTAL_RUNS {
        let mut current_duration = 0;

        for _ in 0..TOTAL_BLOG_POSTS {
            let random_index = time_to_completion.sample(&mut rng);
            current_duration += DURATIONS[random_index];
        }

        if current_duration <= 365 {
            successes += 1
        }
    }

    let p = f64::from(successes) / f64::from(TOTAL_RUNS) * f64::from(100);

    println!("Total Simulations: {}", TOTAL_RUNS);
    println!("Successes: {}", successes);
    println!("Probability of succeeding: {:.2}%", p);
}
