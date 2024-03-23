#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    pllcfgr: PLLCFGR,
    cfgr: CFGR,
    cir: CIR,
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    ahb3rstr: AHB3RSTR,
    _reserved7: [u8; 0x04],
    apb1rstr: APB1RSTR,
    apb2rstr: APB2RSTR,
    _reserved9: [u8; 0x08],
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    ahb3enr: AHB3ENR,
    _reserved12: [u8; 0x04],
    apb1enr: APB1ENR,
    apb2enr: APB2ENR,
    _reserved14: [u8; 0x08],
    ahb1lpenr: AHB1LPENR,
    ahb2lpenr: AHB2LPENR,
    ahb3lpenr: AHB3LPENR,
    _reserved17: [u8; 0x04],
    apb1lpenr: APB1LPENR,
    apb2lpenr: APB2LPENR,
    _reserved19: [u8; 0x08],
    bdcr: BDCR,
    csr: CSR,
    _reserved21: [u8; 0x08],
    sscgr: SSCGR,
    plli2scfgr: PLLI2SCFGR,
    pllsaicfgr: PLLSAICFGR,
    dckcfgr1: DCKCFGR1,
    dckcfgr2: DCKCFGR2,
}
impl RegisterBlock {
    #[doc = "0x00 - clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &PLLCFGR {
        &self.pllcfgr
    }
    #[doc = "0x08 - clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x0c - clock interrupt register"]
    #[inline(always)]
    pub const fn cir(&self) -> &CIR {
        &self.cir
    }
    #[doc = "0x10 - AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    #[doc = "0x14 - AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    #[doc = "0x18 - AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    #[doc = "0x20 - APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &APB1RSTR {
        &self.apb1rstr
    }
    #[doc = "0x24 - APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    #[doc = "0x30 - AHB1 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    #[doc = "0x34 - AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    #[doc = "0x38 - AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    #[doc = "0x40 - APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1enr(&self) -> &APB1ENR {
        &self.apb1enr
    }
    #[doc = "0x44 - APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    #[doc = "0x50 - AHB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &AHB1LPENR {
        &self.ahb1lpenr
    }
    #[doc = "0x54 - AHB2 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &AHB2LPENR {
        &self.ahb2lpenr
    }
    #[doc = "0x58 - AHB3 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb3lpenr(&self) -> &AHB3LPENR {
        &self.ahb3lpenr
    }
    #[doc = "0x60 - APB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn apb1lpenr(&self) -> &APB1LPENR {
        &self.apb1lpenr
    }
    #[doc = "0x64 - APB2 peripheral clock enabled in low power mode register"]
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    #[doc = "0x70 - Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    #[doc = "0x74 - clock control &amp; status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x80 - spread spectrum clock generation register"]
    #[inline(always)]
    pub const fn sscgr(&self) -> &SSCGR {
        &self.sscgr
    }
    #[doc = "0x84 - PLLI2S configuration register"]
    #[inline(always)]
    pub const fn plli2scfgr(&self) -> &PLLI2SCFGR {
        &self.plli2scfgr
    }
    #[doc = "0x88 - PLL configuration register"]
    #[inline(always)]
    pub const fn pllsaicfgr(&self) -> &PLLSAICFGR {
        &self.pllsaicfgr
    }
    #[doc = "0x8c - dedicated clocks configuration register"]
    #[inline(always)]
    pub const fn dckcfgr1(&self) -> &DCKCFGR1 {
        &self.dckcfgr1
    }
    #[doc = "0x90 - dedicated clocks configuration register"]
    #[inline(always)]
    pub const fn dckcfgr2(&self) -> &DCKCFGR2 {
        &self.dckcfgr2
    }
}
#[doc = "CR (rw) register accessor: clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "clock control register"]
pub mod cr;
#[doc = "PLLCFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`]
module"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "CFGR (rw) register accessor: clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "clock configuration register"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: clock interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir`]
module"]
pub type CIR = crate::Reg<cir::CIRrs>;
#[doc = "clock interrupt register"]
pub mod cir;
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
#[doc = "APB1RSTR (rw) register accessor: APB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr`]
module"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTRrs>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "APB2RSTR (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`]
module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1ENR (rw) register accessor: AHB1 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`]
module"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
#[doc = "AHB1 peripheral clock register"]
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
#[doc = "APB1ENR (rw) register accessor: APB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr`]
module"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENRrs>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "APB2ENR (rw) register accessor: APB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`]
module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "AHB1LPENR (rw) register accessor: AHB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1lpenr`]
module"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENRrs>;
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub mod ahb1lpenr;
#[doc = "AHB2LPENR (rw) register accessor: AHB2 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2lpenr`]
module"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENRrs>;
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub mod ahb2lpenr;
#[doc = "AHB3LPENR (rw) register accessor: AHB3 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3lpenr`]
module"]
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENRrs>;
#[doc = "AHB3 peripheral clock enable in low power mode register"]
pub mod ahb3lpenr;
#[doc = "APB1LPENR (rw) register accessor: APB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lpenr`]
module"]
pub type APB1LPENR = crate::Reg<apb1lpenr::APB1LPENRrs>;
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
#[doc = "APB2LPENR (rw) register accessor: APB2 peripheral clock enabled in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpenr`]
module"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub mod apb2lpenr;
#[doc = "BDCR (rw) register accessor: Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: clock control &amp; status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "clock control &amp; status register"]
pub mod csr;
#[doc = "SSCGR (rw) register accessor: spread spectrum clock generation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sscgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sscgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sscgr`]
module"]
pub type SSCGR = crate::Reg<sscgr::SSCGRrs>;
#[doc = "spread spectrum clock generation register"]
pub mod sscgr;
#[doc = "PLLI2SCFGR (rw) register accessor: PLLI2S configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plli2scfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plli2scfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plli2scfgr`]
module"]
pub type PLLI2SCFGR = crate::Reg<plli2scfgr::PLLI2SCFGRrs>;
#[doc = "PLLI2S configuration register"]
pub mod plli2scfgr;
#[doc = "PLLSAICFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsaicfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsaicfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllsaicfgr`]
module"]
pub type PLLSAICFGR = crate::Reg<pllsaicfgr::PLLSAICFGRrs>;
#[doc = "PLL configuration register"]
pub mod pllsaicfgr;
#[doc = "DCKCFGR1 (rw) register accessor: dedicated clocks configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dckcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dckcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dckcfgr1`]
module"]
pub type DCKCFGR1 = crate::Reg<dckcfgr1::DCKCFGR1rs>;
#[doc = "dedicated clocks configuration register"]
pub mod dckcfgr1;
#[doc = "DCKCFGR2 (rw) register accessor: dedicated clocks configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dckcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dckcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dckcfgr2`]
module"]
pub type DCKCFGR2 = crate::Reg<dckcfgr2::DCKCFGR2rs>;
#[doc = "dedicated clocks configuration register"]
pub mod dckcfgr2;
