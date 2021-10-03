#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: crate::Reg<icscr::ICSCR_SPEC>,
    #[doc = "0x08 - Clock configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x0c - Clock interrupt register"]
    pub cir: crate::Reg<cir::CIR_SPEC>,
    #[doc = "0x10 - AHB peripheral reset register"]
    pub ahbrstr: crate::Reg<ahbrstr::AHBRSTR_SPEC>,
    #[doc = "0x14 - APB2 peripheral reset register"]
    pub apb2rstr: crate::Reg<apb2rstr::APB2RSTR_SPEC>,
    #[doc = "0x18 - APB1 peripheral reset register"]
    pub apb1rstr: crate::Reg<apb1rstr::APB1RSTR_SPEC>,
    #[doc = "0x1c - AHB peripheral clock enable register"]
    pub ahbenr: crate::Reg<ahbenr::AHBENR_SPEC>,
    #[doc = "0x20 - APB2 peripheral clock enable register"]
    pub apb2enr: crate::Reg<apb2enr::APB2ENR_SPEC>,
    #[doc = "0x24 - APB1 peripheral clock enable register"]
    pub apb1enr: crate::Reg<apb1enr::APB1ENR_SPEC>,
    #[doc = "0x28 - AHB peripheral clock enable in low power mode register"]
    pub ahblpenr: crate::Reg<ahblpenr::AHBLPENR_SPEC>,
    #[doc = "0x2c - APB2 peripheral clock enable in low power mode register"]
    pub apb2lpenr: crate::Reg<apb2lpenr::APB2LPENR_SPEC>,
    #[doc = "0x30 - APB1 peripheral clock enable in low power mode register"]
    pub apb1lpenr: crate::Reg<apb1lpenr::APB1LPENR_SPEC>,
    #[doc = "0x34 - Control/status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "ICSCR register accessor: an alias for `Reg<ICSCR_SPEC>`"]
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "CIR register accessor: an alias for `Reg<CIR_SPEC>`"]
pub type CIR = crate::Reg<cir::CIR_SPEC>;
#[doc = "Clock interrupt register"]
pub mod cir;
#[doc = "AHBRSTR register accessor: an alias for `Reg<AHBRSTR_SPEC>`"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "APB2RSTR register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB1RSTR register accessor: an alias for `Reg<APB1RSTR_SPEC>`"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "AHBENR register accessor: an alias for `Reg<AHBENR_SPEC>`"]
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
#[doc = "AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APB2ENR register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "APB1ENR register accessor: an alias for `Reg<APB1ENR_SPEC>`"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "AHBLPENR register accessor: an alias for `Reg<AHBLPENR_SPEC>`"]
pub type AHBLPENR = crate::Reg<ahblpenr::AHBLPENR_SPEC>;
#[doc = "AHB peripheral clock enable in low power mode register"]
pub mod ahblpenr;
#[doc = "APB2LPENR register accessor: an alias for `Reg<APB2LPENR_SPEC>`"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
#[doc = "APB2 peripheral clock enable in low power mode register"]
pub mod apb2lpenr;
#[doc = "APB1LPENR register accessor: an alias for `Reg<APB1LPENR_SPEC>`"]
pub type APB1LPENR = crate::Reg<apb1lpenr::APB1LPENR_SPEC>;
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control/status register"]
pub mod csr;
