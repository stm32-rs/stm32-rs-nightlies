#[repr(C)]
#[doc = "Register block"]
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
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - RCC HSI calibration register"]
    #[inline(always)]
    pub const fn hsicfgr(&self) -> &HSICFGR {
        &self.hsicfgr
    }
    #[doc = "0x08 - RCC clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    #[doc = "0x0c - RCC CSI calibration register"]
    #[inline(always)]
    pub const fn csicfgr(&self) -> &CSICFGR {
        &self.csicfgr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn cdcfgr1(&self) -> &CDCFGR1 {
        &self.cdcfgr1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn cdcfgr2(&self) -> &CDCFGR2 {
        &self.cdcfgr2
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn srdcfgr(&self) -> &SRDCFGR {
        &self.srdcfgr
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn pllckselr(&self) -> &PLLCKSELR {
        &self.pllckselr
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &PLLCFGR {
        &self.pllcfgr
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn pll1divr(&self) -> &PLL1DIVR {
        &self.pll1divr
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &PLL1FRACR {
        &self.pll1fracr
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn pll2divr(&self) -> &PLL2DIVR {
        &self.pll2divr
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &PLL2FRACR {
        &self.pll2fracr
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn pll3divr(&self) -> &PLL3DIVR {
        &self.pll3divr
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn pll3fracr(&self) -> &PLL3FRACR {
        &self.pll3fracr
    }
    #[doc = "0x4c - RCC CPU domain kernel clock configuration register"]
    #[inline(always)]
    pub const fn cdccipr(&self) -> &CDCCIPR {
        &self.cdccipr
    }
    #[doc = "0x50 - RCC CPU domain kernel clock configuration register"]
    #[inline(always)]
    pub const fn cdccip1r(&self) -> &CDCCIP1R {
        &self.cdccip1r
    }
    #[doc = "0x54 - RCC CPU domain kernel clock configuration register"]
    #[inline(always)]
    pub const fn cdccip2r(&self) -> &CDCCIP2R {
        &self.cdccip2r
    }
    #[doc = "0x58 - RCC SmartRun domain kernel clock configuration register"]
    #[inline(always)]
    pub const fn srdccipr(&self) -> &SRDCCIPR {
        &self.srdccipr
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    #[doc = "0x70 - RCC Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    #[doc = "0x74 - RCC clock control and status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn ahb4rstr(&self) -> &AHB4RSTR {
        &self.ahb4rstr
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn apb3rstr(&self) -> &APB3RSTR {
        &self.apb3rstr
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn apb1lrstr(&self) -> &APB1LRSTR {
        &self.apb1lrstr
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn apb1hrstr(&self) -> &APB1HRSTR {
        &self.apb1hrstr
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn apb4rstr(&self) -> &APB4RSTR {
        &self.apb4rstr
    }
    #[doc = "0xa0 - Global Control Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0xa8 - RCC SmartRun domain Autonomous mode register"]
    #[inline(always)]
    pub const fn srdamr(&self) -> &SRDAMR {
        &self.srdamr
    }
    #[doc = "0xb0 - RCC AXI clocks gating enable register"]
    #[inline(always)]
    pub const fn ckgaenr(&self) -> &CKGAENR {
        &self.ckgaenr
    }
    #[doc = "0x130 - RCC reset status register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        &self.rsr
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn ahb4enr(&self) -> &AHB4ENR {
        &self.ahb4enr
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn apb3enr(&self) -> &APB3ENR {
        &self.apb3enr
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn apb1lenr(&self) -> &APB1LENR {
        &self.apb1lenr
    }
    #[doc = "0x14c - "]
    #[inline(always)]
    pub const fn apb1henr(&self) -> &APB1HENR {
        &self.apb1henr
    }
    #[doc = "0x150 - "]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    #[doc = "0x154 - "]
    #[inline(always)]
    pub const fn apb4enr(&self) -> &APB4ENR {
        &self.apb4enr
    }
    #[doc = "0x15c - "]
    #[inline(always)]
    pub const fn ahb3lpenr(&self) -> &AHB3LPENR {
        &self.ahb3lpenr
    }
    #[doc = "0x160 - "]
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &AHB1LPENR {
        &self.ahb1lpenr
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &AHB2LPENR {
        &self.ahb2lpenr
    }
    #[doc = "0x168 - "]
    #[inline(always)]
    pub const fn ahb4lpenr(&self) -> &AHB4LPENR {
        &self.ahb4lpenr
    }
    #[doc = "0x16c - "]
    #[inline(always)]
    pub const fn apb3lpenr(&self) -> &APB3LPENR {
        &self.apb3lpenr
    }
    #[doc = "0x170 - "]
    #[inline(always)]
    pub const fn apb1llpenr(&self) -> &APB1LLPENR {
        &self.apb1llpenr
    }
    #[doc = "0x174 - "]
    #[inline(always)]
    pub const fn apb1hlpenr(&self) -> &APB1HLPENR {
        &self.apb1hlpenr
    }
    #[doc = "0x178 - "]
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    #[doc = "0x17c - "]
    #[inline(always)]
    pub const fn apb4lpenr(&self) -> &APB4LPENR {
        &self.apb4lpenr
    }
}
#[doc = "CR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = ""]
pub mod cr;
#[doc = "HSICFGR (rw) register accessor: RCC HSI calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsicfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsicfgr`]
module"]
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGRrs>;
#[doc = "RCC HSI calibration register"]
pub mod hsicfgr;
#[doc = "CRRCR (r) register accessor: RCC clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crrcr`]
module"]
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
#[doc = "RCC clock recovery RC register"]
pub mod crrcr;
#[doc = "CSICFGR (rw) register accessor: RCC CSI calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csicfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csicfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csicfgr`]
module"]
pub type CSICFGR = crate::Reg<csicfgr::CSICFGRrs>;
#[doc = "RCC CSI calibration register"]
pub mod csicfgr;
#[doc = "CFGR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = ""]
pub mod cfgr;
#[doc = "CDCFGR1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdcfgr1`]
module"]
pub type CDCFGR1 = crate::Reg<cdcfgr1::CDCFGR1rs>;
#[doc = ""]
pub mod cdcfgr1;
#[doc = "CDCFGR2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdcfgr2`]
module"]
pub type CDCFGR2 = crate::Reg<cdcfgr2::CDCFGR2rs>;
#[doc = ""]
pub mod cdcfgr2;
#[doc = "SRDCFGR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srdcfgr`]
module"]
pub type SRDCFGR = crate::Reg<srdcfgr::SRDCFGRrs>;
#[doc = ""]
pub mod srdcfgr;
#[doc = "PLLCKSELR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllckselr`]
module"]
pub type PLLCKSELR = crate::Reg<pllckselr::PLLCKSELRrs>;
#[doc = ""]
pub mod pllckselr;
#[doc = "PLLCFGR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`]
module"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
#[doc = ""]
pub mod pllcfgr;
#[doc = "PLL1DIVR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1divr`]
module"]
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVRrs>;
#[doc = ""]
pub mod pll1divr;
#[doc = "PLL1FRACR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1fracr`]
module"]
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
#[doc = ""]
pub mod pll1fracr;
#[doc = "PLL2DIVR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2divr`]
module"]
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVRrs>;
#[doc = ""]
pub mod pll2divr;
#[doc = "PLL2FRACR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2fracr`]
module"]
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACRrs>;
#[doc = ""]
pub mod pll2fracr;
#[doc = "PLL3DIVR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll3divr`]
module"]
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVRrs>;
#[doc = ""]
pub mod pll3divr;
#[doc = "PLL3FRACR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll3fracr`]
module"]
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACRrs>;
#[doc = ""]
pub mod pll3fracr;
#[doc = "CDCCIPR (rw) register accessor: RCC CPU domain kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdccipr`]
module"]
pub type CDCCIPR = crate::Reg<cdccipr::CDCCIPRrs>;
#[doc = "RCC CPU domain kernel clock configuration register"]
pub mod cdccipr;
#[doc = "CDCCIP1R (rw) register accessor: RCC CPU domain kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdccip1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdccip1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdccip1r`]
module"]
pub type CDCCIP1R = crate::Reg<cdccip1r::CDCCIP1Rrs>;
#[doc = "RCC CPU domain kernel clock configuration register"]
pub mod cdccip1r;
#[doc = "CDCCIP2R (rw) register accessor: RCC CPU domain kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdccip2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdccip2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdccip2r`]
module"]
pub type CDCCIP2R = crate::Reg<cdccip2r::CDCCIP2Rrs>;
#[doc = "RCC CPU domain kernel clock configuration register"]
pub mod cdccip2r;
#[doc = "SRDCCIPR (rw) register accessor: RCC SmartRun domain kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srdccipr`]
module"]
pub type SRDCCIPR = crate::Reg<srdccipr::SRDCCIPRrs>;
#[doc = "RCC SmartRun domain kernel clock configuration register"]
pub mod srdccipr;
#[doc = "CIER (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`]
module"]
pub type CIER = crate::Reg<cier::CIERrs>;
#[doc = ""]
pub mod cier;
#[doc = "CIFR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`]
module"]
pub type CIFR = crate::Reg<cifr::CIFRrs>;
#[doc = ""]
pub mod cifr;
#[doc = "CICR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`]
module"]
pub type CICR = crate::Reg<cicr::CICRrs>;
#[doc = ""]
pub mod cicr;
#[doc = "BDCR (rw) register accessor: RCC Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
#[doc = "RCC Backup domain control register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: RCC clock control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "RCC clock control and status register"]
pub mod csr;
#[doc = "AHB3RSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3rstr`]
module"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
#[doc = ""]
pub mod ahb3rstr;
#[doc = "AHB1RSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`]
module"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
#[doc = ""]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`]
module"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
#[doc = ""]
pub mod ahb2rstr;
#[doc = "AHB4RSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4rstr`]
module"]
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTRrs>;
#[doc = ""]
pub mod ahb4rstr;
#[doc = "APB3RSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3rstr`]
module"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTRrs>;
#[doc = ""]
pub mod apb3rstr;
#[doc = "APB1LRSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lrstr`]
module"]
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTRrs>;
#[doc = ""]
pub mod apb1lrstr;
#[doc = "APB1HRSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hrstr`]
module"]
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTRrs>;
#[doc = ""]
pub mod apb1hrstr;
#[doc = "APB2RSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`]
module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
#[doc = ""]
pub mod apb2rstr;
#[doc = "APB4RSTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4rstr`]
module"]
pub type APB4RSTR = crate::Reg<apb4rstr::APB4RSTRrs>;
#[doc = ""]
pub mod apb4rstr;
#[doc = "SRDAMR (rw) register accessor: RCC SmartRun domain Autonomous mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdamr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdamr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srdamr`]
module"]
pub type SRDAMR = crate::Reg<srdamr::SRDAMRrs>;
#[doc = "RCC SmartRun domain Autonomous mode register"]
pub mod srdamr;
#[doc = "CKGAENR (rw) register accessor: RCC AXI clocks gating enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgaenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgaenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgaenr`]
module"]
pub type CKGAENR = crate::Reg<ckgaenr::CKGAENRrs>;
#[doc = "RCC AXI clocks gating enable register"]
pub mod ckgaenr;
#[doc = "RSR (rw) register accessor: RCC reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
pub type RSR = crate::Reg<rsr::RSRrs>;
#[doc = "RCC reset status register"]
pub mod rsr;
#[doc = "AHB3ENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3enr`]
module"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
#[doc = ""]
pub mod ahb3enr;
#[doc = "AHB1ENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`]
module"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
#[doc = ""]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`]
module"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
#[doc = ""]
pub mod ahb2enr;
#[doc = "AHB4ENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4enr`]
module"]
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENRrs>;
#[doc = ""]
pub mod ahb4enr;
#[doc = "APB3ENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3enr`]
module"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENRrs>;
#[doc = ""]
pub mod apb3enr;
#[doc = "APB1LENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lenr`]
module"]
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENRrs>;
#[doc = ""]
pub mod apb1lenr;
#[doc = "APB1HENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1henr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1henr`]
module"]
pub type APB1HENR = crate::Reg<apb1henr::APB1HENRrs>;
#[doc = ""]
pub mod apb1henr;
#[doc = "APB2ENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`]
module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
#[doc = ""]
pub mod apb2enr;
#[doc = "APB4ENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4enr`]
module"]
pub type APB4ENR = crate::Reg<apb4enr::APB4ENRrs>;
#[doc = ""]
pub mod apb4enr;
#[doc = "AHB3LPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3lpenr`]
module"]
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENRrs>;
#[doc = ""]
pub mod ahb3lpenr;
#[doc = "AHB1LPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1lpenr`]
module"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENRrs>;
#[doc = ""]
pub mod ahb1lpenr;
#[doc = "AHB2LPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2lpenr`]
module"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENRrs>;
#[doc = ""]
pub mod ahb2lpenr;
#[doc = "AHB4LPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4lpenr`]
module"]
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENRrs>;
#[doc = ""]
pub mod ahb4lpenr;
#[doc = "APB3LPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3lpenr`]
module"]
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENRrs>;
#[doc = ""]
pub mod apb3lpenr;
#[doc = "APB1LLPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1llpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1llpenr`]
module"]
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENRrs>;
#[doc = ""]
pub mod apb1llpenr;
#[doc = "APB1HLPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hlpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hlpenr`]
module"]
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENRrs>;
#[doc = ""]
pub mod apb1hlpenr;
#[doc = "APB2LPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpenr`]
module"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
#[doc = ""]
pub mod apb2lpenr;
#[doc = "APB4LPENR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4lpenr`]
module"]
pub type APB4LPENR = crate::Reg<apb4lpenr::APB4LPENRrs>;
#[doc = ""]
pub mod apb4lpenr;
#[doc = "GCR (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCRrs>;
#[doc = "Global Control Register"]
pub mod gcr;
