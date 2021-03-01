// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

use hanzi_ime;

// Minimal example of using hanzi_ime as library with std and CLI
fn main() {
    let queries = &[
        &"woxiangheguozhi",
        &"woxiang heguozhi",
        &"woxiang he guozhi",
        &"woxianheguozhi11",
    ];
    // Make a stack allocated string buffer with the Writer trait
    // that hanzi_ime::query() expects
    let mut sink = hanzi_ime::BufWriter::new();
    // Run the queries
    for q in queries.iter() {
        println!("\n{}\n{}", q, hanzi_ime::query(q, &mut sink));
        sink.rewind();
    }
}
