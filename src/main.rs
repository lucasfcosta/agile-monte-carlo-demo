extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const TOTAL_ROLLS: i32 = 1_000_000;

fn main() {
    let mut rng = thread_rng();
    let die = Uniform::from(1..7);

    let mut sevens = 0;
    for _ in 0..TOTAL_ROLLS {
        let first_die = die.sample(&mut rng);
        let second_die = die.sample(&mut rng);
        if first_die + second_die == 7 {
            sevens += 1
        }
    }

    let p: f64 = f64::from(sevens) / f64::from(TOTAL_ROLLS) * f64::from(100);

    println!("Total Rolls: {}", TOTAL_ROLLS);
    println!("Total Sevens: {}", sevens);
    println!("Probability of rolling a seven: {:.2}%", p);
}
