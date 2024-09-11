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
- **STM32F401RE Microcontroller**: This program targets the STM32F401RE microcontroller.

## Building and Flashing the Program

### 1. Clone the Repository

```bash
git clone https://github.com/stevemac321/RustStateVertex.git
cd RustStateVertex


1. **Timer as Timestamp**: The timer’s value (`tim2.cnt.read().cnt().bits()`) is captured and stored as the timestamp. This acts as the chronological portion of the data.
   
2. **Job ID in MSB**: You assign job IDs in the most significant bits (MSBs) of the integer, using the remaining bits for the timestamp. This way, the job ID is prioritized in the priority queue.

3. **Queue Operations**: 
   - **Insert**: Each time a new job is created, it gets assigned a job ID in the MSBs and is combined with the current timer value in the lower bits.
   - **Pop**: When the timer interrupt triggers (e.g., 1 second), you pop the highest-priority job from the priority queue, determined first by the job ID (MSBs) and then by the timestamp (lower bits).
   
4. **Job Execution**: After popping the job from the queue, you extract the job ID and timestamp, pass them to a single job-handling function that prints them out (or performs any required actions).

- **Job Prioritization**: The queue naturally prioritizes jobs by their job ID. If two jobs share the same job ID, the timestamp (or timer value) will act as the tie-breaker.
  

