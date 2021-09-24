extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const TOTAL_RUNS: i32 = 1_000_000;
const TOTAL_STORIES: i32 = 500;

const TEN_DAY_THROUGHPUTS: [i32; 10] = [1, 2, 0, 1, 1, 2, 3, 1, 2, 1];

fn main() {
    let mut rng = thread_rng();
    let throughput = Uniform::from(0..TEN_DAY_THROUGHPUTS.len());

    let mut successes = 0;

    for _ in 0..TOTAL_RUNS {
        let mut stories_completed = 0;

        for _ in 0..366 {
            let random_index = throughput.sample(&mut rng);
            stories_completed += TEN_DAY_THROUGHPUTS[random_index];
        }

        if stories_completed > TOTAL_STORIES {
            successes += 1
        }
    }

    let p = f64::from(successes) / f64::from(TOTAL_RUNS) * f64::from(100);

    println!("Total Simulations: {}", TOTAL_RUNS);
    println!("Successes: {}", successes);
    println!("Probability of succeeding: {:.2}%", p);
}
