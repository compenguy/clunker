use crate::device::Due;

impl Due {
    pub(crate) fn init_flash_rw_cycle_times(&mut self) {
        // Set FWS according to SYS_BOARD_MCKR configuration
        //self.p.EFC0.fmr.write(|w| unsafe { w.fws().bits(4) });
        //self.p.EFC1.fmr.write(|w| unsafe { w.fws().bits(4) });
    }
}
