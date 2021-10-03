#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SWPMI Configuration/Control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - SWPMI Bitrate register"]
    pub brr: crate::Reg<brr::BRR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - SWPMI Interrupt and Status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x10 - SWPMI Interrupt Flag Clear register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x14 - SWPMI Interrupt Enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x18 - SWPMI Receive Frame Length register"]
    pub rfl: crate::Reg<rfl::RFL_SPEC>,
    #[doc = "0x1c - SWPMI Transmit data register"]
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    #[doc = "0x20 - SWPMI Receive data register"]
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
    #[doc = "0x24 - SWPMI Option register"]
    pub or: crate::Reg<or::OR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "SWPMI Configuration/Control register"]
pub mod cr;
#[doc = "BRR register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "SWPMI Bitrate register"]
pub mod brr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "SWPMI Interrupt and Status register"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "SWPMI Interrupt Flag Clear register"]
pub mod icr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "SWPMI Interrupt Enable register"]
pub mod ier;
#[doc = "RFL register accessor: an alias for `Reg<RFL_SPEC>`"]
pub type RFL = crate::Reg<rfl::RFL_SPEC>;
#[doc = "SWPMI Receive Frame Length register"]
pub mod rfl;
#[doc = "TDR register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "SWPMI Transmit data register"]
pub mod tdr;
#[doc = "RDR register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "SWPMI Receive data register"]
pub mod rdr;
#[doc = "OR register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "SWPMI Option register"]
pub mod or;
