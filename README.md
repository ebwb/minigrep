# minigrep
This is a (very humble) re-implementation of `grep`, as led through the Rust Programming Book, Chapter 12: [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).


## Build
Build with cargo. From root of the repository, run `cargo build`. The executable will be located at `target/debug/minigrep`.

Optionally, run `cargo build` with the `--release` option. The exeuctable will instead be located at `target/release/minigrep`.

## Invoke
`$ minigrep grep pattern filename`

## Future Changes
1. Multiple files, allowing usage of the `*` wildcard
1. Options
    1. Case insensitivity
    1. Printing line numbers
    1. Context (similar to `grep`'s `--context` option
    1. Recursive search, digging into contents of folders
