# fdns4users

Allow unprivileged users to start fdns.

## Getting started

First install [rust](https://www.rust-lang.org/tools/install), then execute the following commands.

```
git clone https://github.com/rusty-snake/fdns4users.git
cd fdns4users
cargo build --release
sudo install -D -m 4755 -o root -g root target/release/fdns4users /usr/local/bin/fdns4users
```

:rotating_light: This adds a new [suid](https://en.wikipedia.org/wiki/Setuid#Security)-executable to your system. :rotating_light:

You can now start using it, here an example.

```
fdns4users --proxy-addr=127.70.74.68 --whitelist=debian.org &
firejail --dns=127.70.74.68 wget "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.6.0-amd64-netinst.iso"
```

`--proxy-addr` must be the first argument and only use `127.70.74.*`.

## Alternatives

Instead of a own suid-exeutable you can use polkit or sudo.

### Example for sudo:

Attention:
 - Only edit sudoers if you know what you do!
 - Read the relevant parts of the manpage!
 - Always use `visudo`!
 - Keep a root-shell open to undo mistakes, so you won't get locked out.

Allow the user `john` to start `fdns --proxy-addr=127.70.74.<digit between 0 and 9>` via sudo.

`/etc/sudoers.d/40-fdns`:
```
john ALL=(root) NOPASSWD: /usr/bin/fdns --proxy-addr=127.70.74.[0-9]
```

### Example for polkit

`/etc/polkit-1/rules.d/40-fdns.rules`:
```javascript
polkit.addRule(function(action, subject) {
    const USER = "john";
    const PROGRAM = "/usr/bin/fdns";

    const IP = "127\\.70\\.74\\.[0-9]{1,3}";
    const PROXY_ADDR = `--proxy-addr=${IP}`;
    const WHITELIST = `--whitelist=[A-Za-z0-9._-]+`;
    const ZOM_WHITELIST = `( ${WHITELIST})*`;
    const RE = new RegExp(`^${PROGRAM} ${PROXY_ADDR}${ZOM_WHITELIST}$`);

    // Debugging: uncomment to see the final RegExp
    //polkit.log(RE.toString());

    if (action.id === "org.freedesktop.policykit.exec" &&
        action.lookup("program") === PROGRAM &&
        RE.test(action.lookup("command_line")) &&
        subject.user === USER && subject.local && subject.active) {
        return polkit.Result.YES;
    }
});
```

This allows the user john to start `/usr/bin/fdns --proxy-addr=127.70.68.*` with
`--whitelist=DOMAIN1`, `--whitelist=DOMAIN2`, ….

## License

```
Copyright © 2020 rusty-snake
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```
