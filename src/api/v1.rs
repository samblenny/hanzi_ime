// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

//! Export v1 api names. The point of using re-exports is to allow for splitting
//! the crate implementation into relatively small modules that are easy to
//! refactor without breaking the public api.

// Re-export names from modules into the v1 namespace
pub use crate::dialects::translate_zh_hans;

/// These tests aim to cover all names exported in the v1 api
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_translate_zh_hans() {
        assert_eq!(&translate_zh_hans(&"1"), &"1");
    }
}
