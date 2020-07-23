use crate::device::Device;
use atsam3x8e;
use nb;

pub(crate) enum SerialError {
    Overflow,
}

impl Device<atsam3x8e::Peripherals> {
    // UART clock control (UART)
    // TODO:
    // Datasheet in one place says uart (PID8) is always clocked, no PMC control
    // In another place it says that the UART clock _is_ controller by PMC as PID1.
    pub(crate) fn uart_start_clock(&mut self) {
        self.p.PMC.pmc_pcer0.write_with_zero(|w| w.pid8().set_bit());
    }

    pub(crate) fn uart_stop_clock(&mut self) {
        self.p.PMC.pmc_pcdr0.write_with_zero(|w| w.pid8().set_bit());
    }

    pub(crate) fn enable(&mut self) {
        self.p.UART.cr.write_with_zero(|w| w.rxen().set_bit());
    }

    pub(crate) fn disable(&mut self) {
        self.p.UART.cr.write_with_zero(|w| w.rxdis().set_bit());
    }

    pub(crate) fn get_baud_mck_divider(&self) -> u16 {
        self.p.UART.brgr.read().cd().bits() * 16
    }

    pub(crate) fn set_baud_mck_divider(&mut self, divider: u32) {
        self.p
            .UART
            .brgr
            .write_with_zero(|w| unsafe { w.cd().bits((divider / 16) as u16) });
    }

    pub(crate) fn write(&mut self, word: u8) -> nb::Result<(), nb::Error<SerialError>> {
        if self.p.UART.sr.read().txempty().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            self.p
                .UART
                .thr
                .write_with_zero(|w| unsafe { w.txchr().bits(word) });
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), nb::Error<SerialError>> {
        if self.p.UART.sr.read().txempty().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}
