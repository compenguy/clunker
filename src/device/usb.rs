use crate::device::Device;
use atsam3x8e;

const USB_DEVICE_MAX_EP: u8 = 10;
const USB_HOST_MAX_PIPE: u8 = 10;

impl Device<atsam3x8e::Peripherals> {
    // USB clock control (UOTGHS)
    pub(crate) fn usb_start_clock(&mut self) {
        self.p
            .PMC
            .pmc_pcer1
            .write_with_zero(|w| w.pid40().set_bit());
    }

    pub(crate) fn usb_stop_clock(&mut self) {
        self.p
            .PMC
            .pmc_pcdr1
            .write_with_zero(|w| w.pid40().set_bit());
    }
}
