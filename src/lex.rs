// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![forbid(unsafe_code)]

use crate::autogen_hsk;
use crate::constants;
use crate::dialects;

// Data structure for tracking lexemes of query input and their meanings.
// TokenQueue is no_std, stack-only substitute for Vec<Token>. If TokenQueue
// were Vec<Token>, it would require heap allocation and linking std.

const TOKEN_QUEUE_SIZE: usize = constants::BUF_SIZE;
// Holds one Token.
#[derive(Copy, Clone)]
pub enum Token {
    CiOne(dialects::CiyuIndex),
    CiOpenChoice(dialects::CiyuIndex),
    MaybeChoice(char),
    Other(char),
    Skip,
}
// Holds queue of Tokens (append only)
pub struct TokenQueue {
    pub queue: [Token; TOKEN_QUEUE_SIZE],
    pub count: usize,
}
impl TokenQueue {
    // Initialize queue.
    pub fn new() -> TokenQueue {
        TokenQueue {
            queue: [Token::Skip; TOKEN_QUEUE_SIZE],
            count: 0,
        }
    }
    // Add Token to queue.
    pub fn push(&mut self, tk: Token) -> bool {
        if self.count < TOKEN_QUEUE_SIZE {
            self.queue[self.count] = tk;
            self.count += 1;
            true
        } else {
            // Error: Queue is full
            false
        }
    }
    // Iterate through tokens, resolve choices, render as strings.
    // Side-effect: render strings into buffer provided by Writer.
    // Possible surprising behavior:
    // - Value of CiOpenChoice depends on lookahead for MaybeChoice
    // - MaybeChoice gets consumed (skipped) if used to resolve choice
    pub fn render_and_write(&mut self, sink: &mut impl dialects::Writer) {
        let mut current = 0;
        let mut utf8_buf = [0u8; 4];
        while current < self.count {
            match self.queue[current] {
                // CiOne: This is an clear pinyin match for just one 词语
                Token::CiOne(ciyu_i) => {
                    sink.write(&autogen_hsk::CIYU[ciyu_i]);
                    // Look ahead for adjacent space that might be intended
                    // to prevent this ciyu from getting matched as part
                    // of the pinyin for another longer ciyu
                    if current + 1 < self.count {
                        if let Token::MaybeChoice(tk) = self.queue[current + 1] {
                            // Consume the space
                            if tk == ' ' {
                                self.queue[current + 1] = Token::Skip;
                            }
                        }
                    }
                }

                // CiOpenChoice: This is an ambiguous pinyin match for
                // a set of homphone 词语 that require further input to
                // resolve the choice between them
                Token::CiOpenChoice(ciyu_i) => {
                    let ciyu = &autogen_hsk::CIYU[ciyu_i];
                    // Look ahead for a possible MaybeChoice token to
                    // resolve the open choice
                    let mut choice_resolved = false;
                    for i in current..self.count {
                        if let Token::MaybeChoice(tk) = self.queue[i] {
                            match dialects::expand_choice_and_write(ciyu, tk, sink) {
                                dialects::ExpandChoiceResult::WasChoice => {
                                    self.queue[i] = Token::Skip;
                                    choice_resolved = true;
                                    break;
                                }
                                dialects::ExpandChoiceResult::WasNotChoice => {}
                            }
                        }
                    }
                    if !choice_resolved {
                        // TODO: use enum variant instead of '0' to indicate no MaybeChoice found
                        let _ = dialects::expand_choice_and_write(ciyu, '0', sink);
                    }
                }

                // MaybeChoice: This is for spaces or numbers that should
                // be passed through unchanged because they were not
                // consumed by the lookahead from a CiOne or CiOpenChoice
                Token::MaybeChoice(tk) => sink.write(tk.encode_utf8(&mut utf8_buf)),

                // Other: This is for stuff like "UPPER CASE" or emoji
                Token::Other(tk) => sink.write(tk.encode_utf8(&mut utf8_buf)),

                // Skip: This marks spaces and numbers consumed by the
                // lookahead for CiOne or CiOpenChoice, and it fills empty
                // region of buffer
                Token::Skip => {}
            }
            current += 1;
        } // end while
    } // end render_and_write()
} // end impl TokenQueue
