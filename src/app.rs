use atsam3x8e as target_device;
use atsam3x8e_hal as hal;

use hal::prelude::*;

pub(crate) struct App {
    clk: hal::clock::SystemClocks,
    delay: hal::delay::Delay<target_device::SYST>,
    piob: atsam3x8e::PIOB,
}

impl App {
    pub(crate) fn new() -> Self {
        let atsam3x8e::Peripherals {
            PMC: pmc,
            SUPC: supc,
            WDT: wdt,
            PIOB: piob,
            ..
        } = atsam3x8e::Peripherals::take().expect("Failed to acquire device peripherals");

        let target_device::CorePeripherals { SYST: syst, .. } =
            target_device::CorePeripherals::take().expect("Failed to acquire core peripherals");

        let mut clk = hal::clock::SystemClocks::new(pmc, supc);
        //let clk = hal::clock::SystemClocks::with_plla_clk(pmc, supc);

        // disable watchdog timer, otherwise the system resets every 15.996ms
        let _wdt = hal::watchdog::WdtBuilder::from(wdt).disable();

        // configure blocking sleep delay object
        let delay = hal::delay::Delay::new(syst, clk.get_syscore());

        let mut a = Self { clk, delay, piob };
        a._enable_led();
        a
    }

    fn _enable_led(&mut self) {
        self.clk
            .state
            .enable_peripheral_clock(hal::clock::PeripheralID::Id12PioB);

        // Configure PIOB pin 27 (LED)
        // enable, set to output, disable pull-up resistor
        self.piob.per.write_with_zero(|w| w.p27().set_bit());
        self.piob.oer.write_with_zero(|w| w.p27().set_bit());
        self.piob.pudr.write_with_zero(|w| w.p27().set_bit());

        self.led_off();
    }

    pub(crate) fn led_off(&mut self) {
        self.piob.codr.write_with_zero(|w| w.p27().set_bit());
    }

    pub(crate) fn led_on(&mut self) {
        self.piob.sodr.write_with_zero(|w| w.p27().set_bit());
    }

    pub(crate) fn run(&mut self) -> ! {
        loop {
            self.led_on();
            let _ = self.delay.try_delay_ms(1000u32);
            self.led_off();
            let _ = self.delay.try_delay_ms(1000u32);
        }
    }
}
