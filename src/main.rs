use rna_edit_distance::levenshtein_distance;
use std::env;

/// Entry point for the RNA edit distance calculator.
///
/// This program calculates the Levenshtein distance between two RNA sequences.
/// The Levenshtein distance is a measure of the difference between two sequences,
/// defined as the minimum number of edits (insertions, deletions, or substitutions)
/// needed to transform one sequence into the other.
///
/// The program expects two command-line arguments: the two RNA sequences to compare.
/// If the correct number of arguments is not provided, the program will print a usage
/// message and exit with a status code of 1.
///
/// If the calculation is successful, the program prints the Levenshtein distance
/// between the two sequences. If an error occurs (for example, if either sequence
/// contains an invalid character), the program prints an error message.
///
/// # Usage
///
/// ```bash
/// rna_edit_distance first_string second_string
/// ```
///
/// # Example
///
/// ```bash
/// rna_edit_distance AGCT AGTC
/// ```
///
/// This will output: "The Levenshtein distance between 'AGCT' and 'AGTC' is 2"
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: rna_edit_distance first_string second_string");

        std::process::exit(1);
    }

    let s1: &String = &args[1];
    let s2: &String = &args[2];

    let distance: Result<u64, &str> = levenshtein_distance(s1, s2);

    if let Ok(value) = distance {
        println!(
            "The Levenshtein distance between '{}' and '{}' is {}",
            s1, s2, value
        );
    } else if let Err(err) = distance {
        eprintln!("Oops, an error was found: {}", err);
    }
}
