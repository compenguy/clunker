#![no_std]
#![no_main]

use cortex_m_rt::entry;
extern crate panic_halt;

mod app;

#[entry]
fn main() -> ! {
    let mut app = app::App::new();
    app.run();
}
