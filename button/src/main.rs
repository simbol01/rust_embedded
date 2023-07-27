#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO, Delay};


/*
Button mapping

Boot -> GPIo0
EN -> GPI01
LED -> GPIo2
POWER LED -> GPIo3


*/


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
    //println!("Hello world!");

    /* 1. Creating a panic situation and seeing how it is being printed out by the block */
    //panic!("This is a panic error!");


    /*2. Blinking the led light of the board */
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    
    // button definition and assignment
    let mut led = io.pins.gpio2.into_push_pull_output();
    let mut power_led = io.pins.gpio3.into_push_pull_output();
    let mut boot_btn = io.pins.gpio0.into_pull_up_input();
    let mut en_btn = io.pins.gpio1.into_pull_up_input();

    // set led button low
    //led.set_low().unwrap();
    //power_btn.set_low().unwrap();
    //let mut delay = Delay::new(&clocks);
    /* 
    loop {
        led.toggle().unwrap();
        println!("led shinning!");
        delay.delay_ms(1000u32);
    }
    */
    loop{
        if boot_btn.is_low().unwrap(){
            led.set_high().unwrap();
        }else{
            led.set_low().unwrap();
        }
        //delay.delay_ms(1000u32)
    }
}
