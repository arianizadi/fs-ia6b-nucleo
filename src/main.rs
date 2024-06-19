#![no_std]
#![no_main]

// use cortex_m_semihosting::hprintln;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32f4xx_hal::pac::Peripherals;
use stm32f4xx_hal::prelude::*;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    if let Some(peripherals) = Peripherals::take() {
        let gpioa = peripherals.GPIOA.split();
        let gpioc = peripherals.GPIOC.split();

        let mut led = gpioa.pa5.into_push_pull_output();
        let button = gpioc.pc13;
        let mut led2 = gpioa.pa8.into_push_pull_output();

        loop {
            if button.is_high() {
                led.set_low();
                led2.set_low();
            } else {
                led.set_high();
                led2.set_high();
            }
        }
    }
    loop {}
}
