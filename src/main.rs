#![no_std]
#![no_main]
mod vertex;
mod event;
use event::poll_gpio_pin_change;
use event::poll_interrupt_event;
use event::poll_timer_event;

use vertex::Vertex;
use vertex::VertexBuffer;
use crate::vertex::EventField;

use cortex_m_rt::entry;
// Mock system polling functions

fn add_vertex(buffer: &mut VertexBuffer) {
    let mut event = EventField::empty();

    if poll_timer_event() {
        event |= EventField::TIMER_EVENT;
    }
    if poll_gpio_pin_change() {
        event |= EventField::GPIO_PIN_CHANGE;
    }
    if poll_interrupt_event() {
        event |= EventField::INTERRUPT_EVENT;
    }

    let new_vertex = Vertex::new(
        123456,         // Mock timestamp
        event,          // Event field
        0,              // Default distance
        -1,             // Default parent
        [0; 8],         // General-purpose registers
        [0; 4],         // Hardware-specific registers
        0,              // Adjacency list
    );

    buffer.add_vertex(new_vertex);
}

#[entry]
fn main() -> ! {
    let mut vertex_buffer = VertexBuffer::new();
    add_vertex(&mut vertex_buffer);  // Continuously poll for events and add vertices
    loop {
        

        // Later, you can add a `report` function here to print out the vertex data
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
