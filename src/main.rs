#![no_std]
#![no_main]

mod priority_queue;
use priority_queue::PriorityQueue;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
   
    
    let arr: [u32; 32] = [23, 87, 54, 12, 6, 78, 65, 42, 19, 30, 
    4, 91, 72, 11, 57, 99, 18, 25, 60, 35, 
    48, 66, 27, 81, 13, 92, 74, 31, 50, 38, 
    76, 22];

    let mut pq = PriorityQueue::from_array(arr);
    pq.build_max_heap();
    pq.print_heap();
    pq.build_min_heap();
   
   
    loop {
      if !pq.is_empty() {
        let job = pq.pop().expect("empty");
        job_function(job);
      }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
fn job_function(job: u32) {
    let i = job;
}