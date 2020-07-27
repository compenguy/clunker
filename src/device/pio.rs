use crate::device::Due;
use atsam3x8e;

impl Due {
    // Parallel I/O controller B clock control (PIOB)
    pub(crate) fn piob_start_clock(&mut self) {
        self.p
            .PMC
            .pmc_pcer0
            .write_with_zero(|w| w.pid12().set_bit());
    }

    pub(crate) fn piob_stop_clock(&mut self) {
        self.p
            .PMC
            .pmc_pcdr0
            .write_with_zero(|w| w.pid12().set_bit());
    }
}
