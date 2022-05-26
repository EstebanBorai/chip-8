## Running it Locally

```bash
cargo run ./roms/IBM.ch8
```

## Prerequisites

You may need to setup some libraries in order to run this project locally.

### macOS

- SDL2: Install it with brew using `brew install SDL2`

Then either extend the `LIBRARY_PATH` environment variable to include
Homebrew's installed libraries by adding:

```bash
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

To your `~/.zshenv` or `~/.bash_profile` or specify the environment variable
when running `cargo run`.

```bash
LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib" cargo run roms/INVADERS
```

### Linux

> Feel free to open a PR providing Linux system setup

### Windows

> Feel free to open a PR providing Windows system setup

## References

- [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0)
- [Wikipedia Chip-8 Page](https://en.wikipedia.org/wiki/CHIP-8)
