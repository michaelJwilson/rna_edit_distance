use rna_edit_distance::{levenshtein_distance, calc_base_frac};
use std::env;

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

    let sequence = b"ACGTACGTACGT";
    let mut fraction: [f64; 4] = [0.0; 4];

    unsafe {
        calc_base_frac(sequence.as_ptr(), sequence.len(), fraction.as_mut_ptr());
    }

    println!(
	"Found a base fractions of {:?}",
	fraction
    );

    assert_eq!(fraction, [0.25, 0.25, 0.25, 0.25]);
}
