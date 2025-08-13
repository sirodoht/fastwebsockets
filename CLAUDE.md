# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

fastwebsockets is a fast, minimal WebSocket protocol implementation in Rust that passes the Autobahn TestSuite and is fuzzed with LLVM's libfuzzer. It provides both low-level frame parsing and high-level WebSocket client/server functionality.

## Development Commands

### Building and Testing
- `cargo build` - Build the library
- `cargo build --release` - Build optimized release version
- `cargo test` - Run all tests
- `cargo test --features "upgrade"` - Run tests with upgrade feature
- `cargo test --features "upgrade,unstable-split"` - Run tests with split functionality
- `cargo bench` - Run benchmarks

### Examples
- `cargo run --example echo_server --features "upgrade"` - Run echo server example
- `cargo run --example axum --features "upgrade,with_axum"` - Run Axum integration example
- `cargo run --example tls_server --features "upgrade"` - Run TLS server example
- `cargo run --example autobahn_client --features "upgrade"` - Run Autobahn test client

### Autobahn TestSuite
- `cd autobahn && make run-server` - Run server against Autobahn TestSuite
- `cd autobahn && make run-client` - Run client against Autobahn TestSuite

### Fuzzing
- `cd fuzz && cargo fuzz run parse_frame` - Fuzz frame parsing
- `cd fuzz && cargo fuzz run parse_frame_fragment` - Fuzz frame fragmentation
- `cd fuzz && cargo fuzz run unmask` - Fuzz unmasking logic

## Code Architecture

### Core Components

**WebSocket struct (src/lib.rs:363)**: Main WebSocket implementation that combines read and write halves. Provides `read_frame()` and `write_frame()` methods for basic WebSocket operations.

**Frame parsing (src/frame.rs)**: Low-level WebSocket frame parsing and writing. Handles opcode detection, masking/unmasking, and payload management with different payload types (Borrowed, Owned, Bytes).

**Fragment handling (src/fragment.rs)**: `FragmentCollector` assembles fragmented WebSocket messages into complete messages. Essential for proper WebSocket message handling as the library defaults to returning raw frames.

**Handshake module (src/handshake.rs)**: Client-side WebSocket handshake implementation using hyper for HTTP upgrade requests.

**Upgrade module (src/upgrade.rs)**: Server-side HTTP-to-WebSocket upgrade handling, also powered by hyper.

### Key Features

**Feature flags**:
- `upgrade`: Enables HTTP upgrade functionality (both client and server)
- `with_axum`: Enables Axum web framework integration
- `unstable-split`: Enables splitting WebSocket into separate read/write halves

**Roles**: WebSocket can operate as either `Role::Server` or `Role::Client`, affecting masking behavior and frame validation.

**Auto-behaviors**: Configurable automatic handling of ping/pong frames (`set_auto_pong`), close frames (`set_auto_close`), and masking (`set_auto_apply_mask`).

### Integration Patterns

**Raw usage**: Use `WebSocket::after_handshake()` with a pre-established connection after manual handshake.

**With HTTP servers**: Use the `upgrade` feature with hyper-based servers. The upgrade returns a future that resolves to a WebSocket.

**With Axum**: Enable `with_axum` feature for direct integration with Axum web framework.

**Fragmentation**: For applications expecting complete messages rather than individual frames, wrap the WebSocket in a `FragmentCollector`.

## Testing Strategy

The codebase uses multiple testing approaches:
- Unit tests in each module
- Integration tests in `tests/` directory  
- Autobahn TestSuite compliance testing
- Fuzz testing for frame parsing and critical paths
- UI tests for compile-time error validation

## Rust Toolchain

The project uses Rust 1.89.0 as specified in `rust-toolchain`.