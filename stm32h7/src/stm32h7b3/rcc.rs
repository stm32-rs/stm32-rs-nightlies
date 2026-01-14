#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    hsicfgr: HSICFGR,
    crrcr: CRRCR,
    csicfgr: CSICFGR,
    cfgr: CFGR,
    _reserved5: [u8; 0x04],
    cdcfgr1: CDCFGR1,
    cdcfgr2: CDCFGR2,
    srdcfgr: SRDCFGR,
    _reserved8: [u8; 0x04],
    pllckselr: PLLCKSELR,
    pllcfgr: PLLCFGR,
    pll1divr: PLL1DIVR,
    pll1fracr: PLL1FRACR,
    pll2divr: PLL2DIVR,
    pll2fracr: PLL2FRACR,
    pll3divr: PLL3DIVR,
    pll3fracr: PLL3FRACR,
    _reserved16: [u8; 0x04],
    cdccipr: CDCCIPR,
    cdccip1r: CDCCIP1R,
    cdccip2r: CDCCIP2R,
    srdccipr: SRDCCIPR,
    _reserved20: [u8; 0x04],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved23: [u8; 0x04],
    bdcr: BDCR,
    csr: CSR,
    _reserved25: [u8; 0x04],
    ahb3rstr: AHB3RSTR,
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    ahb4rstr: AHB4RSTR,
    apb3rstr: APB3RSTR,
    apb1lrstr: APB1LRSTR,
    apb1hrstr: APB1HRSTR,
    apb2rstr: APB2RSTR,
    apb4rstr: APB4RSTR,
    gcr: GCR,
    _reserved35: [u8; 0x04],
    srdamr: SRDAMR,
    _reserved36: [u8; 0x04],
    ckgaenr: CKGAENR,
    _reserved37: [u8; 0x7c],
    rsr: RSR,
    ahb3enr: AHB3ENR,
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    ahb4enr: AHB4ENR,
    apb3enr: APB3ENR,
    apb1lenr: APB1LENR,
    apb1henr: APB1HENR,
    apb2enr: APB2ENR,
    apb4enr: APB4ENR,
    _reserved47: [u8; 0x04],
    ahb3lpenr: AHB3LPENR,
    ahb1lpenr: AHB1LPENR,
    ahb2lpenr: AHB2LPENR,
    ahb4lpenr: AHB4LPENR,
    apb3lpenr: APB3LPENR,
    apb1llpenr: APB1LLPENR,
    apb1hlpenr: APB1HLPENR,
    apb2lpenr: APB2LPENR,
    apb4lpenr: APB4LPENR,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RCC HSI calibration register
    #[inline(always)]
    pub const fn hsicfgr(&self) -> &HSICFGR {
        &self.hsicfgr
    }
    ///0x08 - RCC clock recovery RC register
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    ///0x0c - RCC CSI calibration register
    #[inline(always)]
    pub const fn csicfgr(&self) -> &CSICFGR {
        &self.csicfgr
    }
    ///0x10 -
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x18 -
    #[inline(always)]
    pub const fn cdcfgr1(&self) -> &CDCFGR1 {
        &self.cdcfgr1
    }
    ///0x1c -
    #[inline(always)]
    pub const fn cdcfgr2(&self) -> &CDCFGR2 {
        &self.cdcfgr2
    }
    ///0x20 -
    #[inline(always)]
    pub const fn srdcfgr(&self) -> &SRDCFGR {
        &self.srdcfgr
    }
    ///0x28 -
    #[inline(always)]
    pub const fn pllckselr(&self) -> &PLLCKSELR {
        &self.pllckselr
    }
    ///0x2c -
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &PLLCFGR {
        &self.pllcfgr
    }
    ///0x30 -
    #[inline(always)]
    pub const fn pll1divr(&self) -> &PLL1DIVR {
        &self.pll1divr
    }
    ///0x34 -
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &PLL1FRACR {
        &self.pll1fracr
    }
    ///0x38 -
    #[inline(always)]
    pub const fn pll2divr(&self) -> &PLL2DIVR {
        &self.pll2divr
    }
    ///0x3c -
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &PLL2FRACR {
        &self.pll2fracr
    }
    ///0x40 -
    #[inline(always)]
    pub const fn pll3divr(&self) -> &PLL3DIVR {
        &self.pll3divr
    }
    ///0x44 -
    #[inline(always)]
    pub const fn pll3fracr(&self) -> &PLL3FRACR {
        &self.pll3fracr
    }
    ///0x4c - RCC CPU domain kernel clock configuration register
    #[inline(always)]
    pub const fn cdccipr(&self) -> &CDCCIPR {
        &self.cdccipr
    }
    ///0x50 - RCC CPU domain kernel clock configuration register
    #[inline(always)]
    pub const fn cdccip1r(&self) -> &CDCCIP1R {
        &self.cdccip1r
    }
    ///0x54 - RCC CPU domain kernel clock configuration register
    #[inline(always)]
    pub const fn cdccip2r(&self) -> &CDCCIP2R {
        &self.cdccip2r
    }
    ///0x58 - RCC SmartRun domain kernel clock configuration register
    #[inline(always)]
    pub const fn srdccipr(&self) -> &SRDCCIPR {
        &self.srdccipr
    }
    ///0x60 -
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x64 -
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x68 -
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x70 - RCC Backup domain control register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x74 - RCC clock control and status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x7c -
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    ///0x80 -
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    ///0x84 -
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    ///0x88 -
    #[inline(always)]
    pub const fn ahb4rstr(&self) -> &AHB4RSTR {
        &self.ahb4rstr
    }
    ///0x8c -
    #[inline(always)]
    pub const fn apb3rstr(&self) -> &APB3RSTR {
        &self.apb3rstr
    }
    ///0x90 -
    #[inline(always)]
    pub const fn apb1lrstr(&self) -> &APB1LRSTR {
        &self.apb1lrstr
    }
    ///0x94 -
    #[inline(always)]
    pub const fn apb1hrstr(&self) -> &APB1HRSTR {
        &self.apb1hrstr
    }
    ///0x98 -
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x9c -
    #[inline(always)]
    pub const fn apb4rstr(&self) -> &APB4RSTR {
        &self.apb4rstr
    }
    ///0xa0 - Global Control Register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0xa8 - RCC SmartRun domain Autonomous mode register
    #[inline(always)]
    pub const fn srdamr(&self) -> &SRDAMR {
        &self.srdamr
    }
    ///0xb0 - RCC AXI clocks gating enable register
    #[inline(always)]
    pub const fn ckgaenr(&self) -> &CKGAENR {
        &self.ckgaenr
    }
    ///0x130 - RCC reset status register
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        &self.rsr
    }
    ///0x134 -
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    ///0x138 -
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    ///0x13c -
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    ///0x140 -
    #[inline(always)]
    pub const fn ahb4enr(&self) -> &AHB4ENR {
        &self.ahb4enr
    }
    ///0x144 -
    #[inline(always)]
    pub const fn apb3enr(&self) -> &APB3ENR {
        &self.apb3enr
    }
    ///0x148 -
    #[inline(always)]
    pub const fn apb1lenr(&self) -> &APB1LENR {
        &self.apb1lenr
    }
    ///0x14c -
    #[inline(always)]
    pub const fn apb1henr(&self) -> &APB1HENR {
        &self.apb1henr
    }
    ///0x150 -
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0x154 -
    #[inline(always)]
    pub const fn apb4enr(&self) -> &APB4ENR {
        &self.apb4enr
    }
    ///0x15c -
    #[inline(always)]
    pub const fn ahb3lpenr(&self) -> &AHB3LPENR {
        &self.ahb3lpenr
    }
    ///0x160 -
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &AHB1LPENR {
        &self.ahb1lpenr
    }
    ///0x164 -
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &AHB2LPENR {
        &self.ahb2lpenr
    }
    ///0x168 -
    #[inline(always)]
    pub const fn ahb4lpenr(&self) -> &AHB4LPENR {
        &self.ahb4lpenr
    }
    ///0x16c -
    #[inline(always)]
    pub const fn apb3lpenr(&self) -> &APB3LPENR {
        &self.apb3lpenr
    }
    ///0x170 -
    #[inline(always)]
    pub const fn apb1llpenr(&self) -> &APB1LLPENR {
        &self.apb1llpenr
    }
    ///0x174 -
    #[inline(always)]
    pub const fn apb1hlpenr(&self) -> &APB1HLPENR {
        &self.apb1hlpenr
    }
    ///0x178 -
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    ///0x17c -
    #[inline(always)]
    pub const fn apb4lpenr(&self) -> &APB4LPENR {
        &self.apb4lpenr
    }
}
/**CR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///
pub mod cr;
/**HSICFGR (rw) register accessor: RCC HSI calibration register

You can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:HSICFGR)

For information about available fields see [`mod@hsicfgr`] module*/
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGRrs>;
///RCC HSI calibration register
pub mod hsicfgr;
/**CRRCR (r) register accessor: RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CRRCR)

For information about available fields see [`mod@crrcr`] module*/
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
///RCC clock recovery RC register
pub mod crrcr;
/**CSICFGR (rw) register accessor: RCC CSI calibration register

You can [`read`](crate::Reg::read) this register and get [`csicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CSICFGR)

For information about available fields see [`mod@csicfgr`] module*/
pub type CSICFGR = crate::Reg<csicfgr::CSICFGRrs>;
///RCC CSI calibration register
pub mod csicfgr;
/**CFGR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///
pub mod cfgr;
/**CDCFGR1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cdcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CDCFGR1)

For information about available fields see [`mod@cdcfgr1`] module*/
pub type CDCFGR1 = crate::Reg<cdcfgr1::CDCFGR1rs>;
///
pub mod cdcfgr1;
/**CDCFGR2 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cdcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CDCFGR2)

For information about available fields see [`mod@cdcfgr2`] module*/
pub type CDCFGR2 = crate::Reg<cdcfgr2::CDCFGR2rs>;
///
pub mod cdcfgr2;
/**SRDCFGR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`srdcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srdcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:SRDCFGR)

For information about available fields see [`mod@srdcfgr`] module*/
pub type SRDCFGR = crate::Reg<srdcfgr::SRDCFGRrs>;
///
pub mod srdcfgr;
/**PLLCKSELR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pllckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLLCKSELR)

For information about available fields see [`mod@pllckselr`] module*/
pub type PLLCKSELR = crate::Reg<pllckselr::PLLCKSELRrs>;
///
pub mod pllckselr;
/**PLLCFGR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLLCFGR)

For information about available fields see [`mod@pllcfgr`] module*/
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
///
pub mod pllcfgr;
/**PLL1DIVR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pll1divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLL1DIVR)

For information about available fields see [`mod@pll1divr`] module*/
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVRrs>;
///
pub mod pll1divr;
/**PLL1FRACR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLL1FRACR)

For information about available fields see [`mod@pll1fracr`] module*/
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
///
pub mod pll1fracr;
/**PLL2DIVR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pll2divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLL2DIVR)

For information about available fields see [`mod@pll2divr`] module*/
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVRrs>;
///
pub mod pll2divr;
/**PLL2FRACR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLL2FRACR)

For information about available fields see [`mod@pll2fracr`] module*/
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACRrs>;
///
pub mod pll2fracr;
/**PLL3DIVR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pll3divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLL3DIVR)

For information about available fields see [`mod@pll3divr`] module*/
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVRrs>;
///
pub mod pll3divr;
/**PLL3FRACR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pll3fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLL3FRACR)

For information about available fields see [`mod@pll3fracr`] module*/
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACRrs>;
///
pub mod pll3fracr;
/**CDCCIPR (rw) register accessor: RCC CPU domain kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cdccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CDCCIPR)

For information about available fields see [`mod@cdccipr`] module*/
pub type CDCCIPR = crate::Reg<cdccipr::CDCCIPRrs>;
///RCC CPU domain kernel clock configuration register
pub mod cdccipr;
/**CDCCIP1R (rw) register accessor: RCC CPU domain kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cdccip1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdccip1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CDCCIP1R)

For information about available fields see [`mod@cdccip1r`] module*/
pub type CDCCIP1R = crate::Reg<cdccip1r::CDCCIP1Rrs>;
///RCC CPU domain kernel clock configuration register
pub mod cdccip1r;
/**CDCCIP2R (rw) register accessor: RCC CPU domain kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cdccip2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdccip2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CDCCIP2R)

For information about available fields see [`mod@cdccip2r`] module*/
pub type CDCCIP2R = crate::Reg<cdccip2r::CDCCIP2Rrs>;
///RCC CPU domain kernel clock configuration register
pub mod cdccip2r;
/**SRDCCIPR (rw) register accessor: RCC SmartRun domain kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`srdccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srdccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:SRDCCIPR)

For information about available fields see [`mod@srdccipr`] module*/
pub type SRDCCIPR = crate::Reg<srdccipr::SRDCCIPRrs>;
///RCC SmartRun domain kernel clock configuration register
pub mod srdccipr;
/**CIER (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///
pub mod cier;
/**CIFR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///
pub mod cifr;
/**CICR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///
pub mod cicr;
/**BDCR (rw) register accessor: RCC Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///RCC Backup domain control register
pub mod bdcr;
/**CSR (rw) register accessor: RCC clock control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///RCC clock control and status register
pub mod csr;
/**AHB3RSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB3RSTR)

For information about available fields see [`mod@ahb3rstr`] module*/
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
///
pub mod ahb3rstr;
/**AHB1RSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB1RSTR)

For information about available fields see [`mod@ahb1rstr`] module*/
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
///
pub mod ahb1rstr;
/**AHB2RSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB2RSTR)

For information about available fields see [`mod@ahb2rstr`] module*/
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
///
pub mod ahb2rstr;
/**AHB4RSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB4RSTR)

For information about available fields see [`mod@ahb4rstr`] module*/
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTRrs>;
///
pub mod ahb4rstr;
/**APB3RSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB3RSTR)

For information about available fields see [`mod@apb3rstr`] module*/
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTRrs>;
///
pub mod apb3rstr;
/**APB1LRSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb1lrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB1LRSTR)

For information about available fields see [`mod@apb1lrstr`] module*/
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTRrs>;
///
pub mod apb1lrstr;
/**APB1HRSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB1HRSTR)

For information about available fields see [`mod@apb1hrstr`] module*/
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTRrs>;
///
pub mod apb1hrstr;
/**APB2RSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///
pub mod apb2rstr;
/**APB4RSTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB4RSTR)

For information about available fields see [`mod@apb4rstr`] module*/
pub type APB4RSTR = crate::Reg<apb4rstr::APB4RSTRrs>;
///
pub mod apb4rstr;
/**GCR (rw) register accessor: Global Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///Global Control Register
pub mod gcr;
/**SRDAMR (rw) register accessor: RCC SmartRun domain Autonomous mode register

You can [`read`](crate::Reg::read) this register and get [`srdamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srdamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:SRDAMR)

For information about available fields see [`mod@srdamr`] module*/
pub type SRDAMR = crate::Reg<srdamr::SRDAMRrs>;
///RCC SmartRun domain Autonomous mode register
pub mod srdamr;
/**CKGAENR (rw) register accessor: RCC AXI clocks gating enable register

You can [`read`](crate::Reg::read) this register and get [`ckgaenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgaenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:CKGAENR)

For information about available fields see [`mod@ckgaenr`] module*/
pub type CKGAENR = crate::Reg<ckgaenr::CKGAENRrs>;
///RCC AXI clocks gating enable register
pub mod ckgaenr;
/**RSR (rw) register accessor: RCC reset status register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:RSR)

For information about available fields see [`mod@rsr`] module*/
pub type RSR = crate::Reg<rsr::RSRrs>;
///RCC reset status register
pub mod rsr;
/**AHB3ENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB3ENR)

For information about available fields see [`mod@ahb3enr`] module*/
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
///
pub mod ahb3enr;
/**AHB1ENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB1ENR)

For information about available fields see [`mod@ahb1enr`] module*/
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
///
pub mod ahb1enr;
/**AHB2ENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB2ENR)

For information about available fields see [`mod@ahb2enr`] module*/
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
///
pub mod ahb2enr;
/**AHB4ENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB4ENR)

For information about available fields see [`mod@ahb4enr`] module*/
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENRrs>;
///
pub mod ahb4enr;
/**APB3ENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB3ENR)

For information about available fields see [`mod@apb3enr`] module*/
pub type APB3ENR = crate::Reg<apb3enr::APB3ENRrs>;
///
pub mod apb3enr;
/**APB1LENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb1lenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB1LENR)

For information about available fields see [`mod@apb1lenr`] module*/
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENRrs>;
///
pub mod apb1lenr;
/**APB1HENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb1henr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB1HENR)

For information about available fields see [`mod@apb1henr`] module*/
pub type APB1HENR = crate::Reg<apb1henr::APB1HENRrs>;
///
pub mod apb1henr;
/**APB2ENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///
pub mod apb2enr;
/**APB4ENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB4ENR)

For information about available fields see [`mod@apb4enr`] module*/
pub type APB4ENR = crate::Reg<apb4enr::APB4ENRrs>;
///
pub mod apb4enr;
/**AHB3LPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB3LPENR)

For information about available fields see [`mod@ahb3lpenr`] module*/
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENRrs>;
///
pub mod ahb3lpenr;
/**AHB1LPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB1LPENR)

For information about available fields see [`mod@ahb1lpenr`] module*/
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENRrs>;
///
pub mod ahb1lpenr;
/**AHB2LPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB2LPENR)

For information about available fields see [`mod@ahb2lpenr`] module*/
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENRrs>;
///
pub mod ahb2lpenr;
/**AHB4LPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB4LPENR)

For information about available fields see [`mod@ahb4lpenr`] module*/
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENRrs>;
///
pub mod ahb4lpenr;
/**APB3LPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB3LPENR)

For information about available fields see [`mod@apb3lpenr`] module*/
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENRrs>;
///
pub mod apb3lpenr;
/**APB1LLPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb1llpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB1LLPENR)

For information about available fields see [`mod@apb1llpenr`] module*/
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENRrs>;
///
pub mod apb1llpenr;
/**APB1HLPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB1HLPENR)

For information about available fields see [`mod@apb1hlpenr`] module*/
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENRrs>;
///
pub mod apb1hlpenr;
/**APB2LPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB2LPENR)

For information about available fields see [`mod@apb2lpenr`] module*/
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
///
pub mod apb2lpenr;
/**APB4LPENR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`apb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:APB4LPENR)

For information about available fields see [`mod@apb4lpenr`] module*/
pub type APB4LPENR = crate::Reg<apb4lpenr::APB4LPENRrs>;
///
pub mod apb4lpenr;
