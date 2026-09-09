# CHIP 8 Emulator

A simple CHIP 8 Emulator/Intepreter written in Rust.

Functional, but still in development. Final binary coming soon, for now compile from source.

## Usage:

Include the rom as a command line argument

```bash
cargo run --bin Chip_8 [FLAGS] [PATH_TO_ROM]
```

## Flags:
- -l/--legacy: Some instructions behave differently in different implementations. If a rom isn't working properly, try this flag
- -o/--odds: allows odd addresses in memory. When toggled the emulator won't stop if the program counter jumps to an invalid location. This is required for some roms.
- -w/--wrapping: By default sprites that go off screen are clipped. Enable this flag to wrap them to the other side instead. This is needed on some older roms.

## [Debugger](src/bin/c8db.md)
