use crate::device::Device;
use atsam3x8e;

impl Device<atsam3x8e::Peripherals> {
    pub(crate) fn disable_watchdog(&mut self) {
        self.p.WDT.mr.write(|w| w.wddis().set_bit());
    }
}
