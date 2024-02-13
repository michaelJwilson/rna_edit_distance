use rand::{prelude::ThreadRng, thread_rng, Rng};

pub fn simulate_rna(length: usize) -> String {
    /*
    Generates a random RNA sequence of the given length.

    # Arguments
        * `length` - The length of the RNA sequence to generate.

    # Returns
        * `String` RNA sequence. Each character is one of 'A', 'C', 'G', or 'U'.
    */
    let nucleotides: [char; 4] = ['A', 'C', 'G', 'U'];

    let mut sequence: String = String::with_capacity(length);
    let mut rng: ThreadRng = thread_rng();

    for _ in 0..length {
        let nucleotide: usize = rng.gen_range(0..nucleotides.len());
        sequence.push(nucleotides[nucleotide]);
    }

    sequence
}

#[cfg(test)]
mod test_simulate_rna {
    use super::*;

    #[test]
    fn test_simulate_rna() {
        let sequence: String = simulate_rna(10);

        assert_eq!(sequence.len(), 10);

        for c in sequence.chars() {
            assert!(matches!(c, 'A' | 'C' | 'G' | 'U'));
        }
    }
}
