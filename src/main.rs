mod ga;
use clap::{Arg, Command};

#[derive(Debug)]
struct Algebra {
    positive: u8,
    negative: u8,
    zero: u8,
    size: u32,
}

impl Algebra {
    fn new(positive: u8, negative: u8, zero: u8) -> Self {
        // Convert to u32 before adding to prevent overflow
        let size = u32::from(positive) + u32::from(negative) + u32::from(zero);

        Algebra {
            positive,
            negative,
            zero,
            size,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("My CLI App")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Does awesome things with positive, negative and zero values.")
        .arg(
            Arg::new("positive")
                .short('p')
                .long("positive")
                .value_name("NUMBER")
                .help("Sets number of unit vector squaring to 1")
                .value_parser(clap::value_parser!(u8)),
        )
        .arg(
            Arg::new("negative")
                .short('n') // Changed from 'q' to 'n' for clarity
                .long("negative")
                .value_name("NUMBER")
                .help("Sets number of unit vectors squaring to -1")
                .value_parser(clap::value_parser!(u8)),
        )
        .arg(
            Arg::new("zero")
                .short('z')
                .long("zero")
                .value_name("NUMBER")
                .help("Sets number of unit vector squaring to 0")
                .value_parser(clap::value_parser!(u8)),
        )
        .arg(
            Arg::new("cayley")
                .long("cayley")
                .help("Specifies the Cayley table for the algebra")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let algebra = Algebra::new(
        matches.get_one::<u8>("positive").copied().unwrap_or(0),
        matches.get_one::<u8>("negative").copied().unwrap_or(0),
        matches.get_one::<u8>("zero").copied().unwrap_or(0),
    );

    println!("Positive: {}", algebra.positive);
    println!("Negative: {}", algebra.negative);
    println!("Zero: {}", algebra.zero);
    println!("Size: {}", algebra.size);
    println!("Show Cayley diagram: {}", matches.get_flag("cayley"));

    Ok(())
}
