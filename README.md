# RustEmbeddedStateVertex

A simple Rust-based embedded systems program that simulates adding vertices to a circular buffer while polling for system events. This project demonstrates the use of Rust in embedded development, integrating system event polling with vertex management, and employing the circular buffer technique for efficient data storage.

## Features

- **Vertex Management**: Each vertex contains a timestamp, event flags, general-purpose registers, hardware-specific registers, and adjacency list data.
- **Circular Buffer**: The vertices are stored in a circular buffer to efficiently manage a fixed number of items.
- **System Event Polling**: The program polls for various system events (e.g., timer events, GPIO pin changes, interrupts) using the `EventField` bitflags.

## Requirements

- **Rust Toolchain**: Ensure Rust and Cargo are installed. You can install Rust by following the instructions at [rustup.rs](https://rustup.rs).
- **ARM Embedded Toolchain**: You'll need the ARM embedded toolchain for compiling the program for embedded systems.
- **ST-Link Utility**: This is used for flashing the binary to an ARM Cortex-M microcontroller.

## Building and Flashing the Program

### 1. Clone the Repository

```bash
git clone https://github.com/stevemac321/RustStateVertex.git
cd RustStateVertex

