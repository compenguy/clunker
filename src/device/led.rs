use crate::device::Due;

impl Due {
    pub(crate) fn led_enable(&mut self) {
        // Configure PIOB pin 27 (LED)
        // enable, set to output, disable pull-up resistor
        self.piob.per.write_with_zero(|w| w.p27().set_bit());
        self.piob.oer.write_with_zero(|w| w.p27().set_bit());
        self.piob.pudr.write_with_zero(|w| w.p27().set_bit());
    }

    pub(crate) fn led_on(&mut self) {
        self.piob.sodr.write_with_zero(|w| w.p27().set_bit());
    }

    pub(crate) fn led_off(&mut self) {
        self.piob.codr.write_with_zero(|w| w.p27().set_bit());
    }
}
