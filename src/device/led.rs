use crate::device::Device;
use atsam3x8e;

impl Device<atsam3x8e::Peripherals> {
    pub(crate) fn led_enable(&mut self) {
        // Configure PIOB pin 27 (LED)
        // enable, set to output, disable pull-up resistor
        self.p.PIOB.per.write_with_zero(|w| w.p27().set_bit());
        self.p.PIOB.oer.write_with_zero(|w| w.p27().set_bit());
        self.p.PIOB.pudr.write_with_zero(|w| w.p27().set_bit());
    }

    pub(crate) fn led_on(&mut self) {
        self.p.PIOB.odsr.modify(|_, w| w.p27().set_bit());
    }

    pub(crate) fn led_off(&mut self) {
        self.p.PIOB.odsr.modify(|_, w| w.p27().clear_bit());
    }
}
