#![no_std]

mod constants;
mod haraka256;
mod haraka512;
mod haraka512_keyed; // Add new module
mod simd128;

pub fn haraka256<const N_ROUNDS: usize>(dst: &mut [u8; 32], src: &[u8; 32]) {
    haraka256::haraka256::<{ N_ROUNDS }>(dst, src)
}

pub fn haraka512<const N_ROUNDS: usize>(dst: &mut [u8; 32], src: &[u8; 64]) {
    haraka512::haraka512::<{ N_ROUNDS }>(dst, src)
}

/// Computes the keyed Haraka-512 permutation with N_ROUNDS rounds.
///
/// See `haraka512_keyed::haraka512_keyed` for details.
pub fn haraka512_keyed<const N_ROUNDS: usize>(
    dst: &mut [u8; 32],
    state: &[u8; 64],
    key: &[u8; 64],
) {
    haraka512_keyed::haraka512_keyed::<{ N_ROUNDS }>(dst, state, key)
}
