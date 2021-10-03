#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - Power control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x08 - Power control register 3"]
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    #[doc = "0x0c - Power control register 4"]
    pub cr4: crate::Reg<cr4::CR4_SPEC>,
    #[doc = "0x10 - Power status register 1"]
    pub sr1: crate::Reg<sr1::SR1_SPEC>,
    #[doc = "0x14 - Power status register 2"]
    pub sr2: crate::Reg<sr2::SR2_SPEC>,
    #[doc = "0x18 - Power status clear register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Power Port A pull-up control register"]
    pub pucra: crate::Reg<pucra::PUCRA_SPEC>,
    #[doc = "0x24 - Power Port A pull-down control register"]
    pub pdcra: crate::Reg<pdcra::PDCRA_SPEC>,
    #[doc = "0x28 - Power Port B pull-up control register"]
    pub pucrb: crate::Reg<pucrb::PUCRB_SPEC>,
    #[doc = "0x2c - Power Port B pull-down control register"]
    pub pdcrb: crate::Reg<pdcrb::PDCRB_SPEC>,
    #[doc = "0x30 - Power Port C pull-up control register"]
    pub pucrc: crate::Reg<pucrc::PUCRC_SPEC>,
    #[doc = "0x34 - Power Port C pull-down control register"]
    pub pdcrc: crate::Reg<pdcrc::PDCRC_SPEC>,
    #[doc = "0x38 - Power Port D pull-up control register"]
    pub pucrd: crate::Reg<pucrd::PUCRD_SPEC>,
    #[doc = "0x3c - Power Port D pull-down control register"]
    pub pdcrd: crate::Reg<pdcrd::PDCRD_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x48 - Power Port F pull-up control register"]
    pub pucrf: crate::Reg<pucrf::PUCRF_SPEC>,
    #[doc = "0x4c - Power Port F pull-down control register"]
    pub pdcrf: crate::Reg<pdcrf::PDCRF_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Power control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Power control register 2"]
pub mod cr2;
#[doc = "CR3 register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "Power control register 3"]
pub mod cr3;
#[doc = "CR4 register accessor: an alias for `Reg<CR4_SPEC>`"]
pub type CR4 = crate::Reg<cr4::CR4_SPEC>;
#[doc = "Power control register 4"]
pub mod cr4;
#[doc = "SR1 register accessor: an alias for `Reg<SR1_SPEC>`"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "Power status register 1"]
pub mod sr1;
#[doc = "SR2 register accessor: an alias for `Reg<SR2_SPEC>`"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "Power status register 2"]
pub mod sr2;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Power status clear register"]
pub mod scr;
#[doc = "PUCRA register accessor: an alias for `Reg<PUCRA_SPEC>`"]
pub type PUCRA = crate::Reg<pucra::PUCRA_SPEC>;
#[doc = "Power Port A pull-up control register"]
pub mod pucra;
#[doc = "PDCRA register accessor: an alias for `Reg<PDCRA_SPEC>`"]
pub type PDCRA = crate::Reg<pdcra::PDCRA_SPEC>;
#[doc = "Power Port A pull-down control register"]
pub mod pdcra;
#[doc = "PUCRB register accessor: an alias for `Reg<PUCRB_SPEC>`"]
pub type PUCRB = crate::Reg<pucrb::PUCRB_SPEC>;
#[doc = "Power Port B pull-up control register"]
pub mod pucrb;
#[doc = "PDCRB register accessor: an alias for `Reg<PDCRB_SPEC>`"]
pub type PDCRB = crate::Reg<pdcrb::PDCRB_SPEC>;
#[doc = "Power Port B pull-down control register"]
pub mod pdcrb;
#[doc = "PUCRC register accessor: an alias for `Reg<PUCRC_SPEC>`"]
pub type PUCRC = crate::Reg<pucrc::PUCRC_SPEC>;
#[doc = "Power Port C pull-up control register"]
pub mod pucrc;
#[doc = "PDCRC register accessor: an alias for `Reg<PDCRC_SPEC>`"]
pub type PDCRC = crate::Reg<pdcrc::PDCRC_SPEC>;
#[doc = "Power Port C pull-down control register"]
pub mod pdcrc;
#[doc = "PUCRD register accessor: an alias for `Reg<PUCRD_SPEC>`"]
pub type PUCRD = crate::Reg<pucrd::PUCRD_SPEC>;
#[doc = "Power Port D pull-up control register"]
pub mod pucrd;
#[doc = "PDCRD register accessor: an alias for `Reg<PDCRD_SPEC>`"]
pub type PDCRD = crate::Reg<pdcrd::PDCRD_SPEC>;
#[doc = "Power Port D pull-down control register"]
pub mod pdcrd;
#[doc = "PUCRF register accessor: an alias for `Reg<PUCRF_SPEC>`"]
pub type PUCRF = crate::Reg<pucrf::PUCRF_SPEC>;
#[doc = "Power Port F pull-up control register"]
pub mod pucrf;
#[doc = "PDCRF register accessor: an alias for `Reg<PDCRF_SPEC>`"]
pub type PDCRF = crate::Reg<pdcrf::PDCRF_SPEC>;
#[doc = "Power Port F pull-down control register"]
pub mod pdcrf;
