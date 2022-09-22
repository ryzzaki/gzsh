## The Gen Z Shell 🤩

The most productive shell you can get in 2022 where all the commands are being steadily replaced by emojis. `Note:` The `Gen Z Shell (gzsh)` is not to be confused with the far inferior and outdated `Z Shell (zsh)`.

## Intro

Why use commands, when you can just emote all day? Heavily inspired by the philosophical nature of The Emoji Movie.

The `Gen Z Shell` is a quick one file bootstrap (for now) of a custom shell written in pure unfiltered `Rust` as the language consistently ranks #1 and is officially part of the 2022 hype beast tech stack.

## Quick Start

```bash
# you can alias cargo to 📦 and use it as `📦 run` to keep the good vibes going
cargo run
```

## List of Implemented Commands

| Emoji | Command |
| ----- | ------- |
| 👋    | `exit`  |
| 🏃    | `cd`    |
| 📖    | `ls`    |
| 🔎    | `cat`   |
| 💀    | `rm`    |
| ❌    | `clear` |
| 🗺     | `pwd`   |
| 📦    | `cargo` |
| 😭    | `help`  |

## Examples

```bash
👉 📦 --version
cargo 1.63.0 (fd9c4297c 2022-07-01)
👉 🏃 ..
👉 📖 -a
. .. .git Cargo.lock	Cargo.toml	src		target
👉 🔎 Cargo.lock
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "gen-z-shell"
version = "0.1.0"
👉 👋
Sayonara 🫡
```

### Todo

1. Refactor, refactor, refactor
2. Implement Pipes
3. Implement Error Handling
4. Keep implementing more shell built-ins to handle certain actions internally in the custom shell
5. Emojify arguments as well for maximising productivity (e.g. `♊️ 🔁 ./a ./b` === `cp -r ./a ./b` - you know, because geminis are twins)