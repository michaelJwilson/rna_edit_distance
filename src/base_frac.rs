/*
Calls the 'calc_base_frac' function from the correspodning C library.

This function calculates the fraction of each nucleotide (A, C, G, T) in a given DNA sequence.

# Safety

This function is unsafe because it calls an `extern` function. The caller must ensure that the
'sequence' pointer is valid and that the `length` correctly represents the length of the sequence.

# Arguments
    * 'sequence' - A pointer to the DNA sequence.
    * 'length' - The length of the DNA sequence.
    * 'fraction' - A pointer to an array where the function will store the fraction of each nucleotide.

# Example
'''
let sequence = b"ACGTACGTACGT";
let mut fraction: [f64; 4] = [0.0; 4];

unsafe {
    calc_base_frac(sequence.as_ptr(), sequence.len(), fraction.as_mut_ptr());
}

assert_eq!(fraction, [0.25, 0.25, 0.25, 0.25]);
'''
*/
extern "C" {
    pub fn calc_base_frac(sequence: *const u8, length: usize, frac: *mut f64);
}

#[cfg(test)]
mod test_base_frac {
    use super::*;

    #[test]
    fn test_base_frac() {
        let sequence = b"ACGTACGTACGT";
        let mut result: [f64; 4] = [0.0; 4];

        unsafe {
            calc_base_frac(sequence.as_ptr(), sequence.len(), result.as_mut_ptr());
        }

        assert_eq!(result, [0.25, 0.25, 0.25, 0.25]);
    }
}
