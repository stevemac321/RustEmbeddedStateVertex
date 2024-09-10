#![no_std]
#![no_main]

use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;
use bitflags::bitflags;
use core::cell::Cell;

const QUEUE_SIZE: usize = 32;

bitflags! {
    pub struct EventField: u32 {
        const TIMER_EVENT            = 0x00000001;
        const GPIO_PIN_CHANGE        = 0x00000002;
        const INTERRUPT_EVENT        = 0x00000004;
        const ENTER_CRITICAL_SECTION = 0x00000008;
        const EXIT_CRITICAL_SECTION  = 0x00000010;
        const POWER_STATE_CHANGE     = 0x00000020;
        const ERROR_EVENT            = 0x00000040;
        const WATCHDOG_RESET         = 0x00000080;
        const SENSOR_READ_EVENT      = 0x00000100;
        const COMMUNICATION_EVENT    = 0x00000200;
        const MAIN_LOOP_EVENT        = 0x00000400;
        const INTERRUPT_HANDLER_EVENT = 0x00000800;
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    timestamp: i32,
    event: EventField,
    distance: i32,
    parent: i32,
    gp_registers: [i32; 8],
    hw_registers: [i32; 4],
    adj_list: u32,
}

struct VertexBuffer {
    buffer: [Option<Vertex>; QUEUE_SIZE],
    head: Cell<usize>,
    tail: Cell<usize>,
}

impl VertexBuffer {
    fn new() -> Self {
        VertexBuffer {
            buffer: [None; QUEUE_SIZE],
            head: Cell::new(0),
            tail: Cell::new(0),
        }
    }

    fn add_vertex(&mut self, vertex: Vertex) {
        let tail = self.tail.get();
        self.buffer[tail] = Some(vertex);
        self.tail.set((tail + 1) % QUEUE_SIZE);

        if self.tail.get() == self.head.get() {
            self.head.set((self.head.get() + 1) % QUEUE_SIZE);
        }
    }
}

// Mock system polling functions
fn poll_timer_event() -> bool {
    false
}

fn poll_gpio_pin_change() -> bool {
    false
}

fn poll_interrupt_event() -> bool {
    false
}

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

    let new_vertex = Vertex {
        timestamp: 123456,  // Mock timestamp
        event,
        distance: 0,        // Default value
        parent: -1,         // Default parent
        gp_registers: [0; 8], // Placeholder general-purpose registers
        hw_registers: [0; 4], // Placeholder hardware-specific registers
        adj_list: 0,        // No adjacency list initially
    };

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
