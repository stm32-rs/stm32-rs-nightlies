#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    crrcr: CRRCR,
    cfgr: CFGR,
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    ioprstr: IOPRSTR,
    ahbrstr: AHBRSTR,
    apb2rstr: APB2RSTR,
    apb1rstr: APB1RSTR,
    iopenr: IOPENR,
    ahbenr: AHBENR,
    apb2enr: APB2ENR,
    apb1enr: APB1ENR,
    iopsmen: IOPSMEN,
    ahbsmenr: AHBSMENR,
    apb2smenr: APB2SMENR,
    apb1smenr: APB1SMENR,
    ccipr: CCIPR,
    csr: CSR,
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
    #[doc = "0x08 - Clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    #[doc = "0x0c - Clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x10 - Clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    #[doc = "0x14 - Clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    #[doc = "0x18 - Clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    #[doc = "0x1c - GPIO reset register"]
    #[inline(always)]
    pub const fn ioprstr(&self) -> &IOPRSTR {
        &self.ioprstr
    }
    #[doc = "0x20 - AHB peripheral reset register"]
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &AHBRSTR {
        &self.ahbrstr
    }
    #[doc = "0x24 - APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    #[doc = "0x28 - APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &APB1RSTR {
        &self.apb1rstr
    }
    #[doc = "0x2c - GPIO clock enable register"]
    #[inline(always)]
    pub const fn iopenr(&self) -> &IOPENR {
        &self.iopenr
    }
    #[doc = "0x30 - AHB peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    #[doc = "0x34 - APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    #[doc = "0x38 - APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1enr(&self) -> &APB1ENR {
        &self.apb1enr
    }
    #[doc = "0x3c - GPIO clock enable in sleep mode register"]
    #[inline(always)]
    pub const fn iopsmen(&self) -> &IOPSMEN {
        &self.iopsmen
    }
    #[doc = "0x40 - AHB peripheral clock enable in sleep mode register"]
    #[inline(always)]
    pub const fn ahbsmenr(&self) -> &AHBSMENR {
        &self.ahbsmenr
    }
    #[doc = "0x44 - APB2 peripheral clock enable in sleep mode register"]
    #[inline(always)]
    pub const fn apb2smenr(&self) -> &APB2SMENR {
        &self.apb2smenr
    }
    #[doc = "0x48 - APB1 peripheral clock enable in sleep mode register"]
    #[inline(always)]
    pub const fn apb1smenr(&self) -> &APB1SMENR {
        &self.apb1smenr
    }
    #[doc = "0x4c - Clock configuration register"]
    #[inline(always)]
    pub const fn ccipr(&self) -> &CCIPR {
        &self.ccipr
    }
    #[doc = "0x50 - Control and status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
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
#[doc = "CRRCR (rw) register accessor: Clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crrcr`]
module"]
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
#[doc = "Clock recovery RC register"]
pub mod crrcr;
#[doc = "CFGR (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "CIER (r) register accessor: Clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`]
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
#[doc = "IOPRSTR (rw) register accessor: GPIO reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioprstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioprstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioprstr`]
module"]
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTRrs>;
#[doc = "GPIO reset register"]
pub mod ioprstr;
#[doc = "AHBRSTR (rw) register accessor: AHB peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`]
module"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTRrs>;
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "APB2RSTR (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`]
module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB1RSTR (rw) register accessor: APB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr`]
module"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTRrs>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "IOPENR (rw) register accessor: GPIO clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopenr`]
module"]
pub type IOPENR = crate::Reg<iopenr::IOPENRrs>;
#[doc = "GPIO clock enable register"]
pub mod iopenr;
#[doc = "AHBENR (rw) register accessor: AHB peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`]
module"]
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
#[doc = "AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APB2ENR (rw) register accessor: APB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`]
module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "APB1ENR (rw) register accessor: APB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr`]
module"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENRrs>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "IOPSMEN (rw) register accessor: GPIO clock enable in sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopsmen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopsmen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopsmen`]
module"]
pub type IOPSMEN = crate::Reg<iopsmen::IOPSMENrs>;
#[doc = "GPIO clock enable in sleep mode register"]
pub mod iopsmen;
#[doc = "AHBSMENR (rw) register accessor: AHB peripheral clock enable in sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbsmenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbsmenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbsmenr`]
module"]
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENRrs>;
#[doc = "AHB peripheral clock enable in sleep mode register"]
pub mod ahbsmenr;
#[doc = "APB2SMENR (rw) register accessor: APB2 peripheral clock enable in sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2smenr`]
module"]
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENRrs>;
#[doc = "APB2 peripheral clock enable in sleep mode register"]
pub mod apb2smenr;
#[doc = "APB1SMENR (rw) register accessor: APB1 peripheral clock enable in sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1smenr`]
module"]
pub type APB1SMENR = crate::Reg<apb1smenr::APB1SMENRrs>;
#[doc = "APB1 peripheral clock enable in sleep mode register"]
pub mod apb1smenr;
#[doc = "CCIPR (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr`]
module"]
pub type CCIPR = crate::Reg<ccipr::CCIPRrs>;
#[doc = "Clock configuration register"]
pub mod ccipr;
#[doc = "CSR (rw) register accessor: Control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "Control and status register"]
pub mod csr;
