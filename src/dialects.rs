// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

use crate::autogen_hsk;
use crate::constants;
use crate::lex;
use crate::m3hash;

pub fn translate_zh_hans(pinyin_ascii: &str) -> &str {
    let _ = m3hash::grapheme_cluster(&"", 0, 1);
    pinyin_ascii
}

// Static word list arrays generated by vocab precompute ruby script
// CiyuIndex is type for phrases listed in autogen_hsk::CIYU array
pub type CiyuIndex = usize;

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

// Utf8Str adds character boundary metadata to &str to help with safely slicing
// substrings. "Safely" means avoid panic from requesting slice with byte range
// not aligned on encoded Unicode character boundaries.
struct Utf8Str<'a> {
    str_slice: &'a str,
    char_start_list: [usize; constants::BUF_SIZE],
    char_end_list: [usize; constants::BUF_SIZE],
    char_count: usize,
}
impl<'a> Utf8Str<'a> {
    pub fn new(str_slice: &'a str) -> Utf8Str {
        // Find start (inclusive lower bound) and end (exclusive upper bound) byte
        // index of each UTF-8 character in string slice
        let mut char_start_list: [usize; constants::BUF_SIZE] = [0; constants::BUF_SIZE];
        let mut char_end_list: [usize; constants::BUF_SIZE] = [0; constants::BUF_SIZE];
        let mut char_count = 0;
        for i in 1..str_slice.len() + 1 {
            if str_slice.is_char_boundary(i) {
                if char_count + 1 < constants::BUF_SIZE {
                    char_start_list[char_count + 1] = i;
                }
                if char_count < constants::BUF_SIZE {
                    char_end_list[char_count] = i;
                    char_count += 1;
                }
            }
        }
        Utf8Str {
            str_slice,
            char_start_list: char_start_list,
            char_end_list: char_end_list,
            char_count,
        }
    }

    // Slice a substring using character range (not bytes!).
    // Using get(start..end) instead of [start..end] avoids possible panic.
    // This follows start..end range semantics (upper bound exclusive).
    pub fn char_slice(&self, start: usize, end: usize) -> Option<&str> {
        // Subtle point: implicit test for end > 0
        if start < end && end <= constants::BUF_SIZE {
            let start_b = self.char_start_list[start];
            // Must not allow end==0 here. For usize, (0 - 1) will panic.
            let end_b = self.char_end_list[end - 1];
            self.str_slice.get(start_b..end_b)
        } else {
            None
        }
    }
}

// Murmur3 hash function; Unicode ordinal value of each char is a u32 block.
// Credits: Derived from MurmurHash3.cpp (public domain) by Austin Appleby.
// Returns: u32 hash
pub fn murmur3(key: &str, seed: u32) -> u32 {
    let mut h = seed;
    let mut k;
    // Hash each character as its own u32 block
    for c in key.chars() {
        k = c as u32;
        k = k.wrapping_mul(0xcc9e2d51);
        k = k.rotate_left(15);
        k = k.wrapping_mul(0x1b873593);
        h ^= k;
        h = h.rotate_left(13);
        h = h.wrapping_mul(5);
        h = h.wrapping_add(0xe6546b64);
    }
    h ^= key.bytes().count() as u32;
    // Finalize with avalanche
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^ (h >> 16)
}

// Find longest 词语 match in start..end character window of query buffer.
// Side-effect: None.
// Return: (index in 词语 array for match, end boundary character in query for match)
fn longest_match(query: &Utf8Str, start: usize, mut end: usize) -> Option<(CiyuIndex, usize)> {
    end = min(query.char_count, end);
    // Subtle point: implicit test for end > 0
    while end > start {
        if let Some(query_slice) = query.char_slice(start, end) {
            let key = murmur3(&query_slice, autogen_hsk::MURMUR3_SEED);
            if let Ok(ciyu) = autogen_hsk::PINYIN.binary_search(&key) {
                return Some((ciyu, end));
            }
        }
        // Must not allow end==0 here. For usize, (0 - 1) will panic.
        end -= 1;
    }
    return None;
}

// Render 词语 multi-matches as resolved choice or prompt for choice.
// Side-effect: render strings into buffer provided by Writer.
// Return: Was the maybe_choice token used to resolve a choice?
pub enum ExpandChoiceResult {
    WasChoice,
    WasNotChoice,
}
pub fn expand_choice_and_write(
    ciyu: &str,
    maybe_choice: char,
    sink: &mut impl Writer,
) -> ExpandChoiceResult {
    let n = ciyu.split("\t").count();
    if n == 1 {
        // If this ever happens, there's a bug. Log and recover.
        sink.trace(901);
        sink.write(ciyu);
        return ExpandChoiceResult::WasNotChoice;
    }
    // Try to pick a choice (return immediately if number out of range)
    let pick = match maybe_choice {
        ' ' => 1, // Spacebar picks default option (label=1)
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9, // TODO: Fix. This only works for <=9 choices
        _ => 0,
    };
    if pick > 0 {
        for (i, choice) in ciyu.split("\t").enumerate() {
            if i + 1 == pick {
                sink.write(choice);
                return ExpandChoiceResult::WasChoice;
            }
        }
        // Out of range for possible choice, so return without sink.write() to
        // prevent duplicate choice prompting
        return ExpandChoiceResult::WasNotChoice;
    }
    // Show all choices
    sink.write(&" (");
    for (i, choice) in ciyu.split("\t").enumerate() {
        sink.write(match i {
            0 => &"1",
            1 => &"2",
            2 => &"3",
            3 => &"4",
            4 => &"5",
            5 => &"6",
            6 => &"7",
            7 => &"8",
            _ => &"9",
        });
        sink.write(choice);
        if i + 1 < n {
            sink.write(&" ");
        }
    }
    sink.write(&") ");
    return ExpandChoiceResult::WasNotChoice;
}

// Search for 词语 matches in substrings of query.
// Side-effect: Push tokens into queue.
fn search(
    query: &Utf8Str,
    queue: &mut lex::TokenQueue,
    mut start: usize,
    end: usize,
    sink: &mut impl Writer,
) {
    while start < end {
        // Limit window size to length of longest phrase in pinyin array
        let window_end = min(start + autogen_hsk::PINYIN_SIZE_MAX, end);
        if let Some((ciyu_i, match_end)) = longest_match(query, start, window_end) {
            // Got Match: push match, continue search in remainder of query
            if autogen_hsk::CIYU[ciyu_i].contains("\t") {
                queue.push(lex::Token::CiOpenChoice(ciyu_i));
            } else {
                queue.push(lex::Token::CiOne(ciyu_i));
            }
            start = match_end;
        } else {
            // No match... push one character, continue search in remainder of query
            if let Some(s) = query.char_slice(start, start + 1) {
                // TODO: Better solution than silently ignoring possible full queue
                let _ = match s {
                    // Space and digit characters may be intended to resolve a
                    // choice of homophone 词语 from an earlier CiOpenChoice
                    // token. Spaces may separate the pinyin from a CiOne token
                    // so the pinyin does not get consumed as the prefix to a
                    // longer 词语. Spaces and digits may also be intended to
                    // pass through as ASCII.
                    " " => queue.push(lex::Token::MaybeChoice(' ')),
                    "1" => queue.push(lex::Token::MaybeChoice('1')),
                    "2" => queue.push(lex::Token::MaybeChoice('2')),
                    "3" => queue.push(lex::Token::MaybeChoice('3')),
                    "4" => queue.push(lex::Token::MaybeChoice('4')),
                    "5" => queue.push(lex::Token::MaybeChoice('5')),
                    "6" => queue.push(lex::Token::MaybeChoice('6')),
                    "7" => queue.push(lex::Token::MaybeChoice('7')),
                    "8" => queue.push(lex::Token::MaybeChoice('8')),
                    "9" => queue.push(lex::Token::MaybeChoice('9')),
                    _ => {
                        if let Some(c) = s.chars().nth(0) {
                            // This covers stuff like "UPPER CASE" and emoji
                            queue.push(lex::Token::Other(c))
                        } else {
                            // Reaching this branch is a bug. For nth(0) to
                            // return None, s would have to be "" when
                            // s.chars() gets called. The `if let Some(s)` and
                            // `while start < end` above should not allow that
                            // to happen.
                            sink.trace(902);
                            false
                        }
                    }
                };
            }
            start += 1;
        }
    }
}

// Look up 词语 for search query (pinyin keys are ASCII, but inbox is UTF-8).
// Side-effect: renders utf8 result string into buffer provided by Writer.
pub fn look_up(query_bytes: &str, sink: &mut impl Writer) {
    let query = Utf8Str::new(query_bytes);
    let mut queue = lex::TokenQueue::new();
    let start = 0;
    let end = query.char_count;
    search(&query, &mut queue, start, end, sink);
    queue.render_and_write(sink);
}

// Writer decouples query response formatting from stream IO implementation details.
pub trait Writer {
    fn write(&mut self, message: &str);
    fn trace(&mut self, trace_code: i32);
    fn to_s(&self) -> &str;
}

// BufWriter is a Writer for string slices backed by stack allocated [u8].
pub struct BufWriter {
    buf: [u8; constants::BUF_SIZE],
    buf_pos: usize,
}
impl BufWriter {
    // Return empty buffer ready for use.
    pub fn new() -> BufWriter {
        BufWriter {
            buf: [0; constants::BUF_SIZE],
            buf_pos: 0,
        }
    }
    // Truncate buffer position back to 0 bytes.
    pub fn rewind(&mut self) {
        self.buf_pos = 0;
    }
}
impl Writer for BufWriter {
    // Append message to buffer
    fn write(&mut self, message: &str) {
        for b in message.bytes() {
            // TODO: better strategy for overflow (vs. silently drop extra)
            if self.buf_pos < self.buf.len() {
                self.buf[self.buf_pos] = b;
                self.buf_pos += 1;
            }
        }
    }

    // Ignore traces
    fn trace(&mut self, _: i32) {}

    // Return string slice of buffer contents.
    fn to_s(&self) -> &str {
        match core::str::from_utf8(&self.buf[0..self.buf_pos]) {
            Ok(s) => &s,
            Err(_) => &"", // TODO: handle mal-formed utf8 strings better
        }
    }
}

// Look up query, write results to sink.
// This is for calling as a library function from rust.
// Returns: string slice of results backed by sink.
pub fn query<'a>(qry: &str, sink: &'a mut impl Writer) -> &'a str {
    look_up(&qry, sink);
    sink.to_s()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_query() {
        assert_eq!("", query(&"", &mut BufWriter::new()));
    }

    #[test]
    fn max_query() {
        let buf_max = ['A' as u8; constants::BUF_SIZE];
        let qry_max = core::str::from_utf8(&buf_max).unwrap();
        // This should be passed through unchanged as ASCII
        assert_eq!(qry_max, query(qry_max, &mut BufWriter::new()));
    }

    #[test]
    fn max_query_plus_1_truncate() {
        let buf_max = ['A' as u8; constants::BUF_SIZE];
        let qry_max = core::str::from_utf8(&buf_max).unwrap();
        let buf_1_too_big = ['A' as u8; constants::BUF_SIZE + 1];
        let qry_1_too_big = core::str::from_utf8(&buf_1_too_big).unwrap();
        // This should truncate the query
        assert_eq!(qry_max, query(qry_1_too_big, &mut BufWriter::new()));
    }

    #[test]
    fn choice_xiang1() {
        assert_eq!("想", query(&"xiang1", &mut BufWriter::new()));
    }

    #[test]
    fn zhang3chang2() {
        assert!(query(&"zhang", &mut BufWriter::new()).contains("长"));
        assert!(query(&"chang", &mut BufWriter::new()).contains("长"));
    }

    #[test]
    fn query_all_pinyin_search_keys_verify_ciyu() {
        let test_data = &autogen_hsk::PINYIN_CIYU_TEST_DATA;
        for (normalized_pinyin, ciyu) in test_data.iter() {
            assert!(query(normalized_pinyin, &mut BufWriter::new()).contains(ciyu));
        }
    }

    #[test]
    fn choosing_ciyu_with_numbers_and_spaces() {
        assert!(query(&"xiang", &mut BufWriter::new()).contains("(1想"));
        assert!(query(&"xiang", &mut BufWriter::new()).contains("2向"));
        assert_eq!(query(&"xiang ", &mut BufWriter::new()), "想");
        assert!(query(&" xiang", &mut BufWriter::new()).starts_with(" "));
        assert!(query(&" xiang", &mut BufWriter::new()).contains("(1想"));
        assert_eq!(query(&"xiang1", &mut BufWriter::new()), "想");
        assert_eq!(query(&"xiang2", &mut BufWriter::new()), "向");
        assert!(query(&"xianghe", &mut BufWriter::new()).contains("(1想"));
        assert!(query(&"xianghe", &mut BufWriter::new()).contains("2向"));
        assert!(query(&"xianghe", &mut BufWriter::new()).contains("(1喝"));
        assert!(query(&"xianghe", &mut BufWriter::new()).contains("2和"));
        assert!(query(&"xiang he", &mut BufWriter::new()).starts_with("想"));
        assert!(query(&"xiang he", &mut BufWriter::new()).contains("(1喝"));
        assert!(query(&"xiang1he", &mut BufWriter::new()).starts_with("想"));
        assert!(query(&"xiang1he", &mut BufWriter::new()).contains("(1喝"));
        assert!(query(&"xianghe1", &mut BufWriter::new()).starts_with("想"));
        assert!(query(&"xianghe1", &mut BufWriter::new()).contains("(1喝"));
        assert!(query(&"xianghe ", &mut BufWriter::new()).starts_with("想"));
        assert!(query(&"xianghe ", &mut BufWriter::new()).contains("(1喝"));
        assert_eq!(query(&"xianghe 1", &mut BufWriter::new()), "想喝");
        assert_eq!(query(&"xianghe11", &mut BufWriter::new()), "想喝");
        assert_eq!(query(&"xiang he1", &mut BufWriter::new()), "想喝");
        assert_eq!(query(&"xiang he ", &mut BufWriter::new()), "想喝");
        assert_eq!(query(&"xianghe 2", &mut BufWriter::new()), "想和");
    }

    #[test]
    fn query_chars_not_matched_should_pass_through() {
        assert_eq!(query(&"🐇✨", &mut BufWriter::new()), "🐇✨");
        assert_eq!(query(&"baiSEde🐇✨11", &mut BufWriter::new()), "白SE的🐇✨");
        assert_eq!(
            query(&"RABBIT SPARKLES 11", &mut BufWriter::new()),
            "RABBIT SPARKLES 11"
        );
        assert_eq!(query(&"XIANGHE", &mut BufWriter::new()), "XIANGHE");
    }

    #[test]
    fn matching_buffer_sizes() {
        let utf8s = super::Utf8Str::new(&"slice");
        let u_len_s = utf8s.char_start_list.len();
        let u_len_e = utf8s.char_end_list.len();
        let tq = super::lex::TokenQueue::new();
        let tq_len = tq.queue.len();
        let sink = super::BufWriter::new();
        let sink_buf_len = sink.buf.len();
        assert_eq!(constants::BUF_SIZE, u_len_s);
        assert_eq!(constants::BUF_SIZE, u_len_e);
        assert_eq!(constants::BUF_SIZE, tq_len);
        assert_eq!(constants::BUF_SIZE, sink_buf_len);
    }

    #[test]
    fn space_disambiguating_pinyin_prefix_is_consumed() {
        assert_eq!("昆虫", query(&"kunchong", &mut BufWriter::new()));
        assert_eq!("困冲", query(&"kun chong", &mut BufWriter::new()));
        assert_eq!("困冲", query(&"kun chong ", &mut BufWriter::new()));
        assert_eq!(
            "我想喝果汁",
            query(&"wo xiang he guozhi", &mut BufWriter::new())
        );
    }

    // This might fail some day as consequence of vocab data entry. As long as
    // this test continues to pass, using single digit choice picking protocol
    // is okay. Fail means time for fancier algorithm to resolve choices.
    #[test]
    fn longest_choice_has_nine_or_less_options() {
        assert!(autogen_hsk::CIYU_CHOICE_MAX <= 9);
    }

    // This might fail some day as a consequence of vocab data entry. In case
    // of failure due to hash collision, try changing the murmur3 seed in
    // vocab/autogen_hsk.rb.
    #[test]
    fn pinyin_murmur3_hashes_are_sorted_with_no_collisions() {
        let mut prev = autogen_hsk::PINYIN[0];
        for i in 1..autogen_hsk::PINYIN.len() {
            let curr = autogen_hsk::PINYIN[i];
            assert!(curr > prev);
            prev = curr;
        }
    }
}
