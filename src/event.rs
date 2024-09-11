pub fn poll_timer_event() -> bool {
    false
}

pub fn poll_gpio_pin_change() -> bool {
    false
}

pub fn poll_interrupt_event() -> bool {
    false
}
// Function to initialize TIM2 and return the current timer count
pub fn get_time_value() -> u32 {
    // Enable the clock for TIM2 (assuming the STM32 HAL is already set up)
    // The actual register setup depends on your HAL or hardware abstraction layer
    unsafe {
        let rcc = &(*stm32f4::stm32f401::RCC::ptr());
        rcc.apb1enr.modify(|_, w| w.tim2en().enabled()); // Enable TIM2 clock
    }

    // Configure TIM2
    let tim2 = unsafe { &*stm32f4::stm32f401::TIM2::ptr() };
    tim2.cr1.modify(|_, w| w.cen().clear_bit()); // Stop timer if running
    tim2.psc.write(|w| w.psc().bits(15999));     // Set prescaler (adjust based on system clock)
    tim2.arr.write(|w| w.arr().bits(0xFFFF));    // Set auto-reload value
    tim2.egr.write(|w| w.ug().set_bit());        // Generate an update event
    tim2.cr1.modify(|_, w| w.cen().set_bit());   // Start the timer

    // Return the current value of the counter
    tim2.cnt.read().cnt().bits()
}


