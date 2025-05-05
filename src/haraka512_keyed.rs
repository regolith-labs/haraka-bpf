use crate::haraka512::{aes_mix4, truncstore}; // Reuse helpers
use crate::simd128::Simd128;
use arrayref::array_ref;

/// Computes the keyed Haraka-512 permutation.
///
/// 1. Reads the 64-byte input state and 64-byte key.
/// 2. Creates a copy of the original input state for the feed-forward step.
/// 3. XORs the key into the state.
/// 4. Applies `N_ROUNDS` of the Haraka permutation (AES rounds + Mix).
/// 5. XORs the permuted state with the original input state (feed-forward).
/// 6. Truncates the result to 32 bytes and writes it to `dst`.
#[inline(always)]
pub fn haraka512_keyed<const N_ROUNDS: usize>(
    dst: &mut [u8; 32],
    state: &[u8; 64],
    key: &[u8; 64],
) {
    // --- Load initial state and key ---
    let mut s0 = Simd128::read(array_ref![state, 0, 16]);
    let mut s1 = Simd128::read(array_ref![state, 16, 16]);
    let mut s2 = Simd128::read(array_ref![state, 32, 16]);
    let mut s3 = Simd128::read(array_ref![state, 48, 16]);

    let k0 = Simd128::read(array_ref![key, 0, 16]);
    let k1 = Simd128::read(array_ref![key, 16, 16]);
    let k2 = Simd128::read(array_ref![key, 32, 16]);
    let k3 = Simd128::read(array_ref![key, 48, 16]);

    // --- Keep original state for feed-forward ---
    let t0 = s0;
    let t1 = s1;
    let t2 = s2;
    let t3 = s3;

    // --- XOR key into state ---
    Simd128::pxor(&mut s0, &k0);
    Simd128::pxor(&mut s1, &k1);
    Simd128::pxor(&mut s2, &k2);
    Simd128::pxor(&mut s3, &k3);

    // --- Apply Haraka rounds ---
    for i in 0..N_ROUNDS {
        aes_mix4(&mut s0, &mut s1, &mut s2, &mut s3, 8 * i);
    }

    // --- Feed-forward ---
    Simd128::pxor(&mut s0, &t0);
    Simd128::pxor(&mut s1, &t1);
    Simd128::pxor(&mut s2, &t2);
    Simd128::pxor(&mut s3, &t3);

    // --- Truncate and store ---
    truncstore(dst, &s0, &s1, &s2, &s3);
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrayref::array_mut_ref; // Import the missing macro
    #[allow(unused_imports)]
    // TODO: Add test vectors for haraka512_keyed, potentially derived
    // from the VerusHash reference implementation or intermediate values.
    #[test]
    fn test_haraka512_keyed_dummy() {
        // Placeholder test - replace with actual vectors
        let mut dst = [0u8; 32];
        let state = [0xabu8; 64];
        let key = [0xcdu8; 64];
        haraka512_keyed::<5>(&mut dst, &state, &key);
        // Add assert_eq! with expected output once available
        // Example: assert_eq!(dst, [expected bytes...]);
    }

    // Example test structure if you can get intermediate values
    #[test]
    fn test_keyed_xor_step() {
        let state = [0x11u8; 64];
        let key = [0x22u8; 64];
        let expected_xor_state = [0x33u8; 64]; // 0x11 ^ 0x22 = 0x33

        let mut s0 = Simd128::read(array_ref![state, 0, 16]);
        let mut s1 = Simd128::read(array_ref![state, 16, 16]);
        let mut s2 = Simd128::read(array_ref![state, 32, 16]);
        let mut s3 = Simd128::read(array_ref![state, 48, 16]);

        let k0 = Simd128::read(array_ref![key, 0, 16]);
        let k1 = Simd128::read(array_ref![key, 16, 16]);
        let k2 = Simd128::read(array_ref![key, 32, 16]);
        let k3 = Simd128::read(array_ref![key, 48, 16]);

        Simd128::pxor(&mut s0, &k0);
        Simd128::pxor(&mut s1, &k1);
        Simd128::pxor(&mut s2, &k2);
        Simd128::pxor(&mut s3, &k3);

        let mut xor_state_result = [0u8; 64];
        s0.write(array_mut_ref![xor_state_result, 0, 16]); // Use the imported macro
        s1.write(array_mut_ref![xor_state_result, 16, 16]); // Use the imported macro
        s2.write(array_mut_ref![xor_state_result, 32, 16]); // Use the imported macro
        s3.write(array_mut_ref![xor_state_result, 48, 16]); // Use the imported macro

        assert_eq!(xor_state_result, expected_xor_state);
    }
}
