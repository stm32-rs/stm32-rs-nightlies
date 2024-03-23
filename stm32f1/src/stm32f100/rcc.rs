#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    cfgr: CFGR,
    cir: CIR,
    apb2rstr: APB2RSTR,
    apb1rstr: APB1RSTR,
    ahbenr: AHBENR,
    apb2enr: APB2ENR,
    apb1enr: APB1ENR,
    bdcr: BDCR,
    csr: CSR,
    _reserved10: [u8; 0x04],
    cfgr2: CFGR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Clock configuration register (RCC_CFGR)"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x08 - Clock interrupt register (RCC_CIR)"]
    #[inline(always)]
    pub const fn cir(&self) -> &CIR {
        &self.cir
    }
    #[doc = "0x0c - APB2 peripheral reset register (RCC_APB2RSTR)"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    #[doc = "0x10 - APB1 peripheral reset register (RCC_APB1RSTR)"]
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &APB1RSTR {
        &self.apb1rstr
    }
    #[doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)"]
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    #[doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    #[doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)"]
    #[inline(always)]
    pub const fn apb1enr(&self) -> &APB1ENR {
        &self.apb1enr
    }
    #[doc = "0x20 - Backup domain control register (RCC_BDCR)"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    #[doc = "0x24 - Control/status register (RCC_CSR)"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x2c - Clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
}
#[doc = "CR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: Clock configuration register (RCC_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "Clock configuration register (RCC_CFGR)"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: Clock interrupt register (RCC_CIR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir`]
module"]
pub type CIR = crate::Reg<cir::CIRrs>;
#[doc = "Clock interrupt register (RCC_CIR)"]
pub mod cir;
#[doc = "APB2RSTR (rw) register accessor: APB2 peripheral reset register (RCC_APB2RSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`]
module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
pub mod apb2rstr;
#[doc = "APB1RSTR (rw) register accessor: APB1 peripheral reset register (RCC_APB1RSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr`]
module"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTRrs>;
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
pub mod apb1rstr;
#[doc = "AHBENR (rw) register accessor: AHB Peripheral Clock enable register (RCC_AHBENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`]
module"]
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
pub mod ahbenr;
#[doc = "APB2ENR (rw) register accessor: APB2 peripheral clock enable register (RCC_APB2ENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`]
module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
pub mod apb2enr;
#[doc = "APB1ENR (rw) register accessor: APB1 peripheral clock enable register (RCC_APB1ENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr`]
module"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENRrs>;
#[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
pub mod apb1enr;
#[doc = "BDCR (rw) register accessor: Backup domain control register (RCC_BDCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
#[doc = "Backup domain control register (RCC_BDCR)"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: Control/status register (RCC_CSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "Control/status register (RCC_CSR)"]
pub mod csr;
#[doc = "CFGR2 (rw) register accessor: Clock configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "Clock configuration register 2"]
pub mod cfgr2;
