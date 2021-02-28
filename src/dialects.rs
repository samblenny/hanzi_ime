// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

use crate::m3hash;

pub fn translate_zh_hans(pinyin_ascii: &str) -> &str {
    let _ = m3hash::grapheme_cluster(&"", 0, 1);
    pinyin_ascii
}
