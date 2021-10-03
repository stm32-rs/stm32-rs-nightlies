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
    pub pllcfgr: crate::Reg<pllcfgr::PLLCFGR_SPEC>,
    #[doc = "0x10 - PLLSAI1 configuration register"]
    pub pllsai1cfgr: crate::Reg<pllsai1cfgr::PLLSAI1CFGR_SPEC>,
    #[doc = "0x14 - PLLSAI2 configuration register"]
    pub pllsai2cfgr: crate::Reg<pllsai2cfgr::PLLSAI2CFGR_SPEC>,
    #[doc = "0x18 - Clock interrupt enable register"]
    pub cier: crate::Reg<cier::CIER_SPEC>,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub cifr: crate::Reg<cifr::CIFR_SPEC>,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub cicr: crate::Reg<cicr::CICR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - AHB1 peripheral reset register"]
    pub ahb1rstr: crate::Reg<ahb1rstr::AHB1RSTR_SPEC>,
    #[doc = "0x2c - AHB2 peripheral reset register"]
    pub ahb2rstr: crate::Reg<ahb2rstr::AHB2RSTR_SPEC>,
    #[doc = "0x30 - AHB3 peripheral reset register"]
    pub ahb3rstr: crate::Reg<ahb3rstr::AHB3RSTR_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    pub apb1rstr1: crate::Reg<apb1rstr1::APB1RSTR1_SPEC>,
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    pub apb1rstr2: crate::Reg<apb1rstr2::APB1RSTR2_SPEC>,
    #[doc = "0x40 - APB2 peripheral reset register"]
    pub apb2rstr: crate::Reg<apb2rstr::APB2RSTR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    pub ahb1enr: crate::Reg<ahb1enr::AHB1ENR_SPEC>,
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    pub ahb2enr: crate::Reg<ahb2enr::AHB2ENR_SPEC>,
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    pub ahb3enr: crate::Reg<ahb3enr::AHB3ENR_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - APB1ENR1"]
    pub apb1enr1: crate::Reg<apb1enr1::APB1ENR1_SPEC>,
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    pub apb1enr2: crate::Reg<apb1enr2::APB1ENR2_SPEC>,
    #[doc = "0x60 - APB2ENR"]
    pub apb2enr: crate::Reg<apb2enr::APB2ENR_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb1smenr: crate::Reg<ahb1smenr::AHB1SMENR_SPEC>,
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb2smenr: crate::Reg<ahb2smenr::AHB2SMENR_SPEC>,
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb3smenr: crate::Reg<ahb3smenr::AHB3SMENR_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x78 - APB1SMENR1"]
    pub apb1smenr1: crate::Reg<apb1smenr1::APB1SMENR1_SPEC>,
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    pub apb1smenr2: crate::Reg<apb1smenr2::APB1SMENR2_SPEC>,
    #[doc = "0x80 - APB2SMENR"]
    pub apb2smenr: crate::Reg<apb2smenr::APB2SMENR_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x88 - CCIPR"]
    pub ccipr: crate::Reg<ccipr::CCIPR_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x90 - BDCR"]
    pub bdcr: crate::Reg<bdcr::BDCR_SPEC>,
    #[doc = "0x94 - CSR"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x98 - Clock recovery RC register"]
    pub crrcr: crate::Reg<crrcr::CRRCR_SPEC>,
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
#[doc = "PLLCFGR register accessor: an alias for `Reg<PLLCFGR_SPEC>`"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "PLLSAI1CFGR register accessor: an alias for `Reg<PLLSAI1CFGR_SPEC>`"]
pub type PLLSAI1CFGR = crate::Reg<pllsai1cfgr::PLLSAI1CFGR_SPEC>;
#[doc = "PLLSAI1 configuration register"]
pub mod pllsai1cfgr;
#[doc = "PLLSAI2CFGR register accessor: an alias for `Reg<PLLSAI2CFGR_SPEC>`"]
pub type PLLSAI2CFGR = crate::Reg<pllsai2cfgr::PLLSAI2CFGR_SPEC>;
#[doc = "PLLSAI2 configuration register"]
pub mod pllsai2cfgr;
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
#[doc = "AHB1RSTR register accessor: an alias for `Reg<AHB1RSTR_SPEC>`"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR register accessor: an alias for `Reg<AHB2RSTR_SPEC>`"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3RSTR register accessor: an alias for `Reg<AHB3RSTR_SPEC>`"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
#[doc = "APB1RSTR1 register accessor: an alias for `Reg<APB1RSTR1_SPEC>`"]
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1_SPEC>;
#[doc = "APB1 peripheral reset register 1"]
pub mod apb1rstr1;
#[doc = "APB1RSTR2 register accessor: an alias for `Reg<APB1RSTR2_SPEC>`"]
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2_SPEC>;
#[doc = "APB1 peripheral reset register 2"]
pub mod apb1rstr2;
#[doc = "APB2RSTR register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1ENR register accessor: an alias for `Reg<AHB1ENR_SPEC>`"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "AHB1 peripheral clock enable register"]
pub mod ahb1enr;
#[doc = "AHB2ENR register accessor: an alias for `Reg<AHB2ENR_SPEC>`"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3ENR register accessor: an alias for `Reg<AHB3ENR_SPEC>`"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
#[doc = "APB1ENR1 register accessor: an alias for `Reg<APB1ENR1_SPEC>`"]
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1_SPEC>;
#[doc = "APB1ENR1"]
pub mod apb1enr1;
#[doc = "APB1ENR2 register accessor: an alias for `Reg<APB1ENR2_SPEC>`"]
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2_SPEC>;
#[doc = "APB1 peripheral clock enable register 2"]
pub mod apb1enr2;
#[doc = "APB2ENR register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2ENR"]
pub mod apb2enr;
#[doc = "AHB1SMENR register accessor: an alias for `Reg<AHB1SMENR_SPEC>`"]
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENR_SPEC>;
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb1smenr;
#[doc = "AHB2SMENR register accessor: an alias for `Reg<AHB2SMENR_SPEC>`"]
pub type AHB2SMENR = crate::Reg<ahb2smenr::AHB2SMENR_SPEC>;
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb2smenr;
#[doc = "AHB3SMENR register accessor: an alias for `Reg<AHB3SMENR_SPEC>`"]
pub type AHB3SMENR = crate::Reg<ahb3smenr::AHB3SMENR_SPEC>;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb3smenr;
#[doc = "APB1SMENR1 register accessor: an alias for `Reg<APB1SMENR1_SPEC>`"]
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1_SPEC>;
#[doc = "APB1SMENR1"]
pub mod apb1smenr1;
#[doc = "APB1SMENR2 register accessor: an alias for `Reg<APB1SMENR2_SPEC>`"]
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2_SPEC>;
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub mod apb1smenr2;
#[doc = "APB2SMENR register accessor: an alias for `Reg<APB2SMENR_SPEC>`"]
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENR_SPEC>;
#[doc = "APB2SMENR"]
pub mod apb2smenr;
#[doc = "CCIPR register accessor: an alias for `Reg<CCIPR_SPEC>`"]
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
#[doc = "CCIPR"]
pub mod ccipr;
#[doc = "BDCR register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "BDCR"]
pub mod bdcr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CSR"]
pub mod csr;
#[doc = "CRRCR register accessor: an alias for `Reg<CRRCR_SPEC>`"]
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
#[doc = "Clock recovery RC register"]
pub mod crrcr;
