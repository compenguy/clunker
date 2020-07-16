#![no_std]
#![no_main]

use cortex_m_rt::entry;
extern crate panic_halt;

mod device;
use device::Device;
mod time;

fn setup() -> Device {
    let mut device = Device::new();
    device.piob_enable();
    // Configure RTT resolution to approx 1 ms
    device.rtt_set_resolution(0x20);

    // Enable the user LED
    device.led_enable();

    device
}

#[entry]
fn main() -> ! {
    let mut device = setup();
    // blink
    loop {
        device.led_on();
        time::delay_ms(&mut device, 5000);
        device.led_off();
        time::delay_ms(&mut device, 5000);
    }
}
