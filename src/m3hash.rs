// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// This code includes an adaptation of the the murmur3 hash algorithm.
// The murmur3 public domain notice, as retrieved on August 3, 2020 from
// https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp,
// states:
// > MurmurHash3 was written by Austin Appleby, and is placed in the public
// > domain. The author hereby disclaims copyright to this source code.
//
#![forbid(unsafe_code)]

/// Compute Murmur3 hash function of the first limit codepoints of a grapheme
/// cluster string, using each char as a u32 block.
/// Returns: (murmur3 hash, how many bytes of key were hashed (e.g. key[..n]))
pub fn grapheme_cluster(gc: &str, seed: u32, limit: u32) -> (u32, usize) {
    let mut h = seed;
    let mut k;
    // Hash each character as its own u32 block
    let mut n = 0;
    let mut bytes_hashed = gc.len();
    for (i, c) in gc.char_indices() {
        if n >= limit {
            bytes_hashed = i;
            break;
        }
        k = c as u32;
        k = k.wrapping_mul(0xcc9e2d51);
        k = k.rotate_left(15);
        k = k.wrapping_mul(0x1b873593);
        h ^= k;
        h = h.rotate_left(13);
        h = h.wrapping_mul(5);
        h = h.wrapping_add(0xe6546b64);
        n += 1;
    }
    h ^= bytes_hashed as u32;
    // Finalize with avalanche
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    (h, bytes_hashed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_cluster_ascii_seed0_limit1() {
        let gc = &"test";
        let seed = 0;
        let limit = 1;
        // This is hashing just the 't' from "test"
        assert_eq!(grapheme_cluster(gc, seed, limit), (0x31099644, 1));
    }

    #[test]
    fn test_grapheme_cluster_ascii_seed1_limit1() {
        let gc = &"test";
        let seed = 1;
        let limit = 1;
        // This is hashing just the 't' from "test"
        assert_eq!(grapheme_cluster(gc, seed, limit), (0xD667FA27, 1));
    }

    #[test]
    fn test_grapheme_cluster_simple_emoji_limit1() {
        let gc = &"😸test";
        let seed = 0;
        let limit = 1;
        // This is hashing just the "😸" from "😸test"
        // Return of (_, 4) means 4 bytes were used for limit of 1 codepoint
        assert_eq!(grapheme_cluster(gc, seed, limit), (0x86E5DD9A, 4));
    }
}
