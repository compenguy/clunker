#![no_std]
#![no_main]

use cortex_m_rt::entry;
extern crate panic_halt;

mod device;
use crate::device::Atsam3x8e;

struct App {
    sam3: Atsam3x8e,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut a = Self {
            sam3: Atsam3x8e::new(),
        };
        a._init();
        a
    }

    fn _init(&mut self) {
        self.sam3.piob_start_clock();
        self.sam3.led_on();
        // Configure RTT resolution to approx 1 ms
        self.sam3.rtt_set_resolution(0x20);
    }

    fn run(&mut self) -> ! {
        loop {
            self.sam3.led_on();
            self.sam3.delay_ms(1000);
            self.sam3.led_off();
            self.sam3.delay_ms(1000);
        }
    }
}

#[entry]
fn main() -> ! {
    let mut app = App::new();
    app.run();
}
