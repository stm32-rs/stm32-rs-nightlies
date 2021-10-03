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
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Clock interrupt enable register"]
    pub cier: crate::Reg<cier::CIER_SPEC>,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub cifr: crate::Reg<cifr::CIFR_SPEC>,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub cicr: crate::Reg<cicr::CICR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - AHB1 peripheral reset register"]
    pub ahb1rstr: crate::Reg<ahb1rstr::AHB1RSTR_SPEC>,
    #[doc = "0x2c - AHB2 peripheral reset register"]
    pub ahb2rstr: crate::Reg<ahb2rstr::AHB2RSTR_SPEC>,
    #[doc = "0x30 - AHB3 peripheral reset register"]
    pub ahb3rstr: crate::Reg<ahb3rstr::AHB3RSTR_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    pub apb1rstr1: crate::Reg<apb1rstr1::APB1RSTR1_SPEC>,
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    pub apb1rstr2: crate::Reg<apb1rstr2::APB1RSTR2_SPEC>,
    #[doc = "0x40 - APB2 peripheral reset register"]
    pub apb2rstr: crate::Reg<apb2rstr::APB2RSTR_SPEC>,
    #[doc = "0x44 - APB3 peripheral reset register"]
    pub apb3rstr: crate::Reg<apb3rstr::APB3RSTR_SPEC>,
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    pub ahb1enr: crate::Reg<ahb1enr::AHB1ENR_SPEC>,
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    pub ahb2enr: crate::Reg<ahb2enr::AHB2ENR_SPEC>,
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    pub ahb3enr: crate::Reg<ahb3enr::AHB3ENR_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x58 - APB1 peripheral clock enable register 1"]
    pub apb1enr1: crate::Reg<apb1enr1::APB1ENR1_SPEC>,
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    pub apb1enr2: crate::Reg<apb1enr2::APB1ENR2_SPEC>,
    #[doc = "0x60 - APB2 peripheral clock enable register"]
    pub apb2enr: crate::Reg<apb2enr::APB2ENR_SPEC>,
    #[doc = "0x64 - APB3 peripheral clock enable register"]
    pub apb3enr: crate::Reg<apb3enr::APB3ENR_SPEC>,
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep modes register"]
    pub ahb1smenr: crate::Reg<ahb1smenr::AHB1SMENR_SPEC>,
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep modes register"]
    pub ahb2smenr: crate::Reg<ahb2smenr::AHB2SMENR_SPEC>,
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb3smenr: crate::Reg<ahb3smenr::AHB3SMENR_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x78 - APB1 peripheral clocks enable in Sleep mode register 1"]
    pub apb1smenr1: crate::Reg<apb1smenr1::APB1SMENR1_SPEC>,
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep mode register 2"]
    pub apb1smenr2: crate::Reg<apb1smenr2::APB1SMENR2_SPEC>,
    #[doc = "0x80 - APB2 peripheral clocks enable in Sleep mode register"]
    pub apb2smenr: crate::Reg<apb2smenr::APB2SMENR_SPEC>,
    #[doc = "0x84 - APB3 peripheral clock enable in Sleep mode register"]
    pub apb3smenr: crate::Reg<apb3smenr::APB3SMENR_SPEC>,
    #[doc = "0x88 - Peripherals independent clock configuration register"]
    pub ccipr: crate::Reg<ccipr::CCIPR_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0x90 - Backup domain control register"]
    pub bdcr: crate::Reg<bdcr::BDCR_SPEC>,
    #[doc = "0x94 - Control/status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    _reserved31: [u8; 0x70],
    #[doc = "0x108 - Extended clock recovery register"]
    pub extcfgr: crate::Reg<extcfgr::EXTCFGR_SPEC>,
    _reserved32: [u8; 0x3c],
    #[doc = "0x148 - CPU2 AHB1 peripheral clock enable register"]
    pub c2ahb1enr: crate::Reg<c2ahb1enr::C2AHB1ENR_SPEC>,
    #[doc = "0x14c - CPU2 AHB2 peripheral clock enable register"]
    pub c2ahb2enr: crate::Reg<c2ahb2enr::C2AHB2ENR_SPEC>,
    #[doc = "0x150 - CPU2 AHB3 peripheral clock enable register \\[dual core device only\\]"]
    pub c2ahb3enr: crate::Reg<c2ahb3enr::C2AHB3ENR_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0x158 - CPU2 APB1 peripheral clock enable register 1 \\[dual core device only\\]"]
    pub c2apb1enr1: crate::Reg<c2apb1enr1::C2APB1ENR1_SPEC>,
    #[doc = "0x15c - CPU2 APB1 peripheral clock enable register 2 \\[dual core device only\\]"]
    pub c2apb1enr2: crate::Reg<c2apb1enr2::C2APB1ENR2_SPEC>,
    #[doc = "0x160 - CPU2 APB2 peripheral clock enable register \\[dual core device only\\]"]
    pub c2apb2enr: crate::Reg<c2apb2enr::C2APB2ENR_SPEC>,
    #[doc = "0x164 - CPU2 APB3 peripheral clock enable register \\[dual core device only\\]"]
    pub c2apb3enr: crate::Reg<c2apb3enr::C2APB3ENR_SPEC>,
    #[doc = "0x168 - CPU2 AHB1 peripheral clocks enable in Sleep modes register \\[dual core device only\\]"]
    pub c2ahb1smenr: crate::Reg<c2ahb1smenr::C2AHB1SMENR_SPEC>,
    #[doc = "0x16c - CPU2 AHB2 peripheral clocks enable in Sleep modes register \\[dual core device only\\]"]
    pub c2ahb2smenr: crate::Reg<c2ahb2smenr::C2AHB2SMENR_SPEC>,
    #[doc = "0x170 - CPU2 AHB3 peripheral clocks enable in Sleep mode register \\[dual core device only\\]"]
    pub c2ahb3smenr: crate::Reg<c2ahb3smenr::C2AHB3SMENR_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x178 - CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \\[dual core device only\\]"]
    pub c2apb1smenr1: crate::Reg<c2apb1smenr1::C2APB1SMENR1_SPEC>,
    #[doc = "0x17c - CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \\[dual core device only\\]"]
    pub c2apb1smenr2: crate::Reg<c2apb1smenr2::C2APB1SMENR2_SPEC>,
    #[doc = "0x180 - CPU2 APB2 peripheral clocks enable in Sleep mode register \\[dual core device only\\]"]
    pub c2apb2smenr: crate::Reg<c2apb2smenr::C2APB2SMENR_SPEC>,
    #[doc = "0x184 - CPU2 APB3 peripheral clock enable in Sleep mode register \\[dual core device only\\]"]
    pub c2apb3smenr: crate::Reg<c2apb3smenr::C2APB3SMENR_SPEC>,
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
#[doc = "APB3RSTR register accessor: an alias for `Reg<APB3RSTR_SPEC>`"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
#[doc = "APB3 peripheral reset register"]
pub mod apb3rstr;
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
#[doc = "APB1 peripheral clock enable register 1"]
pub mod apb1enr1;
#[doc = "APB1ENR2 register accessor: an alias for `Reg<APB1ENR2_SPEC>`"]
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2_SPEC>;
#[doc = "APB1 peripheral clock enable register 2"]
pub mod apb1enr2;
#[doc = "APB2ENR register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "APB3ENR register accessor: an alias for `Reg<APB3ENR_SPEC>`"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
#[doc = "APB3 peripheral clock enable register"]
pub mod apb3enr;
#[doc = "AHB1SMENR register accessor: an alias for `Reg<AHB1SMENR_SPEC>`"]
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENR_SPEC>;
#[doc = "AHB1 peripheral clocks enable in Sleep modes register"]
pub mod ahb1smenr;
#[doc = "AHB2SMENR register accessor: an alias for `Reg<AHB2SMENR_SPEC>`"]
pub type AHB2SMENR = crate::Reg<ahb2smenr::AHB2SMENR_SPEC>;
#[doc = "AHB2 peripheral clocks enable in Sleep modes register"]
pub mod ahb2smenr;
#[doc = "AHB3SMENR register accessor: an alias for `Reg<AHB3SMENR_SPEC>`"]
pub type AHB3SMENR = crate::Reg<ahb3smenr::AHB3SMENR_SPEC>;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb3smenr;
#[doc = "APB1SMENR1 register accessor: an alias for `Reg<APB1SMENR1_SPEC>`"]
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1_SPEC>;
#[doc = "APB1 peripheral clocks enable in Sleep mode register 1"]
pub mod apb1smenr1;
#[doc = "APB1SMENR2 register accessor: an alias for `Reg<APB1SMENR2_SPEC>`"]
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2_SPEC>;
#[doc = "APB1 peripheral clocks enable in Sleep mode register 2"]
pub mod apb1smenr2;
#[doc = "APB2SMENR register accessor: an alias for `Reg<APB2SMENR_SPEC>`"]
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENR_SPEC>;
#[doc = "APB2 peripheral clocks enable in Sleep mode register"]
pub mod apb2smenr;
#[doc = "APB3SMENR register accessor: an alias for `Reg<APB3SMENR_SPEC>`"]
pub type APB3SMENR = crate::Reg<apb3smenr::APB3SMENR_SPEC>;
#[doc = "APB3 peripheral clock enable in Sleep mode register"]
pub mod apb3smenr;
#[doc = "CCIPR register accessor: an alias for `Reg<CCIPR_SPEC>`"]
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
#[doc = "Peripherals independent clock configuration register"]
pub mod ccipr;
#[doc = "BDCR register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control/status register"]
pub mod csr;
#[doc = "EXTCFGR register accessor: an alias for `Reg<EXTCFGR_SPEC>`"]
pub type EXTCFGR = crate::Reg<extcfgr::EXTCFGR_SPEC>;
#[doc = "Extended clock recovery register"]
pub mod extcfgr;
#[doc = "C2AHB1ENR register accessor: an alias for `Reg<C2AHB1ENR_SPEC>`"]
pub type C2AHB1ENR = crate::Reg<c2ahb1enr::C2AHB1ENR_SPEC>;
#[doc = "CPU2 AHB1 peripheral clock enable register"]
pub mod c2ahb1enr;
#[doc = "C2AHB2ENR register accessor: an alias for `Reg<C2AHB2ENR_SPEC>`"]
pub type C2AHB2ENR = crate::Reg<c2ahb2enr::C2AHB2ENR_SPEC>;
#[doc = "CPU2 AHB2 peripheral clock enable register"]
pub mod c2ahb2enr;
#[doc = "C2AHB3ENR register accessor: an alias for `Reg<C2AHB3ENR_SPEC>`"]
pub type C2AHB3ENR = crate::Reg<c2ahb3enr::C2AHB3ENR_SPEC>;
#[doc = "CPU2 AHB3 peripheral clock enable register \\[dual core device only\\]"]
pub mod c2ahb3enr;
#[doc = "C2APB1ENR1 register accessor: an alias for `Reg<C2APB1ENR1_SPEC>`"]
pub type C2APB1ENR1 = crate::Reg<c2apb1enr1::C2APB1ENR1_SPEC>;
#[doc = "CPU2 APB1 peripheral clock enable register 1 \\[dual core device only\\]"]
pub mod c2apb1enr1;
#[doc = "C2APB1ENR2 register accessor: an alias for `Reg<C2APB1ENR2_SPEC>`"]
pub type C2APB1ENR2 = crate::Reg<c2apb1enr2::C2APB1ENR2_SPEC>;
#[doc = "CPU2 APB1 peripheral clock enable register 2 \\[dual core device only\\]"]
pub mod c2apb1enr2;
#[doc = "C2APB2ENR register accessor: an alias for `Reg<C2APB2ENR_SPEC>`"]
pub type C2APB2ENR = crate::Reg<c2apb2enr::C2APB2ENR_SPEC>;
#[doc = "CPU2 APB2 peripheral clock enable register \\[dual core device only\\]"]
pub mod c2apb2enr;
#[doc = "C2APB3ENR register accessor: an alias for `Reg<C2APB3ENR_SPEC>`"]
pub type C2APB3ENR = crate::Reg<c2apb3enr::C2APB3ENR_SPEC>;
#[doc = "CPU2 APB3 peripheral clock enable register \\[dual core device only\\]"]
pub mod c2apb3enr;
#[doc = "C2AHB1SMENR register accessor: an alias for `Reg<C2AHB1SMENR_SPEC>`"]
pub type C2AHB1SMENR = crate::Reg<c2ahb1smenr::C2AHB1SMENR_SPEC>;
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep modes register \\[dual core device only\\]"]
pub mod c2ahb1smenr;
#[doc = "C2AHB2SMENR register accessor: an alias for `Reg<C2AHB2SMENR_SPEC>`"]
pub type C2AHB2SMENR = crate::Reg<c2ahb2smenr::C2AHB2SMENR_SPEC>;
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep modes register \\[dual core device only\\]"]
pub mod c2ahb2smenr;
#[doc = "C2AHB3SMENR register accessor: an alias for `Reg<C2AHB3SMENR_SPEC>`"]
pub type C2AHB3SMENR = crate::Reg<c2ahb3smenr::C2AHB3SMENR_SPEC>;
#[doc = "CPU2 AHB3 peripheral clocks enable in Sleep mode register \\[dual core device only\\]"]
pub mod c2ahb3smenr;
#[doc = "C2APB1SMENR1 register accessor: an alias for `Reg<C2APB1SMENR1_SPEC>`"]
pub type C2APB1SMENR1 = crate::Reg<c2apb1smenr1::C2APB1SMENR1_SPEC>;
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \\[dual core device only\\]"]
pub mod c2apb1smenr1;
#[doc = "C2APB1SMENR2 register accessor: an alias for `Reg<C2APB1SMENR2_SPEC>`"]
pub type C2APB1SMENR2 = crate::Reg<c2apb1smenr2::C2APB1SMENR2_SPEC>;
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \\[dual core device only\\]"]
pub mod c2apb1smenr2;
#[doc = "C2APB2SMENR register accessor: an alias for `Reg<C2APB2SMENR_SPEC>`"]
pub type C2APB2SMENR = crate::Reg<c2apb2smenr::C2APB2SMENR_SPEC>;
#[doc = "CPU2 APB2 peripheral clocks enable in Sleep mode register \\[dual core device only\\]"]
pub mod c2apb2smenr;
#[doc = "C2APB3SMENR register accessor: an alias for `Reg<C2APB3SMENR_SPEC>`"]
pub type C2APB3SMENR = crate::Reg<c2apb3smenr::C2APB3SMENR_SPEC>;
#[doc = "CPU2 APB3 peripheral clock enable in Sleep mode register \\[dual core device only\\]"]
pub mod c2apb3smenr;
