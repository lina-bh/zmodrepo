# zmodrepo

At the moment, this is a proof of concept that a zsh loadable module can be
written out of the zsh tree and in Rust. Eventually, the goal is to have a
module that can print repository information for Git and more importantly
Mercurial for the shell prompt much faster than any pure-shell implementation
like zsh's included `vcs_info` or whatever hideously slow method oh-my-zsh uses,
so the prompt can be drawn as quickly as possible and hopefully without a
percievable delay.  Using native code means that faster methods of gathering
repository information like libgit2 and Mercurial's command server facility can
be used. Forking a new process on every prompt is expensive, forking a new
_Python interpreter_ is even more expensive.

## Compile

I've been using Rust 1.38 so far, I'm not sure how low you can go but I know
that you need at least Rust 1.31 for Rust 2018 support. Apart from that you'll
need zsh's header files installed, they're in zsh-dev on Ubuntu/Debian.  The
usual cargo build instructions apply:
```
cargo build --release
```

## Install

At the moment, you'll need to move the shared library file yourself. Using
`~/zmods` as an example, add this to your `.zshenv`:

```
module_path=(~/zmods $module_path)
```

Copy the shared object from the `target/release` folder and name it without
the `lib` prefix:

```
cp target/release/libmodrepo.so ~/zmods/zmodrepo
```

Now you can load the module:

```
% zmodload zmodrepo
zmodrepo loaded
% rustexample
hello from rust!
```

(This is to say that it won't crash your shell, or release demons from your
laptop's fan exhaust.)

If you're brave you can unload the module:

```
% zmodload -u zmodrepo
zmodrepo unloaded
```

And that's it!

## Licence

zmodrepo - source control information module for zsh in Rust

Copyright ⓒ 2019, 2020 elatelation

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
