#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - health test control register"]
    pub htcr: crate::Reg<htcr::HTCR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "data register"]
pub mod dr;
#[doc = "HTCR register accessor: an alias for `Reg<HTCR_SPEC>`"]
pub type HTCR = crate::Reg<htcr::HTCR_SPEC>;
#[doc = "health test control register"]
pub mod htcr;
