#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: crate::Reg<icscr::ICSCR_SPEC>,
    #[doc = "0x08 - Clock recovery RC register"]
    pub crrcr: crate::Reg<crrcr::CRRCR_SPEC>,
    #[doc = "0x0c - Clock configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x10 - Clock interrupt enable register"]
    pub cier: crate::Reg<cier::CIER_SPEC>,
    #[doc = "0x14 - Clock interrupt flag register"]
    pub cifr: crate::Reg<cifr::CIFR_SPEC>,
    #[doc = "0x18 - Clock interrupt clear register"]
    pub cicr: crate::Reg<cicr::CICR_SPEC>,
    #[doc = "0x1c - GPIO reset register"]
    pub ioprstr: crate::Reg<ioprstr::IOPRSTR_SPEC>,
    #[doc = "0x20 - AHB peripheral reset register"]
    pub ahbrstr: crate::Reg<ahbrstr::AHBRSTR_SPEC>,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: crate::Reg<apb2rstr::APB2RSTR_SPEC>,
    #[doc = "0x28 - APB1 peripheral reset register"]
    pub apb1rstr: crate::Reg<apb1rstr::APB1RSTR_SPEC>,
    #[doc = "0x2c - GPIO clock enable register"]
    pub iopenr: crate::Reg<iopenr::IOPENR_SPEC>,
    #[doc = "0x30 - AHB peripheral clock enable register"]
    pub ahbenr: crate::Reg<ahbenr::AHBENR_SPEC>,
    #[doc = "0x34 - APB2 peripheral clock enable register"]
    pub apb2enr: crate::Reg<apb2enr::APB2ENR_SPEC>,
    #[doc = "0x38 - APB1 peripheral clock enable register"]
    pub apb1enr: crate::Reg<apb1enr::APB1ENR_SPEC>,
    #[doc = "0x3c - GPIO clock enable in sleep mode register"]
    pub iopsmen: crate::Reg<iopsmen::IOPSMEN_SPEC>,
    #[doc = "0x40 - AHB peripheral clock enable in sleep mode register"]
    pub ahbsmenr: crate::Reg<ahbsmenr::AHBSMENR_SPEC>,
    #[doc = "0x44 - APB2 peripheral clock enable in sleep mode register"]
    pub apb2smenr: crate::Reg<apb2smenr::APB2SMENR_SPEC>,
    #[doc = "0x48 - APB1 peripheral clock enable in sleep mode register"]
    pub apb1smenr: crate::Reg<apb1smenr::APB1SMENR_SPEC>,
    #[doc = "0x4c - Clock configuration register"]
    pub ccipr: crate::Reg<ccipr::CCIPR_SPEC>,
    #[doc = "0x50 - Control and status register"]
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
#[doc = "CRRCR register accessor: an alias for `Reg<CRRCR_SPEC>`"]
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
#[doc = "Clock recovery RC register"]
pub mod crrcr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "CIER register accessor: an alias for `Reg<CIER_SPEC>`"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "CIFR register accessor: an alias for `Reg<CIFR_SPEC>`"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "CICR register accessor: an alias for `Reg<CICR_SPEC>`"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "IOPRSTR register accessor: an alias for `Reg<IOPRSTR_SPEC>`"]
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTR_SPEC>;
#[doc = "GPIO reset register"]
pub mod ioprstr;
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
#[doc = "IOPENR register accessor: an alias for `Reg<IOPENR_SPEC>`"]
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
#[doc = "GPIO clock enable register"]
pub mod iopenr;
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
#[doc = "IOPSMEN register accessor: an alias for `Reg<IOPSMEN_SPEC>`"]
pub type IOPSMEN = crate::Reg<iopsmen::IOPSMEN_SPEC>;
#[doc = "GPIO clock enable in sleep mode register"]
pub mod iopsmen;
#[doc = "AHBSMENR register accessor: an alias for `Reg<AHBSMENR_SPEC>`"]
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENR_SPEC>;
#[doc = "AHB peripheral clock enable in sleep mode register"]
pub mod ahbsmenr;
#[doc = "APB2SMENR register accessor: an alias for `Reg<APB2SMENR_SPEC>`"]
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENR_SPEC>;
#[doc = "APB2 peripheral clock enable in sleep mode register"]
pub mod apb2smenr;
#[doc = "APB1SMENR register accessor: an alias for `Reg<APB1SMENR_SPEC>`"]
pub type APB1SMENR = crate::Reg<apb1smenr::APB1SMENR_SPEC>;
#[doc = "APB1 peripheral clock enable in sleep mode register"]
pub mod apb1smenr;
#[doc = "CCIPR register accessor: an alias for `Reg<CCIPR_SPEC>`"]
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
#[doc = "Clock configuration register"]
pub mod ccipr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control and status register"]
pub mod csr;
