// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]
#![forbid(unsafe_code)]

mod api;
mod dialects;
mod m3hash;

// Export v1 api names. The point of using re-exports is to allow for splitting
// the crate implementation into relatively small modules that are easy to
// refactor without breaking the public api.
pub use api::v1::*;

/// These are integration tests aimed at ensuring stability of the api.
#[cfg(test)]
mod tests {
    use crate::api::v1::*;
    use crate::m3hash;

    #[test]
    fn test_translate_zh_hans_one() {
        assert_eq!(&translate_zh_hans(&"1"), &"1");
    }

    #[test]
    fn test_hash_one() {
        let seed = 0;
        let limit = 1;
        assert_eq!(m3hash::grapheme_cluster(&"1", seed, limit), (1527182858, 1));
    }
}
