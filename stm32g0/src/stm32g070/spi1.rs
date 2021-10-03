#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x08 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x10 - CRC polynomial register"]
    pub crcpr: crate::Reg<crcpr::CRCPR_SPEC>,
    #[doc = "0x14 - RX CRC register"]
    pub rxcrcr: crate::Reg<rxcrcr::RXCRCR_SPEC>,
    #[doc = "0x18 - TX CRC register"]
    pub txcrcr: crate::Reg<txcrcr::TXCRCR_SPEC>,
    #[doc = "0x1c - configuration register"]
    pub i2scfgr: crate::Reg<i2scfgr::I2SCFGR_SPEC>,
    #[doc = "0x20 - prescaler register"]
    pub i2spr: crate::Reg<i2spr::I2SPR_SPEC>,
    _reserved9: [u8; 0x03cc],
    #[doc = "0x3f0 - hardware configuration register"]
    pub hwcfgr: crate::Reg<hwcfgr::HWCFGR_SPEC>,
    #[doc = "0x3f4 - EXTI IP Version register"]
    pub verr: crate::Reg<verr::VERR_SPEC>,
    #[doc = "0x3f8 - EXTI Identification register"]
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    #[doc = "0x3fc - EXTI Size ID register"]
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "data register"]
pub mod dr;
#[doc = "CRCPR register accessor: an alias for `Reg<CRCPR_SPEC>`"]
pub type CRCPR = crate::Reg<crcpr::CRCPR_SPEC>;
#[doc = "CRC polynomial register"]
pub mod crcpr;
#[doc = "RXCRCR register accessor: an alias for `Reg<RXCRCR_SPEC>`"]
pub type RXCRCR = crate::Reg<rxcrcr::RXCRCR_SPEC>;
#[doc = "RX CRC register"]
pub mod rxcrcr;
#[doc = "TXCRCR register accessor: an alias for `Reg<TXCRCR_SPEC>`"]
pub type TXCRCR = crate::Reg<txcrcr::TXCRCR_SPEC>;
#[doc = "TX CRC register"]
pub mod txcrcr;
#[doc = "I2SCFGR register accessor: an alias for `Reg<I2SCFGR_SPEC>`"]
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
#[doc = "configuration register"]
pub mod i2scfgr;
#[doc = "I2SPR register accessor: an alias for `Reg<I2SPR_SPEC>`"]
pub type I2SPR = crate::Reg<i2spr::I2SPR_SPEC>;
#[doc = "prescaler register"]
pub mod i2spr;
#[doc = "HWCFGR register accessor: an alias for `Reg<HWCFGR_SPEC>`"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
#[doc = "hardware configuration register"]
pub mod hwcfgr;
#[doc = "VERR register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "IPIDR register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "EXTI Identification register"]
pub mod ipidr;
#[doc = "SIDR register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "EXTI Size ID register"]
pub mod sidr;
