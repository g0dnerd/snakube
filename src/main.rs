use std::time::Instant;

use clap::Parser;
use snakube::{search, AttemptParams};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    size: usize,
    input: Vec<u8>,

    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let verbose = args.verbose;

    // Fallback to IRL cube if no input is given
    let size = if args.input.is_empty() { 4 } else { args.size };
    let mut input = if args.input.is_empty() {
        vec![
            1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 3, 2, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1,
            1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 3, 1, 1, 2, 1, 2,
        ]
    } else {
        args.input
    };
    input.reverse();

    validate_input(size, &input)?;

    let mut params = AttemptParams::new(&input);

    let now = Instant::now();
    let res = search::search(&mut params, size, verbose, 1);
    let elapsed = now.elapsed();

    if let Some(r) = res {
        for (dir, amt, pos) in r {
            println!("{} {} to {}", dir, amt, pos);
        }
        println!("Solution found in {:.2?}", elapsed);
    } else {
        println!("No solution found.");
    }
    Ok(())
}
fn validate_input(size: usize, input: &[u8]) -> anyhow::Result<()> {
    // Elements need to sum up to size^3 - 1 since our starting coordinate is implied.
    let input_sum = input.iter().sum::<u8>() + 1;
    if input_sum != size.pow(3) as u8 {
        anyhow::bail!(format!(
            "Invalid input sum: expected {}, got {}.",
            size.pow(3),
            input_sum
        ))
    }

    // Elements e for a cube of size n have to be 0 < e < n
    for i in input {
        if i >= &(size as u8) {
            anyhow::bail!(format!("Input element {} is too large", i))
        }
    }

    Ok(())
}