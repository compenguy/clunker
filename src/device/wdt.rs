use crate::device::Due;

impl Due {
    pub(crate) fn disable_watchdog(&mut self) {
        self.wdt.mr.write(|w| w.wddis().set_bit());
    }
}
