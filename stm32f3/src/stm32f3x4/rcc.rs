#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Clock configuration register (RCC_CFGR)"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x08 - Clock interrupt register (RCC_CIR)"]
    pub cir: crate::Reg<cir::CIR_SPEC>,
    #[doc = "0x0c - APB2 peripheral reset register (RCC_APB2RSTR)"]
    pub apb2rstr: crate::Reg<apb2rstr::APB2RSTR_SPEC>,
    #[doc = "0x10 - APB1 peripheral reset register (RCC_APB1RSTR)"]
    pub apb1rstr: crate::Reg<apb1rstr::APB1RSTR_SPEC>,
    #[doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)"]
    pub ahbenr: crate::Reg<ahbenr::AHBENR_SPEC>,
    #[doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)"]
    pub apb2enr: crate::Reg<apb2enr::APB2ENR_SPEC>,
    #[doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)"]
    pub apb1enr: crate::Reg<apb1enr::APB1ENR_SPEC>,
    #[doc = "0x20 - Backup domain control register (RCC_BDCR)"]
    pub bdcr: crate::Reg<bdcr::BDCR_SPEC>,
    #[doc = "0x24 - Control/status register (RCC_CSR)"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x28 - AHB peripheral reset register"]
    pub ahbrstr: crate::Reg<ahbrstr::AHBRSTR_SPEC>,
    #[doc = "0x2c - Clock configuration register 2"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    #[doc = "0x30 - Clock configuration register 3"]
    pub cfgr3: crate::Reg<cfgr3::CFGR3_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Clock control register"]
pub mod cr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Clock configuration register (RCC_CFGR)"]
pub mod cfgr;
#[doc = "CIR register accessor: an alias for `Reg<CIR_SPEC>`"]
pub type CIR = crate::Reg<cir::CIR_SPEC>;
#[doc = "Clock interrupt register (RCC_CIR)"]
pub mod cir;
#[doc = "APB2RSTR register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
pub mod apb2rstr;
#[doc = "APB1RSTR register accessor: an alias for `Reg<APB1RSTR_SPEC>`"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
pub mod apb1rstr;
#[doc = "AHBENR register accessor: an alias for `Reg<AHBENR_SPEC>`"]
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
pub mod ahbenr;
#[doc = "APB2ENR register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
pub mod apb2enr;
#[doc = "APB1ENR register accessor: an alias for `Reg<APB1ENR_SPEC>`"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
#[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
pub mod apb1enr;
#[doc = "BDCR register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "Backup domain control register (RCC_BDCR)"]
pub mod bdcr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control/status register (RCC_CSR)"]
pub mod csr;
#[doc = "AHBRSTR register accessor: an alias for `Reg<AHBRSTR_SPEC>`"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "Clock configuration register 2"]
pub mod cfgr2;
#[doc = "CFGR3 register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "Clock configuration register 3"]
pub mod cfgr3;
