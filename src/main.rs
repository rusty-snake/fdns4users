/*
 * Copyright © 2020 rusty-snake
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use std::env;
use std::process::Command;

const FDNS: &str = "/usr/bin/fdns";

fn main() {
    // Defense in Depth: set the effetive-UID to the real-UID
    unsafe {
        assert!(libc::seteuid(libc::getuid()) == 0);
    }

    // parse commandline args, skip arg0 and keep only '--whitelist=[A-Za-z0-9.-]*'
    let fdns_args = env::args()
        .skip(1)
        .filter(|arg| {
            arg.starts_with("--whitelist=")
                && arg[12..]
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        })
        .collect::<Vec<_>>();

    // set real, effective and saved UID and GID to root
    unsafe {
        assert!(libc::setresuid(0, 0, 0) == 0);
        assert!(libc::setresgid(0, 0, 0) == 0);
    }

    // start fdns
    Command::new(FDNS)
        .arg("--proxy-addr=127.70.74.68")
        .args(&fdns_args)
        .env_clear()
        .spawn()
        .expect("Failed to start fdns")
        .wait()
        .unwrap();
}
