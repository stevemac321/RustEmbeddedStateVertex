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
pub struct Vertex {
    timestamp: i32,
    event: EventField,
    distance: i32,
    parent: i32,
    gp_registers: [i32; 8],
    hw_registers: [i32; 4],
    adj_list: u32,
}
impl Vertex {
    // Constructor method to initialize Vertex
    pub fn new(
        timestamp: i32,
        event: EventField,
        distance: i32,
        parent: i32,
        gp_registers: [i32; 8],
        hw_registers: [i32; 4],
        adj_list: u32,
    ) -> Self {
        Vertex {
            timestamp,
            event,
            distance,
            parent,
            gp_registers,
            hw_registers,
            adj_list,
        }
    }
}

pub struct VertexBuffer {
    buffer: [Option<Vertex>; QUEUE_SIZE],
    head: Cell<usize>,
    tail: Cell<usize>,
}

impl VertexBuffer {
    pub fn new() -> Self {
        VertexBuffer {
            buffer: [None; QUEUE_SIZE],
            head: Cell::new(0),
            tail: Cell::new(0),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        let tail = self.tail.get();
        self.buffer[tail] = Some(vertex);
        self.tail.set((tail + 1) % QUEUE_SIZE);

        if self.tail.get() == self.head.get() {
            self.head.set((self.head.get() + 1) % QUEUE_SIZE);
        }
    }
}

