#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, Delay, IO};// Set GPIO7 as an output, and set its state high initially.
    
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    println!("Hello world, welcome to the world of embedded with esp32 and rust, I really love this!");


    // blink in rust
    // Set GPIO2 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();

    led.set_high().unwrap();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let mut delay = Delay::new(&clocks);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(1000u32);
    }
}
