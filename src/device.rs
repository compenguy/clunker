use atsam3x8e;
use atsam3x8e_hal as hal;

pub(crate) mod led;
//pub(crate) mod time;
//pub(crate) mod serial;
//pub(crate) mod spi;
//pub(crate) mod usb;

pub(crate) struct Device<C, PB> {
    pub clk: C,
    pub piob: PB,
}

pub(crate) type Due = Device<hal::clock::SystemClocks, atsam3x8e::PIOB>;

//impl Device<hal::clock::SystemClocks, atsam3x8e::WDT> {
impl Due {
    pub(crate) fn new() -> Self {
        let atsam3x8e::Peripherals {
            PMC: pmc,
            SUPC: supc,
            WDT: wdt,
            PIOB: piob,
            EFC0: efc0,
            EFC1: efc1,
            ..
        } = atsam3x8e::Peripherals::take().expect("Failed to acquire device peripherals");
        let clk = hal::clock::SystemClocks::new(pmc, supc);
        //let clk = hal::clock::SystemClocks::with_plla_clk(pmc, supc);

        // disable watchdog timer, otherwise the system resets every 15.996ms
        let _wdt = hal::watchdog::WatchdogBuilder::from(wdt).disabled();

        Device { clk, piob }
    }
}
