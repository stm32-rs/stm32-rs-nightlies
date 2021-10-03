#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Multiplexer Channel 0 Control register"]
    pub c0cr: crate::Reg<c0cr::C0CR_SPEC>,
    #[doc = "0x04 - DMA Multiplexer Channel 1 Control register"]
    pub c1cr: crate::Reg<c1cr::C1CR_SPEC>,
    #[doc = "0x08 - DMA Multiplexer Channel 2 Control register"]
    pub c2cr: crate::Reg<c2cr::C2CR_SPEC>,
    #[doc = "0x0c - DMA Multiplexer Channel 3 Control register"]
    pub c3cr: crate::Reg<c3cr::C3CR_SPEC>,
    #[doc = "0x10 - DMA Multiplexer Channel 4 Control register"]
    pub c4cr: crate::Reg<c4cr::C4CR_SPEC>,
    #[doc = "0x14 - DMA Multiplexer Channel 5 Control register"]
    pub c5cr: crate::Reg<c5cr::C5CR_SPEC>,
    #[doc = "0x18 - DMA Multiplexer Channel 6 Control register"]
    pub c6cr: crate::Reg<c6cr::C6CR_SPEC>,
    #[doc = "0x1c - DMA Multiplexer Channel 7 Control register"]
    pub c7cr: crate::Reg<c7cr::C7CR_SPEC>,
    #[doc = "0x20 - DMA Multiplexer Channel 8 Control register"]
    pub c8cr: crate::Reg<c8cr::C8CR_SPEC>,
    #[doc = "0x24 - DMA Multiplexer Channel 9 Control register"]
    pub c9cr: crate::Reg<c9cr::C9CR_SPEC>,
    #[doc = "0x28 - DMA Multiplexer Channel 10 Control register"]
    pub c10cr: crate::Reg<c10cr::C10CR_SPEC>,
    #[doc = "0x2c - DMA Multiplexer Channel 11 Control register"]
    pub c11cr: crate::Reg<c11cr::C11CR_SPEC>,
    #[doc = "0x30 - DMA Multiplexer Channel 12 Control register"]
    pub c12cr: crate::Reg<c12cr::C12CR_SPEC>,
    #[doc = "0x34 - DMA Multiplexer Channel 13 Control register"]
    pub c13cr: crate::Reg<c13cr::C13CR_SPEC>,
    _reserved14: [u8; 0x48],
    #[doc = "0x80 - DMA Multiplexer Channel Status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x84 - DMA Channel Clear Flag Register"]
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    _reserved16: [u8; 0x78],
    #[doc = "0x100 - DMA Request Generator 0 Control Register"]
    pub rg0cr: crate::Reg<rg0cr::RG0CR_SPEC>,
    #[doc = "0x104 - DMA Request Generator 1 Control Register"]
    pub rg1cr: crate::Reg<rg1cr::RG1CR_SPEC>,
    #[doc = "0x108 - DMA Request Generator 2 Control Register"]
    pub rg2cr: crate::Reg<rg2cr::RG2CR_SPEC>,
    #[doc = "0x10c - DMA Request Generator 3 Control Register"]
    pub rg3cr: crate::Reg<rg3cr::RG3CR_SPEC>,
    _reserved20: [u8; 0x30],
    #[doc = "0x140 - DMA Request Generator Status Register"]
    pub rgsr: crate::Reg<rgsr::RGSR_SPEC>,
    #[doc = "0x144 - DMA Request Generator Clear Flag Register"]
    pub rgcfr: crate::Reg<rgcfr::RGCFR_SPEC>,
}
#[doc = "C0CR register accessor: an alias for `Reg<C0CR_SPEC>`"]
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
#[doc = "DMA Multiplexer Channel 0 Control register"]
pub mod c0cr;
#[doc = "C1CR register accessor: an alias for `Reg<C1CR_SPEC>`"]
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
#[doc = "DMA Multiplexer Channel 1 Control register"]
pub mod c1cr;
#[doc = "C2CR register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "DMA Multiplexer Channel 2 Control register"]
pub mod c2cr;
#[doc = "C3CR register accessor: an alias for `Reg<C3CR_SPEC>`"]
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
#[doc = "DMA Multiplexer Channel 3 Control register"]
pub mod c3cr;
#[doc = "C4CR register accessor: an alias for `Reg<C4CR_SPEC>`"]
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
#[doc = "DMA Multiplexer Channel 4 Control register"]
pub mod c4cr;
#[doc = "C5CR register accessor: an alias for `Reg<C5CR_SPEC>`"]
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
#[doc = "DMA Multiplexer Channel 5 Control register"]
pub mod c5cr;
#[doc = "C6CR register accessor: an alias for `Reg<C6CR_SPEC>`"]
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
#[doc = "DMA Multiplexer Channel 6 Control register"]
pub mod c6cr;
#[doc = "C7CR register accessor: an alias for `Reg<C7CR_SPEC>`"]
pub type C7CR = crate::Reg<c7cr::C7CR_SPEC>;
#[doc = "DMA Multiplexer Channel 7 Control register"]
pub mod c7cr;
#[doc = "C8CR register accessor: an alias for `Reg<C8CR_SPEC>`"]
pub type C8CR = crate::Reg<c8cr::C8CR_SPEC>;
#[doc = "DMA Multiplexer Channel 8 Control register"]
pub mod c8cr;
#[doc = "C9CR register accessor: an alias for `Reg<C9CR_SPEC>`"]
pub type C9CR = crate::Reg<c9cr::C9CR_SPEC>;
#[doc = "DMA Multiplexer Channel 9 Control register"]
pub mod c9cr;
#[doc = "C10CR register accessor: an alias for `Reg<C10CR_SPEC>`"]
pub type C10CR = crate::Reg<c10cr::C10CR_SPEC>;
#[doc = "DMA Multiplexer Channel 10 Control register"]
pub mod c10cr;
#[doc = "C11CR register accessor: an alias for `Reg<C11CR_SPEC>`"]
pub type C11CR = crate::Reg<c11cr::C11CR_SPEC>;
#[doc = "DMA Multiplexer Channel 11 Control register"]
pub mod c11cr;
#[doc = "C12CR register accessor: an alias for `Reg<C12CR_SPEC>`"]
pub type C12CR = crate::Reg<c12cr::C12CR_SPEC>;
#[doc = "DMA Multiplexer Channel 12 Control register"]
pub mod c12cr;
#[doc = "C13CR register accessor: an alias for `Reg<C13CR_SPEC>`"]
pub type C13CR = crate::Reg<c13cr::C13CR_SPEC>;
#[doc = "DMA Multiplexer Channel 13 Control register"]
pub mod c13cr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "DMA Multiplexer Channel Status register"]
pub mod csr;
#[doc = "CFR register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "DMA Channel Clear Flag Register"]
pub mod cfr;
#[doc = "RG0CR register accessor: an alias for `Reg<RG0CR_SPEC>`"]
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
#[doc = "DMA Request Generator 0 Control Register"]
pub mod rg0cr;
#[doc = "RG1CR register accessor: an alias for `Reg<RG1CR_SPEC>`"]
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
#[doc = "DMA Request Generator 1 Control Register"]
pub mod rg1cr;
#[doc = "RG2CR register accessor: an alias for `Reg<RG2CR_SPEC>`"]
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
#[doc = "DMA Request Generator 2 Control Register"]
pub mod rg2cr;
#[doc = "RG3CR register accessor: an alias for `Reg<RG3CR_SPEC>`"]
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
#[doc = "DMA Request Generator 3 Control Register"]
pub mod rg3cr;
#[doc = "RGSR register accessor: an alias for `Reg<RGSR_SPEC>`"]
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
#[doc = "DMA Request Generator Status Register"]
pub mod rgsr;
#[doc = "RGCFR register accessor: an alias for `Reg<RGCFR_SPEC>`"]
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
#[doc = "DMA Request Generator Clear Flag Register"]
pub mod rgcfr;
