#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    crrcr: CRRCR,
    _reserved3: [u8; 0x04],
    cfgr: CFGR,
    _reserved4: [u8; 0x04],
    d1cfgr: D1CFGR,
    d2cfgr: D2CFGR,
    d3cfgr: D3CFGR,
    _reserved7: [u8; 0x04],
    pllckselr: PLLCKSELR,
    pllcfgr: PLLCFGR,
    pll1divr: PLL1DIVR,
    pll1fracr: PLL1FRACR,
    pll2divr: PLL2DIVR,
    pll2fracr: PLL2FRACR,
    pll3divr: PLL3DIVR,
    pll3fracr: PLL3FRACR,
    _reserved15: [u8; 0x04],
    d1ccipr: D1CCIPR,
    d2ccip1r: D2CCIP1R,
    d2ccip2r: D2CCIP2R,
    d3ccipr: D3CCIPR,
    _reserved19: [u8; 0x04],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved22: [u8; 0x04],
    bdcr: BDCR,
    csr: CSR,
    _reserved24: [u8; 0x04],
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
    _reserved34: [u8; 0x04],
    d3amr: D3AMR,
    _reserved35: [u8; 0x24],
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
    _reserved45: [u8; 0x04],
    ahb3lpenr: AHB3LPENR,
    ahb1lpenr: AHB1LPENR,
    ahb2lpenr: AHB2LPENR,
    ahb4lpenr: AHB4LPENR,
    apb3lpenr: APB3LPENR,
    apb1llpenr: APB1LLPENR,
    apb1hlpenr: APB1HLPENR,
    apb2lpenr: APB2LPENR,
    apb4lpenr: APB4LPENR,
    _reserved54: [u8; 0x10],
    c1_rsr: C1_RSR,
    c1_ahb3enr: C1_AHB3ENR,
    c1_ahb1enr: C1_AHB1ENR,
    c1_ahb2enr: C1_AHB2ENR,
    c1_ahb4enr: C1_AHB4ENR,
    c1_apb3enr: C1_APB3ENR,
    c1_apb1lenr: C1_APB1LENR,
    c1_apb1henr: C1_APB1HENR,
    c1_apb2enr: C1_APB2ENR,
    c1_apb4enr: C1_APB4ENR,
    _reserved64: [u8; 0x04],
    c1_ahb3lpenr: C1_AHB3LPENR,
    c1_ahb1lpenr: C1_AHB1LPENR,
    c1_ahb2lpenr: C1_AHB2LPENR,
    c1_ahb4lpenr: C1_AHB4LPENR,
    c1_apb3lpenr: C1_APB3LPENR,
    c1_apb1llpenr: C1_APB1LLPENR,
    c1_apb1hlpenr: C1_APB1HLPENR,
    c1_apb2lpenr: C1_APB2LPENR,
    c1_apb4lpenr: C1_APB4LPENR,
}
impl RegisterBlock {
    #[doc = "0x00 - clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - RCC Internal Clock Source Calibration Register"]
    #[inline(always)]
    pub const fn icscr(&self) -> &ICSCR {
        &self.icscr
    }
    #[doc = "0x08 - RCC Clock Recovery RC Register"]
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    #[doc = "0x10 - RCC Clock Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x18 - RCC Domain 1 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1cfgr(&self) -> &D1CFGR {
        &self.d1cfgr
    }
    #[doc = "0x1c - RCC Domain 2 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2cfgr(&self) -> &D2CFGR {
        &self.d2cfgr
    }
    #[doc = "0x20 - RCC Domain 3 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3cfgr(&self) -> &D3CFGR {
        &self.d3cfgr
    }
    #[doc = "0x28 - RCC PLLs Clock Source Selection Register"]
    #[inline(always)]
    pub const fn pllckselr(&self) -> &PLLCKSELR {
        &self.pllckselr
    }
    #[doc = "0x2c - RCC PLLs Configuration Register"]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &PLLCFGR {
        &self.pllcfgr
    }
    #[doc = "0x30 - RCC PLL1 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn pll1divr(&self) -> &PLL1DIVR {
        &self.pll1divr
    }
    #[doc = "0x34 - RCC PLL1 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &PLL1FRACR {
        &self.pll1fracr
    }
    #[doc = "0x38 - RCC PLL2 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn pll2divr(&self) -> &PLL2DIVR {
        &self.pll2divr
    }
    #[doc = "0x3c - RCC PLL2 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &PLL2FRACR {
        &self.pll2fracr
    }
    #[doc = "0x40 - RCC PLL3 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn pll3divr(&self) -> &PLL3DIVR {
        &self.pll3divr
    }
    #[doc = "0x44 - RCC PLL3 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pll3fracr(&self) -> &PLL3FRACR {
        &self.pll3fracr
    }
    #[doc = "0x4c - RCC Domain 1 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1ccipr(&self) -> &D1CCIPR {
        &self.d1ccipr
    }
    #[doc = "0x50 - RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip1r(&self) -> &D2CCIP1R {
        &self.d2ccip1r
    }
    #[doc = "0x54 - RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip2r(&self) -> &D2CCIP2R {
        &self.d2ccip2r
    }
    #[doc = "0x58 - RCC Domain 3 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3ccipr(&self) -> &D3CCIPR {
        &self.d3ccipr
    }
    #[doc = "0x60 - RCC Clock Source Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    #[doc = "0x64 - RCC Clock Source Interrupt Flag Register"]
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    #[doc = "0x68 - RCC Clock Source Interrupt Clear Register"]
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    #[doc = "0x70 - RCC Backup Domain Control Register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    #[doc = "0x74 - RCC Clock Control and Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x7c - RCC AHB3 Reset Register"]
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    #[doc = "0x80 - RCC AHB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    #[doc = "0x84 - RCC AHB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    #[doc = "0x88 - RCC AHB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb4rstr(&self) -> &AHB4RSTR {
        &self.ahb4rstr
    }
    #[doc = "0x8c - RCC APB3 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb3rstr(&self) -> &APB3RSTR {
        &self.apb3rstr
    }
    #[doc = "0x90 - RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1lrstr(&self) -> &APB1LRSTR {
        &self.apb1lrstr
    }
    #[doc = "0x94 - RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1hrstr(&self) -> &APB1HRSTR {
        &self.apb1hrstr
    }
    #[doc = "0x98 - RCC APB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    #[doc = "0x9c - RCC APB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb4rstr(&self) -> &APB4RSTR {
        &self.apb4rstr
    }
    #[doc = "0xa0 - RCC Global Control Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0xa8 - RCC D3 Autonomous mode Register"]
    #[inline(always)]
    pub const fn d3amr(&self) -> &D3AMR {
        &self.d3amr
    }
    #[doc = "0xd0 - RCC Reset Status Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        &self.rsr
    }
    #[doc = "0xd4 - RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    #[doc = "0xd8 - RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    #[doc = "0xdc - RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    #[doc = "0xe0 - RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn ahb4enr(&self) -> &AHB4ENR {
        &self.ahb4enr
    }
    #[doc = "0xe4 - RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn apb3enr(&self) -> &APB3ENR {
        &self.apb3enr
    }
    #[doc = "0xe8 - RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1lenr(&self) -> &APB1LENR {
        &self.apb1lenr
    }
    #[doc = "0xec - RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1henr(&self) -> &APB1HENR {
        &self.apb1henr
    }
    #[doc = "0xf0 - RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    #[doc = "0xf4 - RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn apb4enr(&self) -> &APB4ENR {
        &self.apb4enr
    }
    #[doc = "0xfc - RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb3lpenr(&self) -> &AHB3LPENR {
        &self.ahb3lpenr
    }
    #[doc = "0x100 - RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &AHB1LPENR {
        &self.ahb1lpenr
    }
    #[doc = "0x104 - RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &AHB2LPENR {
        &self.ahb2lpenr
    }
    #[doc = "0x108 - RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb4lpenr(&self) -> &AHB4LPENR {
        &self.ahb4lpenr
    }
    #[doc = "0x10c - RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb3lpenr(&self) -> &APB3LPENR {
        &self.apb3lpenr
    }
    #[doc = "0x110 - RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1llpenr(&self) -> &APB1LLPENR {
        &self.apb1llpenr
    }
    #[doc = "0x114 - RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1hlpenr(&self) -> &APB1HLPENR {
        &self.apb1hlpenr
    }
    #[doc = "0x118 - RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    #[doc = "0x11c - RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb4lpenr(&self) -> &APB4LPENR {
        &self.apb4lpenr
    }
    #[doc = "0x130 - RCC Reset Status Register"]
    #[inline(always)]
    pub const fn c1_rsr(&self) -> &C1_RSR {
        &self.c1_rsr
    }
    #[doc = "0x134 - RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb3enr(&self) -> &C1_AHB3ENR {
        &self.c1_ahb3enr
    }
    #[doc = "0x138 - RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb1enr(&self) -> &C1_AHB1ENR {
        &self.c1_ahb1enr
    }
    #[doc = "0x13c - RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb2enr(&self) -> &C1_AHB2ENR {
        &self.c1_ahb2enr
    }
    #[doc = "0x140 - RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb4enr(&self) -> &C1_AHB4ENR {
        &self.c1_ahb4enr
    }
    #[doc = "0x144 - RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb3enr(&self) -> &C1_APB3ENR {
        &self.c1_apb3enr
    }
    #[doc = "0x148 - RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1lenr(&self) -> &C1_APB1LENR {
        &self.c1_apb1lenr
    }
    #[doc = "0x14c - RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1henr(&self) -> &C1_APB1HENR {
        &self.c1_apb1henr
    }
    #[doc = "0x150 - RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb2enr(&self) -> &C1_APB2ENR {
        &self.c1_apb2enr
    }
    #[doc = "0x154 - RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb4enr(&self) -> &C1_APB4ENR {
        &self.c1_apb4enr
    }
    #[doc = "0x15c - RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb3lpenr(&self) -> &C1_AHB3LPENR {
        &self.c1_ahb3lpenr
    }
    #[doc = "0x160 - RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb1lpenr(&self) -> &C1_AHB1LPENR {
        &self.c1_ahb1lpenr
    }
    #[doc = "0x164 - RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb2lpenr(&self) -> &C1_AHB2LPENR {
        &self.c1_ahb2lpenr
    }
    #[doc = "0x168 - RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb4lpenr(&self) -> &C1_AHB4LPENR {
        &self.c1_ahb4lpenr
    }
    #[doc = "0x16c - RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb3lpenr(&self) -> &C1_APB3LPENR {
        &self.c1_apb3lpenr
    }
    #[doc = "0x170 - RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1llpenr(&self) -> &C1_APB1LLPENR {
        &self.c1_apb1llpenr
    }
    #[doc = "0x174 - RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1hlpenr(&self) -> &C1_APB1HLPENR {
        &self.c1_apb1hlpenr
    }
    #[doc = "0x178 - RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb2lpenr(&self) -> &C1_APB2LPENR {
        &self.c1_apb2lpenr
    }
    #[doc = "0x17c - RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb4lpenr(&self) -> &C1_APB4LPENR {
        &self.c1_apb4lpenr
    }
}
#[doc = "CR (rw) register accessor: clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "clock control register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: RCC Internal Clock Source Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icscr`]
module"]
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
#[doc = "RCC Internal Clock Source Calibration Register"]
pub mod icscr;
#[doc = "CRRCR (r) register accessor: RCC Clock Recovery RC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crrcr`]
module"]
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
#[doc = "RCC Clock Recovery RC Register"]
pub mod crrcr;
#[doc = "CFGR (rw) register accessor: RCC Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "RCC Clock Configuration Register"]
pub mod cfgr;
#[doc = "D1CFGR (rw) register accessor: RCC Domain 1 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1cfgr`]
module"]
pub type D1CFGR = crate::Reg<d1cfgr::D1CFGRrs>;
#[doc = "RCC Domain 1 Clock Configuration Register"]
pub mod d1cfgr;
#[doc = "D2CFGR (rw) register accessor: RCC Domain 2 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2cfgr`]
module"]
pub type D2CFGR = crate::Reg<d2cfgr::D2CFGRrs>;
#[doc = "RCC Domain 2 Clock Configuration Register"]
pub mod d2cfgr;
#[doc = "D3CFGR (rw) register accessor: RCC Domain 3 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3cfgr`]
module"]
pub type D3CFGR = crate::Reg<d3cfgr::D3CFGRrs>;
#[doc = "RCC Domain 3 Clock Configuration Register"]
pub mod d3cfgr;
#[doc = "PLLCKSELR (rw) register accessor: RCC PLLs Clock Source Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllckselr`]
module"]
pub type PLLCKSELR = crate::Reg<pllckselr::PLLCKSELRrs>;
#[doc = "RCC PLLs Clock Source Selection Register"]
pub mod pllckselr;
#[doc = "PLLCFGR (rw) register accessor: RCC PLLs Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`]
module"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
#[doc = "RCC PLLs Configuration Register"]
pub mod pllcfgr;
#[doc = "PLL1DIVR (rw) register accessor: RCC PLL1 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1divr`]
module"]
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVRrs>;
#[doc = "RCC PLL1 Dividers Configuration Register"]
pub mod pll1divr;
#[doc = "PLL1FRACR (rw) register accessor: RCC PLL1 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1fracr`]
module"]
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
#[doc = "RCC PLL1 Fractional Divider Register"]
pub mod pll1fracr;
#[doc = "PLL2DIVR (rw) register accessor: RCC PLL2 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2divr`]
module"]
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVRrs>;
#[doc = "RCC PLL2 Dividers Configuration Register"]
pub mod pll2divr;
#[doc = "PLL2FRACR (rw) register accessor: RCC PLL2 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2fracr`]
module"]
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACRrs>;
#[doc = "RCC PLL2 Fractional Divider Register"]
pub mod pll2fracr;
#[doc = "PLL3DIVR (rw) register accessor: RCC PLL3 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll3divr`]
module"]
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVRrs>;
#[doc = "RCC PLL3 Dividers Configuration Register"]
pub mod pll3divr;
#[doc = "PLL3FRACR (rw) register accessor: RCC PLL3 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll3fracr`]
module"]
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACRrs>;
#[doc = "RCC PLL3 Fractional Divider Register"]
pub mod pll3fracr;
#[doc = "D1CCIPR (rw) register accessor: RCC Domain 1 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1ccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1ccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1ccipr`]
module"]
pub type D1CCIPR = crate::Reg<d1ccipr::D1CCIPRrs>;
#[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
pub mod d1ccipr;
#[doc = "D2CCIP1R (rw) register accessor: RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2ccip1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2ccip1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2ccip1r`]
module"]
pub type D2CCIP1R = crate::Reg<d2ccip1r::D2CCIP1Rrs>;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip1r;
#[doc = "D2CCIP2R (rw) register accessor: RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2ccip2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2ccip2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2ccip2r`]
module"]
pub type D2CCIP2R = crate::Reg<d2ccip2r::D2CCIP2Rrs>;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip2r;
#[doc = "D3CCIPR (rw) register accessor: RCC Domain 3 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3ccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3ccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3ccipr`]
module"]
pub type D3CCIPR = crate::Reg<d3ccipr::D3CCIPRrs>;
#[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
pub mod d3ccipr;
#[doc = "CIER (rw) register accessor: RCC Clock Source Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`]
module"]
pub type CIER = crate::Reg<cier::CIERrs>;
#[doc = "RCC Clock Source Interrupt Enable Register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: RCC Clock Source Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`]
module"]
pub type CIFR = crate::Reg<cifr::CIFRrs>;
#[doc = "RCC Clock Source Interrupt Flag Register"]
pub mod cifr;
#[doc = "CICR (rw) register accessor: RCC Clock Source Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`]
module"]
pub type CICR = crate::Reg<cicr::CICRrs>;
#[doc = "RCC Clock Source Interrupt Clear Register"]
pub mod cicr;
#[doc = "BDCR (rw) register accessor: RCC Backup Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
#[doc = "RCC Backup Domain Control Register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: RCC Clock Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "RCC Clock Control and Status Register"]
pub mod csr;
#[doc = "AHB3RSTR (rw) register accessor: RCC AHB3 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3rstr`]
module"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
#[doc = "RCC AHB3 Reset Register"]
pub mod ahb3rstr;
#[doc = "AHB1RSTR (rw) register accessor: RCC AHB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`]
module"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
#[doc = "RCC AHB1 Peripheral Reset Register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: RCC AHB2 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`]
module"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
#[doc = "RCC AHB2 Peripheral Reset Register"]
pub mod ahb2rstr;
#[doc = "AHB4RSTR (rw) register accessor: RCC AHB4 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4rstr`]
module"]
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTRrs>;
#[doc = "RCC AHB4 Peripheral Reset Register"]
pub mod ahb4rstr;
#[doc = "APB3RSTR (rw) register accessor: RCC APB3 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3rstr`]
module"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTRrs>;
#[doc = "RCC APB3 Peripheral Reset Register"]
pub mod apb3rstr;
#[doc = "APB1LRSTR (rw) register accessor: RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lrstr`]
module"]
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTRrs>;
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1lrstr;
#[doc = "APB1HRSTR (rw) register accessor: RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hrstr`]
module"]
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTRrs>;
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1hrstr;
#[doc = "APB2RSTR (rw) register accessor: RCC APB2 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`]
module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
#[doc = "RCC APB2 Peripheral Reset Register"]
pub mod apb2rstr;
#[doc = "APB4RSTR (rw) register accessor: RCC APB4 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4rstr`]
module"]
pub type APB4RSTR = crate::Reg<apb4rstr::APB4RSTRrs>;
#[doc = "RCC APB4 Peripheral Reset Register"]
pub mod apb4rstr;
#[doc = "GCR (rw) register accessor: RCC Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCRrs>;
#[doc = "RCC Global Control Register"]
pub mod gcr;
#[doc = "D3AMR (rw) register accessor: RCC D3 Autonomous mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3amr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3amr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3amr`]
module"]
pub type D3AMR = crate::Reg<d3amr::D3AMRrs>;
#[doc = "RCC D3 Autonomous mode Register"]
pub mod d3amr;
#[doc = "RSR (rw) register accessor: RCC Reset Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
pub type RSR = crate::Reg<rsr::RSRrs>;
#[doc = "RCC Reset Status Register"]
pub mod rsr;
#[doc = "C1_RSR (rw) register accessor: RCC Reset Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_rsr`]
module"]
pub type C1_RSR = crate::Reg<c1_rsr::C1_RSRrs>;
#[doc = "RCC Reset Status Register"]
pub mod c1_rsr;
#[doc = "C1_AHB3ENR (rw) register accessor: RCC AHB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb3enr`]
module"]
pub type C1_AHB3ENR = crate::Reg<c1_ahb3enr::C1_AHB3ENRrs>;
#[doc = "RCC AHB3 Clock Register"]
pub mod c1_ahb3enr;
#[doc = "AHB3ENR (rw) register accessor: RCC AHB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3enr`]
module"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
#[doc = "RCC AHB3 Clock Register"]
pub mod ahb3enr;
#[doc = "AHB1ENR (rw) register accessor: RCC AHB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`]
module"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
#[doc = "RCC AHB1 Clock Register"]
pub mod ahb1enr;
#[doc = "C1_AHB1ENR (rw) register accessor: RCC AHB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb1enr`]
module"]
pub type C1_AHB1ENR = crate::Reg<c1_ahb1enr::C1_AHB1ENRrs>;
#[doc = "RCC AHB1 Clock Register"]
pub mod c1_ahb1enr;
#[doc = "C1_AHB2ENR (rw) register accessor: RCC AHB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb2enr`]
module"]
pub type C1_AHB2ENR = crate::Reg<c1_ahb2enr::C1_AHB2ENRrs>;
#[doc = "RCC AHB2 Clock Register"]
pub mod c1_ahb2enr;
#[doc = "AHB2ENR (rw) register accessor: RCC AHB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`]
module"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
#[doc = "RCC AHB2 Clock Register"]
pub mod ahb2enr;
#[doc = "AHB4ENR (rw) register accessor: RCC AHB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4enr`]
module"]
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENRrs>;
#[doc = "RCC AHB4 Clock Register"]
pub mod ahb4enr;
#[doc = "C1_AHB4ENR (rw) register accessor: RCC AHB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb4enr`]
module"]
pub type C1_AHB4ENR = crate::Reg<c1_ahb4enr::C1_AHB4ENRrs>;
#[doc = "RCC AHB4 Clock Register"]
pub mod c1_ahb4enr;
#[doc = "C1_APB3ENR (rw) register accessor: RCC APB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb3enr`]
module"]
pub type C1_APB3ENR = crate::Reg<c1_apb3enr::C1_APB3ENRrs>;
#[doc = "RCC APB3 Clock Register"]
pub mod c1_apb3enr;
#[doc = "APB3ENR (rw) register accessor: RCC APB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3enr`]
module"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENRrs>;
#[doc = "RCC APB3 Clock Register"]
pub mod apb3enr;
#[doc = "APB1LENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lenr`]
module"]
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENRrs>;
#[doc = "RCC APB1 Clock Register"]
pub mod apb1lenr;
#[doc = "C1_APB1LENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1lenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1lenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb1lenr`]
module"]
pub type C1_APB1LENR = crate::Reg<c1_apb1lenr::C1_APB1LENRrs>;
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1lenr;
#[doc = "APB1HENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1henr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1henr`]
module"]
pub type APB1HENR = crate::Reg<apb1henr::APB1HENRrs>;
#[doc = "RCC APB1 Clock Register"]
pub mod apb1henr;
#[doc = "C1_APB1HENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1henr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1henr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb1henr`]
module"]
pub type C1_APB1HENR = crate::Reg<c1_apb1henr::C1_APB1HENRrs>;
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1henr;
#[doc = "C1_APB2ENR (rw) register accessor: RCC APB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb2enr`]
module"]
pub type C1_APB2ENR = crate::Reg<c1_apb2enr::C1_APB2ENRrs>;
#[doc = "RCC APB2 Clock Register"]
pub mod c1_apb2enr;
#[doc = "APB2ENR (rw) register accessor: RCC APB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`]
module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
#[doc = "RCC APB2 Clock Register"]
pub mod apb2enr;
#[doc = "APB4ENR (rw) register accessor: RCC APB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4enr`]
module"]
pub type APB4ENR = crate::Reg<apb4enr::APB4ENRrs>;
#[doc = "RCC APB4 Clock Register"]
pub mod apb4enr;
#[doc = "C1_APB4ENR (rw) register accessor: RCC APB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb4enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb4enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb4enr`]
module"]
pub type C1_APB4ENR = crate::Reg<c1_apb4enr::C1_APB4ENRrs>;
#[doc = "RCC APB4 Clock Register"]
pub mod c1_apb4enr;
#[doc = "C1_AHB3LPENR (rw) register accessor: RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb3lpenr`]
module"]
pub type C1_AHB3LPENR = crate::Reg<c1_ahb3lpenr::C1_AHB3LPENRrs>;
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod c1_ahb3lpenr;
#[doc = "AHB3LPENR (rw) register accessor: RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3lpenr`]
module"]
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENRrs>;
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod ahb3lpenr;
#[doc = "AHB1LPENR (rw) register accessor: RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1lpenr`]
module"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENRrs>;
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod ahb1lpenr;
#[doc = "C1_AHB1LPENR (rw) register accessor: RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb1lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb1lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb1lpenr`]
module"]
pub type C1_AHB1LPENR = crate::Reg<c1_ahb1lpenr::C1_AHB1LPENRrs>;
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod c1_ahb1lpenr;
#[doc = "C1_AHB2LPENR (rw) register accessor: RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb2lpenr`]
module"]
pub type C1_AHB2LPENR = crate::Reg<c1_ahb2lpenr::C1_AHB2LPENRrs>;
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod c1_ahb2lpenr;
#[doc = "AHB2LPENR (rw) register accessor: RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2lpenr`]
module"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENRrs>;
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod ahb2lpenr;
#[doc = "AHB4LPENR (rw) register accessor: RCC AHB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4lpenr`]
module"]
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENRrs>;
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod ahb4lpenr;
#[doc = "C1_AHB4LPENR (rw) register accessor: RCC AHB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb4lpenr`]
module"]
pub type C1_AHB4LPENR = crate::Reg<c1_ahb4lpenr::C1_AHB4LPENRrs>;
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod c1_ahb4lpenr;
#[doc = "C1_APB3LPENR (rw) register accessor: RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb3lpenr`]
module"]
pub type C1_APB3LPENR = crate::Reg<c1_apb3lpenr::C1_APB3LPENRrs>;
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod c1_apb3lpenr;
#[doc = "APB3LPENR (rw) register accessor: RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3lpenr`]
module"]
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENRrs>;
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod apb3lpenr;
#[doc = "APB1LLPENR (rw) register accessor: RCC APB1 Low Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1llpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1llpenr`]
module"]
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENRrs>;
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod apb1llpenr;
#[doc = "C1_APB1LLPENR (rw) register accessor: RCC APB1 Low Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1llpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1llpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb1llpenr`]
module"]
pub type C1_APB1LLPENR = crate::Reg<c1_apb1llpenr::C1_APB1LLPENRrs>;
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod c1_apb1llpenr;
#[doc = "C1_APB1HLPENR (rw) register accessor: RCC APB1 High Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb1hlpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb1hlpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb1hlpenr`]
module"]
pub type C1_APB1HLPENR = crate::Reg<c1_apb1hlpenr::C1_APB1HLPENRrs>;
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod c1_apb1hlpenr;
#[doc = "APB1HLPENR (rw) register accessor: RCC APB1 High Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hlpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hlpenr`]
module"]
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENRrs>;
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod apb1hlpenr;
#[doc = "APB2LPENR (rw) register accessor: RCC APB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpenr`]
module"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod apb2lpenr;
#[doc = "C1_APB2LPENR (rw) register accessor: RCC APB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb2lpenr`]
module"]
pub type C1_APB2LPENR = crate::Reg<c1_apb2lpenr::C1_APB2LPENRrs>;
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod c1_apb2lpenr;
#[doc = "C1_APB4LPENR (rw) register accessor: RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb4lpenr`]
module"]
pub type C1_APB4LPENR = crate::Reg<c1_apb4lpenr::C1_APB4LPENRrs>;
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod c1_apb4lpenr;
#[doc = "APB4LPENR (rw) register accessor: RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4lpenr`]
module"]
pub type APB4LPENR = crate::Reg<apb4lpenr::APB4LPENRrs>;
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod apb4lpenr;
