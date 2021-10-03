#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - DMAMux - DMA request line multiplexer channel x control register"]
    pub ccr: [crate::Reg<ccr::CCR_SPEC>; 8],
    _reserved1: [u8; 0x60],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    _reserved3: [u8; 0x78],
    #[doc = "0x100..0x120 - DMAMux - DMA request generator channel x control register"]
    pub rgcr: [crate::Reg<rgcr::RGCR_SPEC>; 8],
    _reserved4: [u8; 0x20],
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    pub rgsr: crate::Reg<rgsr::RGSR_SPEC>,
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    pub rgcfr: crate::Reg<rgcfr::RGCFR_SPEC>,
}
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod ccr;
#[doc = "RGCR register accessor: an alias for `Reg<RGCR_SPEC>`"]
pub type RGCR = crate::Reg<rgcr::RGCR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rgcr;
#[doc = "RGSR register accessor: an alias for `Reg<RGSR_SPEC>`"]
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
#[doc = "DMAMux - DMA request generator status register"]
pub mod rgsr;
#[doc = "RGCFR register accessor: an alias for `Reg<RGCFR_SPEC>`"]
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod rgcfr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "CFR register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
