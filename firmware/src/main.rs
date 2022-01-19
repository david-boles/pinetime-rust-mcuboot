#![no_main]
#![no_std]

// #[no_mangle]
// #[link_section = ".ccmram"]
// pub static VAR1: u32 = 1;

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use nrf52832_hal as hal;
use nrf52832_hal::gpio::Level;
use rtt_target::{rprintln, rtt_init_print};

static FOO: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(0);

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    port0.p0_15.into_push_pull_output(Level::High);
    let button = port0.p0_13.into_floating_input().degrade();
    let mut led_mid = port0.p0_22.into_push_pull_output(Level::High);
    let mut led_high = port0.p0_23.into_push_pull_output(Level::High);

    rprintln!("Blinky button demo starting");
    loop {
        if button.is_high().unwrap() {
            rprintln!("High!");
            led_mid.set_high().unwrap();
            led_high.set_high().unwrap();
            FOO.store(7, core::sync::atomic::Ordering::Relaxed);
        } else {
            rprintln!("Low!");
            led_mid.set_low().unwrap();
            led_high.set_low().unwrap();
        }
    }
}
