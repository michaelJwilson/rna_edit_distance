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