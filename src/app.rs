use atsam3x8e as target_device;
use atsam3x8e_hal as hal;

use hal::prelude::*;

pub(crate) struct App {
    clk: hal::clock::SystemClocks,
    delay: hal::delay::Delay<target_device::SYST>,
    led_pin: hal::gpio::Pb27<hal::gpio::Output<hal::gpio::PushPull>>,
}

impl App {
    pub(crate) fn new() -> Self {
        let atsam3x8e::Peripherals {
            PMC: pmc,
            SUPC: supc,
            WDT: wdt,
            PIOB: piob,
            MATRIX: matrix,
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

        // get PIOB so we can access the pin we need
        let piob = hal::gpio::PioGroup::from(piob);
        let mut led_pin = piob.p27().into_push_pull_output();
        led_pin.set_low();

        // Toggle muxed sysio outputs to their peripheral outputs
        let _matrix = hal::bus::BusInterconnect::from(matrix);

        let mut a = Self { clk, delay, led_pin };
        a._enable_led();
        a
    }

    fn _enable_led(&mut self) {
        self.clk
            .state
            .enable_peripheral_clock(hal::clock::PeripheralID::Id12PioB);

        self.led_pin.set_low();
    }

    pub(crate) fn run(&mut self) -> ! {
        loop {
            self.led_pin.toggle();
            let _ = self.delay.try_delay_ms(1000u32);
        }
    }
}
