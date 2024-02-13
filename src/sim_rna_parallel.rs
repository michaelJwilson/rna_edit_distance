use crate::sim_rna::simulate_rna;
use rayon::prelude::*;

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

        // Check that we got the right number of sequences
        assert_eq!(sequences.len(), 100);

        for sequence in sequences {
            // Check that each sequence has the right length
            assert_eq!(sequence.len(), 10);

            // Check that each sequence only contains valid RNA nucleotides
            for c in sequence.chars() {
                assert!(matches!(c, 'A' | 'C' | 'G' | 'U'));
            }
        }
    }
}
