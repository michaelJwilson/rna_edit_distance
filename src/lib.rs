/*
`lib.rs`

This library provides functionality for simulating RNA sequences and calculating the
Levenshtein distance between two strings.

Modules:

* `edit_distance` - Contains functionality for calculating the Levenshtein distance
                    between two strings.
* `sim_rna` - Contains functionality for simulating a single RNA sequence.
* `sim_rna_parallel` - Contains functionality for simulating multiple RNA sequences
                        in parallel.

Public Functions:
* `levenshtein_distance` - Calculates the Levenshtein distance between two strings.
* `simulate_rna` - Simulates a single RNA sequence.
* `simulate_rna_parallel` - Simulates multiple RNA sequences in parallel.

Example:

use your_library::{levenshtein_distance, simulate_rna, simulate_rna_parallel};

let distance = levenshtein_distance("kitten", "sitting").unwrap();
assert_eq!(distance, 3);

let sequence = simulate_rna(50);
assert_eq!(sequence.len(), 50);

let sequences = simulate_rna_parallel(100, 50);
assert_eq!(sequences.len(), 100);
assert!(sequences.iter().all(|seq| seq.len() == 50));
*/
mod edit_distance;
pub use edit_distance::levenshtein_distance;

mod sim_rna;
pub use sim_rna::simulate_rna;

mod sim_rna_parallel;
pub use sim_rna_parallel::simulate_rna_parallel;

mod base_frac;
pub use base_frac::calc_base_frac;