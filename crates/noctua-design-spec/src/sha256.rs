//! SHA-256, so a vendored drawing is provably the one that was fetched.
//!
//! # Why this is here and not a dependency
//!
//! `NOCTUA.md §2.3` asks for our own solution where one is reasonable, and this
//! one is: FIPS 180-4 is a page of arithmetic with no options, no configuration
//! and no ambiguity, and the test vectors below are the specification's own.
//! A crate would be a supply-chain edge for sixty lines of shifts.
//!
//! It hashes whole files held in memory rather than streaming, because the
//! largest thing it is ever pointed at is a 700-byte SVG.

/// The round constants: the first thirty-two bits of the fractional parts of
/// the cube roots of the first sixty-four primes.
const K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// The eight working variables, by the letters the specification gives them.
///
/// Named indices into one array rather than eight bindings: a round moves every
/// variable one place along, which is one rotation over a slice and was eight
/// assignments in a row that had to be read in the right order to be checked.
/// `d` has no name because nothing reads it except that rotation.
const A: usize = 0;
/// The second.
const B: usize = 1;
/// The third.
const C: usize = 2;
/// The fifth, which is the one the round adds its first sum to.
const E: usize = 4;
/// The sixth.
const F: usize = 5;
/// The seventh.
const G: usize = 6;
/// The eighth.
const H: usize = 7;

/// The initial state: the first thirty-two bits of the fractional parts of the
/// square roots of the first eight primes.
const INITIAL: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

/// The SHA-256 of `message`, lowercase hexadecimal, as `sha256sum` writes it.
///
/// # Panics
///
/// On a message whose length does not fit in a `u64`, which is not a message
/// that fitted in memory on any machine this runs on.
#[must_use]
pub fn hex(message: &[u8]) -> String {
    let mut state = INITIAL;

    // The padding: a one bit, zeros up to 56 bytes modulo 64, then the length
    // in BITS as a big-endian u64. `u64::try_from` cannot fail for any message
    // that fitted in memory, and saying so is cheaper than an `as` cast whose
    // behaviour a reader has to look up.
    let bits = u64::try_from(message.len())
        .expect("a message longer than u64::MAX bytes did not fit in memory")
        * 8;
    let mut padded = Vec::with_capacity(message.len() + 72);
    padded.extend_from_slice(message);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bits.to_be_bytes());

    for block in padded.chunks_exact(64) {
        let mut schedule = [0_u32; 64];
        for (index, word) in block.chunks_exact(4).enumerate() {
            schedule[index] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for index in 16..64 {
            let a = schedule[index - 15];
            let b = schedule[index - 2];
            let s0 = a.rotate_right(7) ^ a.rotate_right(18) ^ (a >> 3);
            let s1 = b.rotate_right(17) ^ b.rotate_right(19) ^ (b >> 10);
            schedule[index] = schedule[index - 16]
                .wrapping_add(s0)
                .wrapping_add(schedule[index - 7])
                .wrapping_add(s1);
        }

        let mut work = state;
        for index in 0..64 {
            let sum1 =
                work[E].rotate_right(6) ^ work[E].rotate_right(11) ^ work[E].rotate_right(25);
            let choose = (work[E] & work[F]) ^ (!work[E] & work[G]);
            let first = work[H]
                .wrapping_add(sum1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(schedule[index]);
            let sum0 =
                work[A].rotate_right(2) ^ work[A].rotate_right(13) ^ work[A].rotate_right(22);
            let majority = (work[A] & work[B]) ^ (work[A] & work[C]) ^ (work[B] & work[C]);

            // Every variable takes its neighbour's value, which is the rotation.
            // Then the two the round actually computes are written over the top:
            // `e` is the old `d` plus the first sum, and `a` is both sums.
            work.rotate_right(1);
            work[E] = work[E].wrapping_add(first);
            work[A] = first.wrapping_add(sum0.wrapping_add(majority));
        }

        for (slot, value) in state.iter_mut().zip(work) {
            *slot = slot.wrapping_add(value);
        }
    }

    let mut out = String::with_capacity(64);
    for word in state {
        for byte in word.to_be_bytes() {
            out.push(char::from_digit(u32::from(byte >> 4), 16).unwrap_or('0'));
            out.push(char::from_digit(u32::from(byte & 0x0f), 16).unwrap_or('0'));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::hex;

    /// FIPS 180-4's own vectors, plus the empty message.
    #[test]
    fn the_specification_vectors() {
        assert_eq!(
            hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    /// A message longer than one block, and one that lands exactly on the
    /// padding boundary — the two places a hand-written padding loop breaks.
    #[test]
    fn the_padding_boundaries() {
        assert_eq!(
            hex(&[b'a'; 55]),
            "9f4390f8d30c2dd92ec9f095b65e2b9ae9b0a925a5258e241c9f1e910f734318"
        );
        assert_eq!(
            hex(&[b'a'; 56]),
            "b35439a4ac6f0948b6d6f9e3c6af0f5f590ce20f1bde7090ef7970686ec6738a"
        );
        assert_eq!(
            hex(&[b'a'; 64]),
            "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"
        );
    }
}
