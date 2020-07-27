use atsam3x8e;
use atsam3x8e_hal as hal;

pub(crate) mod led;
//pub(crate) mod pio;
//pub(crate) mod time;
//pub(crate) mod serial;
//pub(crate) mod spi;
//pub(crate) mod usb;
pub(crate) mod sys;
pub(crate) mod wdt;

pub(crate) struct Device<C, W, PB> {
    pub clk: C,
    pub wdt: W,
    pub piob: PB,
}

pub(crate) type Due = Device<hal::clock::SystemClocks, atsam3x8e::WDT, atsam3x8e::PIOB>;

//impl Device<hal::clock::SystemClocks, atsam3x8e::WDT> {
impl Due {
    pub(crate) fn new() -> Self {
        let atsam3x8e::Peripherals {
            PMC: pmc,
            SUPC: supc,
            WDT: wdt,
            PIOB: piob,
            ..
        } = atsam3x8e::Peripherals::take().expect("Failed to acquire device peripherals");
        let clk = hal::clock::SystemClocks::new(pmc, supc);

        let mut d = Device { clk, wdt, piob };
        d._init();
        d
    }

    // See
    // https://github.com/arduino/ArduinoModule-CMSIS-Atmel/blob/master/CMSIS-Atmel/CMSIS/Device/ATMEL/sam3xa/source/system_sam3xa.c
    // for sys_init steps
    fn _init(&mut self) {
        //self.init_flash_rw_cycle_times();
        self.disable_watchdog();
    }
}
