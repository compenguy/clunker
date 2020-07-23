#![no_std]
#![no_main]

use cortex_m_rt::entry;
extern crate panic_halt;

mod device;
use crate::device::Due;

struct App {
    due: Due,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut a = Self {
            due: Due::new(),
        };
        a._init();
        a
    }

    fn _init(&mut self) {
        self.due.piob_start_clock();
        self.due.led_enable();
        self.due.led_off();
        // Configure RTT resolution to approx 1 ms
        self.due.rtt_set_resolution(0x20);
    }

    fn run(&mut self) -> ! {
        loop {
            self.due.led_on();
            self.due.delay_ms(1000);
            self.due.led_off();
            self.due.delay_ms(1000);
        }
    }
}

#[entry]
fn main() -> ! {
    let mut app = App::new();
    app.run();
}
