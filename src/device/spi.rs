use crate::device::Device;
use atsam3x8e;

impl Device<atsam3x8e::Peripherals> {
    // SPI clock control (SPI0/SPI1)
    pub(crate) fn spi0_start_clock(&mut self) {
        self.p
            .PMC
            .pmc_pcer0
            .write_with_zero(|w| w.pid24().set_bit());
    }

    pub(crate) fn spi0_stop_clock(&mut self) {
        self.p
            .PMC
            .pmc_pcdr0
            .write_with_zero(|w| w.pid24().set_bit());
    }

    pub(crate) fn spi1_start_clock(&mut self) {
        self.p
            .PMC
            .pmc_pcer0
            .write_with_zero(|w| w.pid25().set_bit());
    }

    pub(crate) fn spi1_stop_clock(&mut self) {
        self.p
            .PMC
            .pmc_pcdr0
            .write_with_zero(|w| w.pid25().set_bit());
    }
}
