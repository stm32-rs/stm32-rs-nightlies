#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMux - DMA request line multiplexer channel x control register"]
    pub dmamux_c0cr: crate::Reg<dmamux_c0cr::DMAMUX_C0CR_SPEC>,
    #[doc = "0x04 - DMAMux - DMA request line multiplexer channel x control register"]
    pub dmamux_c1cr: crate::Reg<dmamux_c1cr::DMAMUX_C1CR_SPEC>,
    #[doc = "0x08 - DMAMux - DMA request line multiplexer channel x control register"]
    pub dmamux_c2cr: crate::Reg<dmamux_c2cr::DMAMUX_C2CR_SPEC>,
    #[doc = "0x0c - DMAMux - DMA request line multiplexer channel x control register"]
    pub dmamux_c3cr: crate::Reg<dmamux_c3cr::DMAMUX_C3CR_SPEC>,
    #[doc = "0x10 - DMAMux - DMA request line multiplexer channel x control register"]
    pub dmamux_c4cr: crate::Reg<dmamux_c4cr::DMAMUX_C4CR_SPEC>,
    #[doc = "0x14 - DMAMux - DMA request line multiplexer channel x control register"]
    pub dmamux_c5cr: crate::Reg<dmamux_c5cr::DMAMUX_C5CR_SPEC>,
    #[doc = "0x18 - DMAMux - DMA request line multiplexer channel x control register"]
    pub dmamux_c6cr: crate::Reg<dmamux_c6cr::DMAMUX_C6CR_SPEC>,
    _reserved7: [u8; 0x64],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub dmamux_csr: crate::Reg<dmamux_csr::DMAMUX_CSR_SPEC>,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub dmamux_cfr: crate::Reg<dmamux_cfr::DMAMUX_CFR_SPEC>,
    _reserved9: [u8; 0x78],
    #[doc = "0x100 - DMAMux - DMA request generator channel x control register"]
    pub dmamux_rg0cr: crate::Reg<dmamux_rg0cr::DMAMUX_RG0CR_SPEC>,
    #[doc = "0x104 - DMAMux - DMA request generator channel x control register"]
    pub dmamux_rg1cr: crate::Reg<dmamux_rg1cr::DMAMUX_RG1CR_SPEC>,
    #[doc = "0x108 - DMAMux - DMA request generator channel x control register"]
    pub dmamux_rg2cr: crate::Reg<dmamux_rg2cr::DMAMUX_RG2CR_SPEC>,
    #[doc = "0x10c - DMAMux - DMA request generator channel x control register"]
    pub dmamux_rg3cr: crate::Reg<dmamux_rg3cr::DMAMUX_RG3CR_SPEC>,
    _reserved13: [u8; 0x30],
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    pub dmamux_rgsr: crate::Reg<dmamux_rgsr::DMAMUX_RGSR_SPEC>,
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    pub dmamux_rgcfr: crate::Reg<dmamux_rgcfr::DMAMUX_RGCFR_SPEC>,
    _reserved15: [u8; 0x02a4],
    #[doc = "0x3ec - DMAMUX hardware configuration 2 register"]
    pub dmamux_hwcfgr2: crate::Reg<dmamux_hwcfgr2::DMAMUX_HWCFGR2_SPEC>,
    #[doc = "0x3f0 - DMAMUX hardware configuration 1 register"]
    pub dmamux_hwcfgr1: crate::Reg<dmamux_hwcfgr1::DMAMUX_HWCFGR1_SPEC>,
    #[doc = "0x3f4 - DMAMUX version register"]
    pub dmamux_verr: crate::Reg<dmamux_verr::DMAMUX_VERR_SPEC>,
    #[doc = "0x3f8 - DMAMUX IP identification register"]
    pub dmamux_ipidr: crate::Reg<dmamux_ipidr::DMAMUX_IPIDR_SPEC>,
    #[doc = "0x3fc - DMAMUX size identification register"]
    pub dmamux_sidr: crate::Reg<dmamux_sidr::DMAMUX_SIDR_SPEC>,
}
#[doc = "DMAMUX_C0CR register accessor: an alias for `Reg<DMAMUX_C0CR_SPEC>`"]
pub type DMAMUX_C0CR = crate::Reg<dmamux_c0cr::DMAMUX_C0CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod dmamux_c0cr;
#[doc = "DMAMUX_C1CR register accessor: an alias for `Reg<DMAMUX_C1CR_SPEC>`"]
pub type DMAMUX_C1CR = crate::Reg<dmamux_c1cr::DMAMUX_C1CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod dmamux_c1cr;
#[doc = "DMAMUX_C2CR register accessor: an alias for `Reg<DMAMUX_C2CR_SPEC>`"]
pub type DMAMUX_C2CR = crate::Reg<dmamux_c2cr::DMAMUX_C2CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod dmamux_c2cr;
#[doc = "DMAMUX_C3CR register accessor: an alias for `Reg<DMAMUX_C3CR_SPEC>`"]
pub type DMAMUX_C3CR = crate::Reg<dmamux_c3cr::DMAMUX_C3CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod dmamux_c3cr;
#[doc = "DMAMUX_C4CR register accessor: an alias for `Reg<DMAMUX_C4CR_SPEC>`"]
pub type DMAMUX_C4CR = crate::Reg<dmamux_c4cr::DMAMUX_C4CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod dmamux_c4cr;
#[doc = "DMAMUX_C5CR register accessor: an alias for `Reg<DMAMUX_C5CR_SPEC>`"]
pub type DMAMUX_C5CR = crate::Reg<dmamux_c5cr::DMAMUX_C5CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod dmamux_c5cr;
#[doc = "DMAMUX_C6CR register accessor: an alias for `Reg<DMAMUX_C6CR_SPEC>`"]
pub type DMAMUX_C6CR = crate::Reg<dmamux_c6cr::DMAMUX_C6CR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod dmamux_c6cr;
#[doc = "DMAMUX_RG0CR register accessor: an alias for `Reg<DMAMUX_RG0CR_SPEC>`"]
pub type DMAMUX_RG0CR = crate::Reg<dmamux_rg0cr::DMAMUX_RG0CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod dmamux_rg0cr;
#[doc = "DMAMUX_RG1CR register accessor: an alias for `Reg<DMAMUX_RG1CR_SPEC>`"]
pub type DMAMUX_RG1CR = crate::Reg<dmamux_rg1cr::DMAMUX_RG1CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod dmamux_rg1cr;
#[doc = "DMAMUX_RG2CR register accessor: an alias for `Reg<DMAMUX_RG2CR_SPEC>`"]
pub type DMAMUX_RG2CR = crate::Reg<dmamux_rg2cr::DMAMUX_RG2CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod dmamux_rg2cr;
#[doc = "DMAMUX_RG3CR register accessor: an alias for `Reg<DMAMUX_RG3CR_SPEC>`"]
pub type DMAMUX_RG3CR = crate::Reg<dmamux_rg3cr::DMAMUX_RG3CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod dmamux_rg3cr;
#[doc = "DMAMUX_RGSR register accessor: an alias for `Reg<DMAMUX_RGSR_SPEC>`"]
pub type DMAMUX_RGSR = crate::Reg<dmamux_rgsr::DMAMUX_RGSR_SPEC>;
#[doc = "DMAMux - DMA request generator status register"]
pub mod dmamux_rgsr;
#[doc = "DMAMUX_RGCFR register accessor: an alias for `Reg<DMAMUX_RGCFR_SPEC>`"]
pub type DMAMUX_RGCFR = crate::Reg<dmamux_rgcfr::DMAMUX_RGCFR_SPEC>;
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod dmamux_rgcfr;
#[doc = "DMAMUX_CSR register accessor: an alias for `Reg<DMAMUX_CSR_SPEC>`"]
pub type DMAMUX_CSR = crate::Reg<dmamux_csr::DMAMUX_CSR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod dmamux_csr;
#[doc = "DMAMUX_CFR register accessor: an alias for `Reg<DMAMUX_CFR_SPEC>`"]
pub type DMAMUX_CFR = crate::Reg<dmamux_cfr::DMAMUX_CFR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod dmamux_cfr;
#[doc = "DMAMUX_SIDR register accessor: an alias for `Reg<DMAMUX_SIDR_SPEC>`"]
pub type DMAMUX_SIDR = crate::Reg<dmamux_sidr::DMAMUX_SIDR_SPEC>;
#[doc = "DMAMUX size identification register"]
pub mod dmamux_sidr;
#[doc = "DMAMUX_IPIDR register accessor: an alias for `Reg<DMAMUX_IPIDR_SPEC>`"]
pub type DMAMUX_IPIDR = crate::Reg<dmamux_ipidr::DMAMUX_IPIDR_SPEC>;
#[doc = "DMAMUX IP identification register"]
pub mod dmamux_ipidr;
#[doc = "DMAMUX_VERR register accessor: an alias for `Reg<DMAMUX_VERR_SPEC>`"]
pub type DMAMUX_VERR = crate::Reg<dmamux_verr::DMAMUX_VERR_SPEC>;
#[doc = "DMAMUX version register"]
pub mod dmamux_verr;
#[doc = "DMAMUX_HWCFGR1 register accessor: an alias for `Reg<DMAMUX_HWCFGR1_SPEC>`"]
pub type DMAMUX_HWCFGR1 = crate::Reg<dmamux_hwcfgr1::DMAMUX_HWCFGR1_SPEC>;
#[doc = "DMAMUX hardware configuration 1 register"]
pub mod dmamux_hwcfgr1;
#[doc = "DMAMUX_HWCFGR2 register accessor: an alias for `Reg<DMAMUX_HWCFGR2_SPEC>`"]
pub type DMAMUX_HWCFGR2 = crate::Reg<dmamux_hwcfgr2::DMAMUX_HWCFGR2_SPEC>;
#[doc = "DMAMUX hardware configuration 2 register"]
pub mod dmamux_hwcfgr2;
