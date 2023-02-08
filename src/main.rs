use rand::prelude::*;
use std::io;
use std::time::Instant;

/// Returns if the given x,y point is inside of a circle of radius 1
fn in_circle(x: f64, y: f64) -> bool {
    (x.powf(2.0) + y.powf(2.0)).powf(0.5) <= 1.0
}

fn main() {
    println!("Hello, world!");

    // Get number of iterations to run for
    let num_iterations: u32 = loop {
        println!("Enter the number of iterations to use (or just press enter for default 1,000,000 iterations)");

        let mut requested_iterations = String::new();

        io::stdin()
            .read_line(&mut requested_iterations)
            .expect("Failed to read from stdin!");

        // If nothing entered, default
        if requested_iterations.trim().is_empty() {
            break 1_000_000;
        }

        match requested_iterations.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!("Calculating the value of pi over {num_iterations} iterations");

    let mut rng = thread_rng(); // Initialize RNG

    let mut num_hits = 0;

    let start_time = Instant::now();

    for _ in 0..num_iterations {
        let x = rng.gen();
        let y = rng.gen();
        // If x,y are truly iid, the proportion of points
        // inside the circle should be pi*r^2 / 1
        // here r = 0.5, so we have 0.25*pi
        // and can multiply by 4 to recover pi!
        if in_circle(x, y) {
            num_hits += 1;
        }
    }

    let pi_estimate = 4.0 * (num_hits as f64 / num_iterations as f64);

    println!("Estimated pi to be: {pi_estimate}");
    println!("Elapsed time: {:.2?}", start_time.elapsed());
}
