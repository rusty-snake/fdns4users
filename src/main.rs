/*
 * Copyright © 2020,2021 rusty-snake
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
use std::process::{exit, Command};

/// The path to the fdns binary
const FDNS: &str = "/usr/bin/fdns";

/// The main function
///
/// Start [fdns](FDNS) as root with the arguments returned by [`parse_and_validate_args`].
fn main() {
    // Defense in Depth: set the effetive-UID to the real-UID
    unsafe {
        assert!(libc::seteuid(libc::getuid()) == 0);
    }

    let (proxy_addr, fdns_args) = parse_and_validate_args(&mut env::args().skip(1));

    // set real, effective and saved UID and GID to root
    unsafe {
        assert!(libc::setresuid(0, 0, 0) == 0);
        assert!(libc::setresgid(0, 0, 0) == 0);
    }

    // start fdns
    Command::new(FDNS)
        .arg(&proxy_addr)
        .args(&fdns_args)
        .env_clear()
        .spawn()
        .expect("Failed to start fdns")
        .wait()
        .unwrap();
}

/// Parse and validate the arguments
///
/// The first argument must be `--proxy-addr=127.70.74.[0-9]{1,3}` or `--help`.
/// All other arguments are optional. Currently supported are `--blocklist=[A-Za-z0-9._-]+` and
/// `--whitelist=[A-Za-z0-9._-]+` in any order and number.
fn parse_and_validate_args<T: Iterator<Item = String>>(args: &mut T) -> (String, Vec<String>) {
    // validate first commandline arg (--proxy-addr)
    let proxy_addr = {
        let arg_1 = args
            .next()
            .expect("No command-line arguments given. --proxy-addr must be given.");

        if arg_1 == "--help" {
            help()
        }

        if arg_1.starts_with("--proxy-addr=127.70.74.")
            && arg_1[23..].chars().all(|c| c.is_ascii_digit())
            && 24 <= arg_1.len()
            && arg_1.len() <= 26
        {
            arg_1
        } else {
            panic!(
                "Invalid first argument, must be --help or --proxy-addr with a allowed IP-address."
            );
        }
    };

    // parse left over commandline args, keep only '--whitelist=[A-Za-z0-9._-]*'
    // and '--blocklist=[A-Za-z0-9._-]*'
    let fdns_args = args
        .filter(|arg| {
            (arg.starts_with("--blocklist=") || arg.starts_with("--whitelist="))
                && arg[12..]
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_')
        })
        .collect::<Vec<_>>();

    (proxy_addr, fdns_args)
}

/// Show help and exit
fn help() -> ! {
    unsafe {
        let uid = libc::getuid();
        assert!(libc::setresuid(uid, uid, uid) == 0);
    }

    print!(
        "{} {} -- {}

USAGE:
    {0} --help
    {0} --proxy-addr=127.70.74.<DIGITS> [OPTIONS]

OPTIONS:
    --blocklist=<DOMAIN>
    --whitelist=<DOMAIN>
",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION"),
    );

    exit(0);
}
