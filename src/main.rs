#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

mod app;

#[entry]
fn main() -> ! {
    let mut app = app::App::new();
    app.run();
}
