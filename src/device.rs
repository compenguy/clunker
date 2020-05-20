use sam3x8e;

pub(crate) struct Device {
    p: sam3x8e::Peripherals,
}

impl Device {
    pub(crate) fn new() -> Self {
        Device {
            p: sam3x8e::Peripherals::take().expect("Failed to acquire device peripherals"),
        }
    }

    pub(crate) fn piob_enable(&mut self) {
        self.p.PMC.pmc_pcer0.write(|w| w.pid12().set_bit());
    }

    pub(crate) fn rtt_set_resolution(&mut self, rt_prescale: u16) {
        self.p
            .RTT
            .mr
            .write(|w| unsafe { w.rtpres().bits(rt_prescale) });
    }

    pub(crate) fn rtt_get_value(&self) -> u32 {
        self.p.RTT.vr.read().bits()
    }

    pub(crate) fn led_enable(&mut self) {
        // Configure PIOB pin 27 (LED)
        self.p.PIOB.per.write(|w| w.p27().set_bit());
        self.p.PIOB.oer.write(|w| w.p27().set_bit());
        self.p.PIOB.pudr.write(|w| w.p27().set_bit());
    }

    pub(crate) fn led_on(&mut self) {
        self.p.PIOB.sodr.write(|w| w.p27().set_bit());
    }

    pub(crate) fn led_off(&mut self) {
        self.p.PIOB.codr.write(|w| w.p27().set_bit());
    }
}
