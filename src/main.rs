#![no_std]
#![no_main]

use atsam3x8e as target_device;
use atsam3x8e_hal as hal;
use cortex_m_rt::entry;
use embedded_hal::prelude::*;
extern crate panic_halt;

mod device;
use crate::device::Due;

struct App {
    due: Due,
    delay: hal::delay::Delay,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut due = Due::new();
        let core =
            target_device::CorePeripherals::take().expect("Failed to acquire core peripherals");
        let core_freq = due.clk.get_syscore();
        let mut a = Self {
            due,
            delay: hal::delay::Delay::new(core.SYST, core_freq),
        };
        a._init();
        a
    }

    fn _init(&mut self) {
        self.due
            .clk
            .state
            .enable_peripheral_clock(hal::clock::PeripheralID::Id12PioB);
        self.due.led_enable();
        self.due.led_off();
    }

    fn run(&mut self) -> ! {
        loop {
            self.due.led_on();
            self.delay.delay_ms(1000u32);
            self.due.led_off();
            self.delay.delay_ms(1000u32);
        }
    }
}

#[entry]
fn main() -> ! {
    let mut app = App::new();
    app.run();
}
