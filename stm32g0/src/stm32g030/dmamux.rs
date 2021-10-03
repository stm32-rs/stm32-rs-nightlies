#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c0cr: crate::Reg<c0cr::C0CR_SPEC>,
    #[doc = "0x04 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c1cr: crate::Reg<c1cr::C1CR_SPEC>,
    #[doc = "0x08 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c2cr: crate::Reg<c2cr::C2CR_SPEC>,
    #[doc = "0x0c - DMAMux - DMA request line multiplexer channel x control register"]
    pub c3cr: crate::Reg<c3cr::C3CR_SPEC>,
    #[doc = "0x10 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c4cr: crate::Reg<c4cr::C4CR_SPEC>,
    #[doc = "0x14 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c5cr: crate::Reg<c5cr::C5CR_SPEC>,
    #[doc = "0x18 - DMAMux - DMA request line multiplexer channel x control register"]
    pub c6cr: crate::Reg<c6cr::C6CR_SPEC>,
    _reserved7: [u8; 0xe4],
    #[doc = "0x100 - DMAMux - DMA request generator channel x control register"]
    pub rg0cr: crate::Reg<rg0cr::RG0CR_SPEC>,
    #[doc = "0x104 - DMAMux - DMA request generator channel x control register"]
    pub rg1cr: crate::Reg<rg1cr::RG1CR_SPEC>,
    #[doc = "0x108 - DMAMux - DMA request generator channel x control register"]
    pub rg2cr: crate::Reg<rg2cr::RG2CR_SPEC>,
    #[doc = "0x10c - DMAMux - DMA request generator channel x control register"]
    pub rg3cr: crate::Reg<rg3cr::RG3CR_SPEC>,
    _reserved11: [u8; 0x30],
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    pub rgsr: crate::Reg<rgsr::RGSR_SPEC>,
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    pub rgcfr: crate::Reg<rgcfr::RGCFR_SPEC>,
}
#[doc = "C0CR register accessor: an alias for `Reg<C0CR_SPEC>`"]
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c0cr;
#[doc = "C1CR register accessor: an alias for `Reg<C1CR_SPEC>`"]
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c1cr;
#[doc = "C2CR register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c2cr;
#[doc = "C3CR register accessor: an alias for `Reg<C3CR_SPEC>`"]
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c3cr;
#[doc = "C4CR register accessor: an alias for `Reg<C4CR_SPEC>`"]
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c4cr;
#[doc = "C5CR register accessor: an alias for `Reg<C5CR_SPEC>`"]
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c5cr;
#[doc = "C6CR register accessor: an alias for `Reg<C6CR_SPEC>`"]
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c6cr;
#[doc = "RG0CR register accessor: an alias for `Reg<RG0CR_SPEC>`"]
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg0cr;
#[doc = "RG1CR register accessor: an alias for `Reg<RG1CR_SPEC>`"]
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg1cr;
#[doc = "RG2CR register accessor: an alias for `Reg<RG2CR_SPEC>`"]
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg2cr;
#[doc = "RG3CR register accessor: an alias for `Reg<RG3CR_SPEC>`"]
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg3cr;
#[doc = "RGSR register accessor: an alias for `Reg<RGSR_SPEC>`"]
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
#[doc = "DMAMux - DMA request generator status register"]
pub mod rgsr;
#[doc = "RGCFR register accessor: an alias for `Reg<RGCFR_SPEC>`"]
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod rgcfr;
