#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    cfgr: CFGR,
    pllcfgr: PLLCFGR,
    pllsai1cfgr: PLLSAI1CFGR,
    _reserved5: [u8; 0x04],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved8: [u8; 0x04],
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    ahb3rstr: AHB3RSTR,
    _reserved11: [u8; 0x04],
    apb1rstr1: APB1RSTR1,
    apb1rstr2: APB1RSTR2,
    apb2rstr: APB2RSTR,
    _reserved14: [u8; 0x04],
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    ahb3enr: AHB3ENR,
    _reserved17: [u8; 0x04],
    apb1enr1: APB1ENR1,
    apb1enr2: APB1ENR2,
    apb2enr: APB2ENR,
    _reserved20: [u8; 0x04],
    ahb1smenr: AHB1SMENR,
    ahb2smenr: AHB2SMENR,
    ahb3smenr: AHB3SMENR,
    _reserved23: [u8; 0x04],
    apb1smenr1: APB1SMENR1,
    apb1smenr2: APB1SMENR2,
    apb2smenr: APB2SMENR,
    _reserved26: [u8; 0x04],
    ccipr: CCIPR,
    _reserved27: [u8; 0x04],
    bdcr: BDCR,
    csr: CSR,
    crrcr: CRRCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Internal clock sources calibration register"]
    #[inline(always)]
    pub const fn icscr(&self) -> &ICSCR {
        &self.icscr
    }
    #[doc = "0x08 - Clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x0c - PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &PLLCFGR {
        &self.pllcfgr
    }
    #[doc = "0x10 - PLLSAI1 configuration register"]
    #[inline(always)]
    pub const fn pllsai1cfgr(&self) -> &PLLSAI1CFGR {
        &self.pllsai1cfgr
    }
    #[doc = "0x18 - Clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    #[doc = "0x1c - Clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    #[doc = "0x20 - Clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    #[doc = "0x28 - AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    #[doc = "0x2c - AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    #[doc = "0x30 - AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn apb1rstr1(&self) -> &APB1RSTR1 {
        &self.apb1rstr1
    }
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn apb1rstr2(&self) -> &APB1RSTR2 {
        &self.apb1rstr2
    }
    #[doc = "0x40 - APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    #[doc = "0x58 - APB1ENR1"]
    #[inline(always)]
    pub const fn apb1enr1(&self) -> &APB1ENR1 {
        &self.apb1enr1
    }
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn apb1enr2(&self) -> &APB1ENR2 {
        &self.apb1enr2
    }
    #[doc = "0x60 - APB2ENR"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb1smenr(&self) -> &AHB1SMENR {
        &self.ahb1smenr
    }
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb2smenr(&self) -> &AHB2SMENR {
        &self.ahb2smenr
    }
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb3smenr(&self) -> &AHB3SMENR {
        &self.ahb3smenr
    }
    #[doc = "0x78 - APB1SMENR1"]
    #[inline(always)]
    pub const fn apb1smenr1(&self) -> &APB1SMENR1 {
        &self.apb1smenr1
    }
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn apb1smenr2(&self) -> &APB1SMENR2 {
        &self.apb1smenr2
    }
    #[doc = "0x80 - APB2SMENR"]
    #[inline(always)]
    pub const fn apb2smenr(&self) -> &APB2SMENR {
        &self.apb2smenr
    }
    #[doc = "0x88 - CCIPR"]
    #[inline(always)]
    pub const fn ccipr(&self) -> &CCIPR {
        &self.ccipr
    }
    #[doc = "0x90 - BDCR"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    #[doc = "0x94 - CSR"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x98 - Clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
}
#[doc = "CR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "ICSCR (rw) register accessor: Internal clock sources calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icscr`]
module"]
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "CFGR (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "PLLCFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`]
module"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "PLLSAI1CFGR (rw) register accessor: PLLSAI1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsai1cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsai1cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllsai1cfgr`]
module"]
pub type PLLSAI1CFGR = crate::Reg<pllsai1cfgr::PLLSAI1CFGRrs>;
#[doc = "PLLSAI1 configuration register"]
pub mod pllsai1cfgr;
#[doc = "CIER (rw) register accessor: Clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`]
module"]
pub type CIER = crate::Reg<cier::CIERrs>;
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: Clock interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`]
module"]
pub type CIFR = crate::Reg<cifr::CIFRrs>;
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "CICR (w) register accessor: Clock interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`]
module"]
pub type CICR = crate::Reg<cicr::CICRrs>;
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "AHB1RSTR (rw) register accessor: AHB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`]
module"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: AHB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`]
module"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3RSTR (rw) register accessor: AHB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3rstr`]
module"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
#[doc = "APB1RSTR1 (rw) register accessor: APB1 peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr1`]
module"]
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1rs>;
#[doc = "APB1 peripheral reset register 1"]
pub mod apb1rstr1;
#[doc = "APB1RSTR2 (rw) register accessor: APB1 peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr2`]
module"]
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2rs>;
#[doc = "APB1 peripheral reset register 2"]
pub mod apb1rstr2;
#[doc = "APB2RSTR (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`]
module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1ENR (rw) register accessor: AHB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`]
module"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
#[doc = "AHB1 peripheral clock enable register"]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: AHB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`]
module"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3ENR (rw) register accessor: AHB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3enr`]
module"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
#[doc = "APB1ENR1 (rw) register accessor: APB1ENR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr1`]
module"]
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1rs>;
#[doc = "APB1ENR1"]
pub mod apb1enr1;
#[doc = "APB1ENR2 (rw) register accessor: APB1 peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr2`]
module"]
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2rs>;
#[doc = "APB1 peripheral clock enable register 2"]
pub mod apb1enr2;
#[doc = "APB2ENR (rw) register accessor: APB2ENR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`]
module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
#[doc = "APB2ENR"]
pub mod apb2enr;
#[doc = "AHB1SMENR (rw) register accessor: AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1smenr`]
module"]
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENRrs>;
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb1smenr;
#[doc = "AHB2SMENR (rw) register accessor: AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2smenr`]
module"]
pub type AHB2SMENR = crate::Reg<ahb2smenr::AHB2SMENRrs>;
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb2smenr;
#[doc = "AHB3SMENR (rw) register accessor: AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3smenr`]
module"]
pub type AHB3SMENR = crate::Reg<ahb3smenr::AHB3SMENRrs>;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb3smenr;
#[doc = "APB1SMENR1 (rw) register accessor: APB1SMENR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1smenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1smenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1smenr1`]
module"]
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1rs>;
#[doc = "APB1SMENR1"]
pub mod apb1smenr1;
#[doc = "APB1SMENR2 (rw) register accessor: APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1smenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1smenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1smenr2`]
module"]
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2rs>;
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub mod apb1smenr2;
#[doc = "APB2SMENR (rw) register accessor: APB2SMENR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2smenr`]
module"]
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENRrs>;
#[doc = "APB2SMENR"]
pub mod apb2smenr;
#[doc = "CCIPR (rw) register accessor: CCIPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr`]
module"]
pub type CCIPR = crate::Reg<ccipr::CCIPRrs>;
#[doc = "CCIPR"]
pub mod ccipr;
#[doc = "BDCR (rw) register accessor: BDCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
#[doc = "BDCR"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "CSR"]
pub mod csr;
#[doc = "CRRCR (rw) register accessor: Clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crrcr`]
module"]
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
#[doc = "Clock recovery RC register"]
pub mod crrcr;
