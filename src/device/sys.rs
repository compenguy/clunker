use crate::device::Device;
use atsam3x8e::{self, generic::Variant, pmc::ckgr_mor, pmc::pmc_mckr, supc};

pub(crate) const CHIP_FREQ_SLCK_RC_MIN: u32 = 20000;
pub(crate) const CHIP_FREQ_SLCK_RC: u32 = 32000;
pub(crate) const CHIP_FREQ_SLCK_RC_MAX: u32 = 44000;
pub(crate) const CHIP_FREQ_MAINCK_RC_4MHZ: u32 = 4000000;
pub(crate) const CHIP_FREQ_MAINCK_RC_8MHZ: u32 = 8000000;
pub(crate) const CHIP_FREQ_MAINCK_RC_12MHZ: u32 = 12000000;
pub(crate) const CHIP_FREQ_CPU_MAX: u32 = 84000000;
pub(crate) const CHIP_FREQ_XTAL_32K: u32 = 32768;
pub(crate) const CHIP_FREQ_XTAL_12M: u32 = 12000000;
pub(crate) const CHIP_FREQ_UTMIPLL: u32 = 480000000;

impl Device<atsam3x8e::Peripherals> {
    pub(crate) fn init_flash_rw_cycle_times(&mut self) {
        // Set FWS according to SYS_BOARD_MCKR configuration
        self.p.EFC0.fmr.write(|w| unsafe { w.fws().bits(4) });
        self.p.EFC1.fmr.write(|w| unsafe { w.fws().bits(4) });
    }

    pub(crate) fn enable_main_oscillator(&mut self) {
        // startup cycles = 8 * startup_time / SLCK
        let startup_time = 8u8;
        // Enable xtal oscillator while keeping internal oscillator enabled
        self.p.PMC.ckgr_mor.write(|w| unsafe {
            w.key()
                .passwd()
                .moscxtst()
                .bits(startup_time)
                .moscrcen()
                .set_bit()
                .moscxten()
                .set_bit()
        });

        // Wait until xtal oscillator reports ready
        // 0 = not stabilized, 1 = stabilized
        while !self.p.PMC.pmc_sr.read().moscxts().bits() {
            Self::nop()
        }

        // Select xtal oscillator
        self.p.PMC.ckgr_mor.write(|w| unsafe {
            w.key()
                .passwd()
                .moscxtst()
                .bits(startup_time)
                .moscrcen()
                .set_bit()
                .moscxten()
                .set_bit()
                .moscsel()
                .set_bit()
        });

        // Wait until oscillator selection reports ready
        // 0 = done, 1 = in progress
        while self.p.PMC.pmc_sr.read().moscsels().bits() {
            Self::nop()
        }
    }

    pub(crate) fn enable_pll(&mut self) {
        // Enable pll (12 MHz * 14 = 84MHz)
        let multiplier: u16 = 14;
        let divider: u8 = 1;
        let startup_time: u8 = 0x3f;
        self.p.PMC.ckgr_pllar.write(|w| unsafe {
            w.one()
                .set_bit()
                .mula()
                .bits(multiplier - 1)
                .pllacount()
                .bits(startup_time)
                .diva()
                .bits(divider)
        });

        // Wait until pll is locked
        // 0 = not locked, 1 = locked
        while !self.p.PMC.pmc_sr.read().locka().bits() {
            Self::nop()
        }
    }

    pub(crate) fn set_pll_as_main_clock(&mut self) {
        // Set main clock as master clock, then plla as master clock (two step process)
        // Set main clock
        self.p
            .PMC
            .pmc_mckr
            .write(|w| w.css().main_clk().plladiv2().set_bit());

        // Wait until master clock reports ready
        // 0 = not ready, 1 = ready
        while !self.p.PMC.pmc_sr.read().mckrdy().bits() {
            Self::nop()
        }

        // Set plla clock
        self.p
            .PMC
            .pmc_mckr
            .write(|w| w.css().plla_clk().plladiv2().set_bit());

        // Wait until master clock reports ready
        // 0 = not ready, 1 = ready
        while !self.p.PMC.pmc_sr.read().mckrdy().bits() {
            Self::nop()
        }
        self.syscore = CHIP_FREQ_CPU_MAX;
    }

    fn get_main_clock_rate(&self) -> u32 {
        if self.p.PMC.ckgr_mor.read().moscsel().bits() {
            CHIP_FREQ_XTAL_12M
        } else {
            match self.p.PMC.ckgr_mor.read().moscrcf().variant() {
                Variant::Val(ckgr_mor::MOSCRCF_A::_4_MHZ) => CHIP_FREQ_MAINCK_RC_4MHZ,
                Variant::Val(ckgr_mor::MOSCRCF_A::_8_MHZ) => CHIP_FREQ_MAINCK_RC_8MHZ,
                Variant::Val(ckgr_mor::MOSCRCF_A::_12_MHZ) => CHIP_FREQ_MAINCK_RC_12MHZ,
                // TODO:
                Variant::Res(_) => CHIP_FREQ_MAINCK_RC_4MHZ,
            }
        }
    }

    pub(crate) fn update_syscore(&mut self) {
        /* Determine clock frequency according to clock register values */
        let clk_unscaled = match self.p.PMC.pmc_mckr.read().css().variant() {
            pmc_mckr::CSS_A::SLOW_CLK => match self.p.SUPC.sr.read().oscsel().variant() {
                supc::sr::OSCSEL_A::RC => CHIP_FREQ_SLCK_RC,
                supc::sr::OSCSEL_A::CRYST => CHIP_FREQ_XTAL_32K,
            },
            pmc_mckr::CSS_A::MAIN_CLK => self.get_main_clock_rate(),
            pmc_mckr::CSS_A::PLLA_CLK => {
                let mut tmp_clk = self.get_main_clock_rate();
                tmp_clk *= (self.p.PMC.ckgr_pllar.read().mula().bits() + 1) as u32;
                tmp_clk /= self.p.PMC.ckgr_pllar.read().diva().bits() as u32;
                tmp_clk
            }
            pmc_mckr::CSS_A::UPLL_CLK => CHIP_FREQ_UTMIPLL / 2,
        };
        self.syscore = match self.p.PMC.pmc_mckr.read().pres().variant() {
            pmc_mckr::PRES_A::CLK_3 => clk_unscaled / 3,
            x => clk_unscaled >> (x as u8),
        };
    }
}
