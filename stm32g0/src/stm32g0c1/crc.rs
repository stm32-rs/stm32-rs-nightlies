#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub crc_dr: crate::Reg<crc_dr::CRC_DR_SPEC>,
    #[doc = "0x04 - Independent data register"]
    pub crc_idr: crate::Reg<crc_idr::CRC_IDR_SPEC>,
    #[doc = "0x08 - Control register"]
    pub crc_cr: crate::Reg<crc_cr::CRC_CR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Initial CRC value"]
    pub crc_init: crate::Reg<crc_init::CRC_INIT_SPEC>,
    #[doc = "0x14 - polynomial"]
    pub crc_pol: crate::Reg<crc_pol::CRC_POL_SPEC>,
}
#[doc = "CRC_DR register accessor: an alias for `Reg<CRC_DR_SPEC>`"]
pub type CRC_DR = crate::Reg<crc_dr::CRC_DR_SPEC>;
#[doc = "Data register"]
pub mod crc_dr;
#[doc = "CRC_IDR register accessor: an alias for `Reg<CRC_IDR_SPEC>`"]
pub type CRC_IDR = crate::Reg<crc_idr::CRC_IDR_SPEC>;
#[doc = "Independent data register"]
pub mod crc_idr;
#[doc = "CRC_CR register accessor: an alias for `Reg<CRC_CR_SPEC>`"]
pub type CRC_CR = crate::Reg<crc_cr::CRC_CR_SPEC>;
#[doc = "Control register"]
pub mod crc_cr;
#[doc = "CRC_INIT register accessor: an alias for `Reg<CRC_INIT_SPEC>`"]
pub type CRC_INIT = crate::Reg<crc_init::CRC_INIT_SPEC>;
#[doc = "Initial CRC value"]
pub mod crc_init;
#[doc = "CRC_POL register accessor: an alias for `Reg<CRC_POL_SPEC>`"]
pub type CRC_POL = crate::Reg<crc_pol::CRC_POL_SPEC>;
#[doc = "polynomial"]
pub mod crc_pol;
