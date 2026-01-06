# async_rust

Small learning repository demonstrating Rust concurrency and async concepts.

## Overview

This repo contains small example programs that explore threading and low-level
behaviors in Rust. It's intended for experimentation and learning rather than
production use.

## Requirements

- Rust toolchain (stable) with `cargo` installed. Install from https://rustup.rs

## Build and run

To build the project:

```bash
cargo build
```

To run the main binary:

```bash
cargo run
```

To run an example (for example `os_threads`):

```bash
cargo run --example os_threads
```

## Examples

- `examples/os_threads.rs` — demonstrates OS threads usage.
- `examples/assembly_deref.rs` — small assembly/dereference exploration.
- `examples/normal_syscall.rs` — example using the normal libc/syscall path to
  perform a simple syscall demonstration.
- `examples/raw_syscall.rs` — performs the same demonstration using a raw
  syscall invocation (lower-level, unsafe demonstration).

Run a specific example with:

```bash
cargo run --example normal_syscall
cargo run --example raw_syscall
```

## Contributing

Suggestions and PRs welcome. Keep changes small and focused.
