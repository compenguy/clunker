use atsam3x8e;

pub(crate) mod pio;
pub(crate) mod led;
pub(crate) mod time;
pub(crate) mod serial;
pub(crate) mod spi;
pub(crate) mod usb;
pub(crate) mod wdt;
pub(crate) mod sys;
use crate::device::sys::CHIP_FREQ_MAINCK_RC_4MHZ;

pub(crate) struct Device<P> {
    p: P,
    syscore: u32,
}

pub(crate) type Due = Device<atsam3x8e::Peripherals>;

impl Device<atsam3x8e::Peripherals> {
    pub(crate) fn new() -> Self {
        let mut a = Self {
            p: atsam3x8e::Peripherals::take().expect("Failed to acquire device peripherals"),
            syscore: CHIP_FREQ_MAINCK_RC_4MHZ,
        };
        a._init();
        a
    }

    // See
    // https://github.com/arduino/ArduinoModule-CMSIS-Atmel/blob/master/CMSIS-Atmel/CMSIS/Device/ATMEL/sam3xa/source/system_sam3xa.c
    // for sys_init steps
    fn _init(&mut self) {
        self.init_flash_rw_cycle_times();
        self.disable_watchdog();
        self.enable_main_oscillator();
        self.enable_pll();
        self.set_pll_as_main_clock();
        self.update_syscore();
    }
}
