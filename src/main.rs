#![no_std]
#![no_main]
mod event;
mod priority_queue;
use priority_queue::PriorityQueue;
use event::poll_gpio_pin_change;
use event::poll_interrupt_event;
use event::poll_timer_event;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use cortex_m::peripheral::NVIC;
use stm32f4xx_hal::{
    pac::{self, TIM2},
    prelude::*,
    timer::{Event, Timer},
};

// Interrupt handler for TIM2
#[interrupt]
fn TIM2() {
    // This is called on every timer interrupt
    hprintln!("TIM2 interrupt triggered").unwrap(); // Debug print, works if semihosting is enabled

    // Clear interrupt flag
    let tim2 = unsafe { &*TIM2::ptr() };
    tim2.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {

    let peripherals = pac::Peripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();  // Adjust the clock as per your requirements
    let mut timer = Timer::tim2(peripherals.TIM2, 1.hz(), clocks);

    // Start the timer in interrupt mode
    timer.listen(Event::TimeOut);

    // Enable TIM2 interrupt in NVIC
    unsafe {
        NVIC::unmask(pac::Interrupt::TIM2);
    }

   let arr: [i32; 32] = [23, 87, 54, 12, 6, 78, 65, 42, 19, 30, 
   4, 91, 72, 11, 57, 99, 18, 25, 60, 35, 
   48, 66, 27, 81, 13, 92, 74, 31, 50, 38, 
   76, 22];

    let mut pq = PriorityQueue::new(arr);
    pq.build_max_heap();
    pq.print_heap();
    pq.build_min_heap();
    
    loop {
        

        // Later, you can add a `report` function here to print out the vertex data
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
fn encode_job(job_id: u32, timestamp: u32) -> u32 {
    (job_id << 29) | (timestamp & 0x1FFFFFFF)  // Shift Job ID and combine with timestamp
}

fn decode_job(encoded: u32) -> (u32, u32) {
    let job_id = encoded >> 29;  // Extract the 3 MSBs for the job ID
    let timestamp = encoded & 0x1FFFFFFF;  // Extract the remaining 29 bits for the timestamp
    (job_id, timestamp)
}
