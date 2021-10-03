#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x04 - Independent data register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x08 - Control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Initial CRC value"]
    pub init: crate::Reg<init::INIT_SPEC>,
    #[doc = "0x14 - polynomial"]
    pub pol: crate::Reg<pol::POL_SPEC>,
}
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Independent data register"]
pub mod idr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "INIT register accessor: an alias for `Reg<INIT_SPEC>`"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "Initial CRC value"]
pub mod init;
#[doc = "POL register accessor: an alias for `Reg<POL_SPEC>`"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "polynomial"]
pub mod pol;
