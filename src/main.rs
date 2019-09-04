//
// Copyright (c) 2015 Rodolphe Breard
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

use libreauth::oath::TOTPBuilder;
use std::io;

fn get_secret_key() -> Option<String> {
    let mut key = String::new();
    let read_len = match io::stdin().read_line(&mut key) {
        Ok(l) => l,
        Err(_) => {
            return None;
        }
    };
    if read_len != 0 {
        Some(key.trim().to_string())
    } else {
        None
    }
}

fn main() {
    loop {
        let key = match get_secret_key() {
            Some(k) => k,
            None => {
                return;
            }
        };
        let code = TOTPBuilder::new()
            .base32_key(&key)
            .finalize()
            .unwrap()
            .generate();
        println!("{}", code);
    }
}
