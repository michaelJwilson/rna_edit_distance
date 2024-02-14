use crate::sim_rna::simulate_rna;
use rayon::prelude::*;

/**
    Simulates the generation of `n` RNA sequences of a given `length` using multiple threads.

    This function uses rayon's parallel iterators to generate the RNA sequences concurrently.

    Arguments:
        * 'n' - The number of RNA sequences to generate.
        * 'length' - The length of each RNA sequence.

    Returns:
        A `Vec<String>` containing 'n' RNA sequences, each of length 'length'.

    Example:

    let sequences = simulate_rna_parallel(100, 50);

    assert_eq!(sequences.len(), 100);
    assert!(sequences.iter().all(|seq| seq.len() == 50));
*/
pub fn simulate_rna_parallel(n: usize, length: usize) -> Vec<String> {
    (0..n)
        .into_par_iter()
        .map(|_| simulate_rna(length))
        .collect()
}

#[cfg(test)]
mod test_simulate_rna_parallel {
    use super::*;

    #[test]
    fn test_simulate_rna_parallel() {
        let sequences: Vec<String> = simulate_rna_parallel(100, 10);

        assert_eq!(sequences.len(), 100);

        for sequence in sequences {
            assert_eq!(sequence.len(), 10);

            for c in sequence.chars() {
                assert!(matches!(c, 'A' | 'C' | 'G' | 'U'));
            }
        }
    }
}
