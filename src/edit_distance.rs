use std::cmp::min;

const LEV_ERR_MSG: &'static str = "Input strings are not allowed to contain 'z'.";

/**
    Calculates the Levenshtein distance between two strings.

    The Levenshtein distance is a string metric for measuring the difference between two strings.
    Informally, the Levenshtein distance between two strings is the minimum number of single-character
    edits (insertions, deletions or substitutions) required to change one string into the other.

    This function returns an error if either string contains the character 'z'.

    Arguments:
    * `s1` - The first string to compare.
    * `s2` - The second string to compare.

    Returns:
    A `Result<u64, &'static str>`, where the `Ok` variant contains the Levenshtein distance between the two strings,
    and the `Err` variant contains an error message if either string contains the character 'z'.

    # Example
    '''
    let distance = levenshtein_distance("kitten", "sitting").unwrap();
    assert_eq!(distance, 3);
    '''
*/
pub fn levenshtein_distance(s1: &str, s2: &str) -> Result<u64, &'static str> {
    let len1: usize = s1.len();
    let len2: usize = s2.len();

    if s1.contains('z') || s2.contains('z') {
        return Err(LEV_ERR_MSG);
    }

    let mut matrix: Vec<Vec<u64>> = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i as u64;
    }

    for j in 0..=len2 {
        matrix[0][j] = j as u64;
    }

    for j in 1..=len2 {
        for i in 1..=len1 {
            // no operation required
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = 1 + min(
                    // substitution
                    matrix[i - 1][j - 1],
                    min(
                        // insertion
                        matrix[i][j - 1],
                        // deletion
                        matrix[i - 1][j],
                    ),
                );
            }
        }
    }

    Ok(matrix[len1][len2])
}

#[cfg(test)]
mod test_edit_distance {
    use super::*;
    use crate::sim_rna::simulate_rna;

    #[test]
    fn test_simulate_rna() {
        let sequence: String = simulate_rna(10);

        assert_eq!(sequence.len(), 10);

        for c in sequence.chars() {
            assert!(matches!(c, 'A' | 'C' | 'G' | 'U'));
        }
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting").unwrap(), 3);
        assert_eq!(levenshtein_distance("saturday", "sunday").unwrap(), 3);
        assert_eq!(levenshtein_distance("rust", "rust").unwrap(), 0);
        assert_eq!(levenshtein_distance("rust", "").unwrap(), 4);
        assert_eq!(levenshtein_distance("", "").unwrap(), 0);
    }

    #[test]
    fn test_error_on_input_z() {
        match levenshtein_distance("kitten", "sittingz") {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => assert_eq!(e, "Input strings are not allowed to contain 'z'."),
        }
    }

    #[test]
    fn test_levenshtein_distance_with_simulated_rna() {
        let rna1: &str = &simulate_rna(100);
        let rna2: &str = &simulate_rna(100);

        // Since the sequences are random, we can't know the exact distance,
        // but we know it should be between 0 (if they happen to be identical)
        // and 100 (if they are completely different).
        let distance: u64 = levenshtein_distance(rna1, rna2).unwrap();

        assert!(distance <= 100);
    }
}
