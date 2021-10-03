#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: crate::Reg<icscr::ICSCR_SPEC>,
    #[doc = "0x08 - Clock configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x0c - PLL configuration register"]
    pub pllsyscfgr: crate::Reg<pllsyscfgr::PLLSYSCFGR_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Clock interrupt enable register"]
    pub cier: crate::Reg<cier::CIER_SPEC>,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub cifr: crate::Reg<cifr::CIFR_SPEC>,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub cicr: crate::Reg<cicr::CICR_SPEC>,
    #[doc = "0x24 - GPIO reset register"]
    pub ioprstr: crate::Reg<ioprstr::IOPRSTR_SPEC>,
    #[doc = "0x28 - AHB peripheral reset register"]
    pub ahbrstr: crate::Reg<ahbrstr::AHBRSTR_SPEC>,
    #[doc = "0x2c - APB peripheral reset register 1"]
    pub apbrstr1: crate::Reg<apbrstr1::APBRSTR1_SPEC>,
    #[doc = "0x30 - APB peripheral reset register 2"]
    pub apbrstr2: crate::Reg<apbrstr2::APBRSTR2_SPEC>,
    #[doc = "0x34 - GPIO clock enable register"]
    pub iopenr: crate::Reg<iopenr::IOPENR_SPEC>,
    #[doc = "0x38 - AHB peripheral clock enable register"]
    pub ahbenr: crate::Reg<ahbenr::AHBENR_SPEC>,
    #[doc = "0x3c - APB peripheral clock enable register 1"]
    pub apbenr1: crate::Reg<apbenr1::APBENR1_SPEC>,
    #[doc = "0x40 - APB peripheral clock enable register 2"]
    pub apbenr2: crate::Reg<apbenr2::APBENR2_SPEC>,
    #[doc = "0x44 - GPIO in Sleep mode clock enable register"]
    pub iopsmenr: crate::Reg<iopsmenr::IOPSMENR_SPEC>,
    #[doc = "0x48 - AHB peripheral clock enable in Sleep mode register"]
    pub ahbsmenr: crate::Reg<ahbsmenr::AHBSMENR_SPEC>,
    #[doc = "0x4c - APB peripheral clock enable in Sleep mode register 1"]
    pub apbsmenr1: crate::Reg<apbsmenr1::APBSMENR1_SPEC>,
    #[doc = "0x50 - APB peripheral clock enable in Sleep mode register 2"]
    pub apbsmenr2: crate::Reg<apbsmenr2::APBSMENR2_SPEC>,
    #[doc = "0x54 - Peripherals independent clock configuration register"]
    pub ccipr: crate::Reg<ccipr::CCIPR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x5c - RTC domain control register"]
    pub bdcr: crate::Reg<bdcr::BDCR_SPEC>,
    #[doc = "0x60 - Control/status register"]
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
#[doc = "PLLSYSCFGR register accessor: an alias for `Reg<PLLSYSCFGR_SPEC>`"]
pub type PLLSYSCFGR = crate::Reg<pllsyscfgr::PLLSYSCFGR_SPEC>;
#[doc = "PLL configuration register"]
pub mod pllsyscfgr;
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
#[doc = "AHBRSTR register accessor: an alias for `Reg<AHBRSTR_SPEC>`"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "IOPRSTR register accessor: an alias for `Reg<IOPRSTR_SPEC>`"]
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTR_SPEC>;
#[doc = "GPIO reset register"]
pub mod ioprstr;
#[doc = "APBRSTR1 register accessor: an alias for `Reg<APBRSTR1_SPEC>`"]
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1_SPEC>;
#[doc = "APB peripheral reset register 1"]
pub mod apbrstr1;
#[doc = "APBRSTR2 register accessor: an alias for `Reg<APBRSTR2_SPEC>`"]
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2_SPEC>;
#[doc = "APB peripheral reset register 2"]
pub mod apbrstr2;
#[doc = "IOPENR register accessor: an alias for `Reg<IOPENR_SPEC>`"]
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
#[doc = "GPIO clock enable register"]
pub mod iopenr;
#[doc = "AHBENR register accessor: an alias for `Reg<AHBENR_SPEC>`"]
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
#[doc = "AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APBENR1 register accessor: an alias for `Reg<APBENR1_SPEC>`"]
pub type APBENR1 = crate::Reg<apbenr1::APBENR1_SPEC>;
#[doc = "APB peripheral clock enable register 1"]
pub mod apbenr1;
#[doc = "APBENR2 register accessor: an alias for `Reg<APBENR2_SPEC>`"]
pub type APBENR2 = crate::Reg<apbenr2::APBENR2_SPEC>;
#[doc = "APB peripheral clock enable register 2"]
pub mod apbenr2;
#[doc = "IOPSMENR register accessor: an alias for `Reg<IOPSMENR_SPEC>`"]
pub type IOPSMENR = crate::Reg<iopsmenr::IOPSMENR_SPEC>;
#[doc = "GPIO in Sleep mode clock enable register"]
pub mod iopsmenr;
#[doc = "AHBSMENR register accessor: an alias for `Reg<AHBSMENR_SPEC>`"]
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENR_SPEC>;
#[doc = "AHB peripheral clock enable in Sleep mode register"]
pub mod ahbsmenr;
#[doc = "APBSMENR1 register accessor: an alias for `Reg<APBSMENR1_SPEC>`"]
pub type APBSMENR1 = crate::Reg<apbsmenr1::APBSMENR1_SPEC>;
#[doc = "APB peripheral clock enable in Sleep mode register 1"]
pub mod apbsmenr1;
#[doc = "APBSMENR2 register accessor: an alias for `Reg<APBSMENR2_SPEC>`"]
pub type APBSMENR2 = crate::Reg<apbsmenr2::APBSMENR2_SPEC>;
#[doc = "APB peripheral clock enable in Sleep mode register 2"]
pub mod apbsmenr2;
#[doc = "CCIPR register accessor: an alias for `Reg<CCIPR_SPEC>`"]
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
#[doc = "Peripherals independent clock configuration register"]
pub mod ccipr;
#[doc = "BDCR register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "RTC domain control register"]
pub mod bdcr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control/status register"]
pub mod csr;
