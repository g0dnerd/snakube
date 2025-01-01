use clap::Parser;
use snakube::{search, AttemptParams};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    size: usize,
    input: Vec<u8>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let input = if args.input.is_empty() {
        // &vec![2, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 2]
        &vec![
            1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 3, 2, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1,
            1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 3, 1, 1, 2, 1, 2,
        ]
    } else {
        &args.input
    };
    let size = if args.input.is_empty() { 4 } else { args.size };

    validate_input(size, input)?;

    let mut params = AttemptParams::new(input);
    if let Some(res) = search(&mut params, size, 1) {
        for (dir, amt, pos) in res {
            println!("{} {} to {}", dir, amt, pos);
        }
    } else {
        println!("No solution found.");
    }
    Ok(())
}
fn validate_input(size: usize, input: &[u8]) -> anyhow::Result<()> {
    if input.is_empty() {
        anyhow::bail!("No input specified")
    }
    let input_sum = input.iter().sum::<u8>() + 1;
    if input_sum != size.pow(3) as u8 {
        anyhow::bail!(format!(
            "Invalid input sum: expected {}, got {}.",
            size.pow(3),
            input_sum
        ))
    }
    for i in input {
        if i >= &(size as u8) {
            anyhow::bail!(format!("Input element {} is too large", i))
        }
    }

    Ok(())
}
