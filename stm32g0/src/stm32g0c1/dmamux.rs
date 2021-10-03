#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c0cr: crate::Reg<dmamux_c0cr::DMAMUX_C0CR_SPEC>,
    #[doc = "0x04 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c1cr: crate::Reg<dmamux_c1cr::DMAMUX_C1CR_SPEC>,
    #[doc = "0x08 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c2cr: crate::Reg<dmamux_c2cr::DMAMUX_C2CR_SPEC>,
    #[doc = "0x0c - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c3cr: crate::Reg<dmamux_c3cr::DMAMUX_C3CR_SPEC>,
    #[doc = "0x10 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c4cr: crate::Reg<dmamux_c4cr::DMAMUX_C4CR_SPEC>,
    #[doc = "0x14 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c5cr: crate::Reg<dmamux_c5cr::DMAMUX_C5CR_SPEC>,
    #[doc = "0x18 - DMAMUX request line multiplexer channel x configuration register"]
    pub dmamux_c6cr: crate::Reg<dmamux_c6cr::DMAMUX_C6CR_SPEC>,
    _reserved7: [u8; 0x64],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub dmamux_csr: crate::Reg<dmamux_csr::DMAMUX_CSR_SPEC>,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub dmamux_cfr: crate::Reg<dmamux_cfr::DMAMUX_CFR_SPEC>,
    _reserved9: [u8; 0x78],
    #[doc = "0x100 - DMAMUX request generator channel x configuration register"]
    pub dmamux_rg0cr: crate::Reg<dmamux_rg0cr::DMAMUX_RG0CR_SPEC>,
    #[doc = "0x104 - DMAMUX request generator channel x configuration register"]
    pub dmamux_rg1cr: crate::Reg<dmamux_rg1cr::DMAMUX_RG1CR_SPEC>,
    #[doc = "0x108 - DMAMUX request generator channel x configuration register"]
    pub dmamux_rg2cr: crate::Reg<dmamux_rg2cr::DMAMUX_RG2CR_SPEC>,
    #[doc = "0x10c - DMAMUX request generator channel x configuration register"]
    pub dmamux_rg3cr: crate::Reg<dmamux_rg3cr::DMAMUX_RG3CR_SPEC>,
    _reserved13: [u8; 0x30],
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    pub dmamux_rgsr: crate::Reg<dmamux_rgsr::DMAMUX_RGSR_SPEC>,
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    pub dmamux_rgcfr: crate::Reg<dmamux_rgcfr::DMAMUX_RGCFR_SPEC>,
}
#[doc = "DMAMUX_C0CR register accessor: an alias for `Reg<DMAMUX_C0CR_SPEC>`"]
pub type DMAMUX_C0CR = crate::Reg<dmamux_c0cr::DMAMUX_C0CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c0cr;
#[doc = "DMAMUX_C1CR register accessor: an alias for `Reg<DMAMUX_C1CR_SPEC>`"]
pub type DMAMUX_C1CR = crate::Reg<dmamux_c1cr::DMAMUX_C1CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c1cr;
#[doc = "DMAMUX_C2CR register accessor: an alias for `Reg<DMAMUX_C2CR_SPEC>`"]
pub type DMAMUX_C2CR = crate::Reg<dmamux_c2cr::DMAMUX_C2CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c2cr;
#[doc = "DMAMUX_C3CR register accessor: an alias for `Reg<DMAMUX_C3CR_SPEC>`"]
pub type DMAMUX_C3CR = crate::Reg<dmamux_c3cr::DMAMUX_C3CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c3cr;
#[doc = "DMAMUX_C4CR register accessor: an alias for `Reg<DMAMUX_C4CR_SPEC>`"]
pub type DMAMUX_C4CR = crate::Reg<dmamux_c4cr::DMAMUX_C4CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c4cr;
#[doc = "DMAMUX_C5CR register accessor: an alias for `Reg<DMAMUX_C5CR_SPEC>`"]
pub type DMAMUX_C5CR = crate::Reg<dmamux_c5cr::DMAMUX_C5CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c5cr;
#[doc = "DMAMUX_C6CR register accessor: an alias for `Reg<DMAMUX_C6CR_SPEC>`"]
pub type DMAMUX_C6CR = crate::Reg<dmamux_c6cr::DMAMUX_C6CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c6cr;
#[doc = "DMAMUX_CSR register accessor: an alias for `Reg<DMAMUX_CSR_SPEC>`"]
pub type DMAMUX_CSR = crate::Reg<dmamux_csr::DMAMUX_CSR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod dmamux_csr;
#[doc = "DMAMUX_CFR register accessor: an alias for `Reg<DMAMUX_CFR_SPEC>`"]
pub type DMAMUX_CFR = crate::Reg<dmamux_cfr::DMAMUX_CFR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod dmamux_cfr;
#[doc = "DMAMUX_RG0CR register accessor: an alias for `Reg<DMAMUX_RG0CR_SPEC>`"]
pub type DMAMUX_RG0CR = crate::Reg<dmamux_rg0cr::DMAMUX_RG0CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg0cr;
#[doc = "DMAMUX_RG1CR register accessor: an alias for `Reg<DMAMUX_RG1CR_SPEC>`"]
pub type DMAMUX_RG1CR = crate::Reg<dmamux_rg1cr::DMAMUX_RG1CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg1cr;
#[doc = "DMAMUX_RG2CR register accessor: an alias for `Reg<DMAMUX_RG2CR_SPEC>`"]
pub type DMAMUX_RG2CR = crate::Reg<dmamux_rg2cr::DMAMUX_RG2CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg2cr;
#[doc = "DMAMUX_RG3CR register accessor: an alias for `Reg<DMAMUX_RG3CR_SPEC>`"]
pub type DMAMUX_RG3CR = crate::Reg<dmamux_rg3cr::DMAMUX_RG3CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg3cr;
#[doc = "DMAMUX_RGSR register accessor: an alias for `Reg<DMAMUX_RGSR_SPEC>`"]
pub type DMAMUX_RGSR = crate::Reg<dmamux_rgsr::DMAMUX_RGSR_SPEC>;
#[doc = "DMAMUX request generator interrupt status register"]
pub mod dmamux_rgsr;
#[doc = "DMAMUX_RGCFR register accessor: an alias for `Reg<DMAMUX_RGCFR_SPEC>`"]
pub type DMAMUX_RGCFR = crate::Reg<dmamux_rgcfr::DMAMUX_RGCFR_SPEC>;
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod dmamux_rgcfr;
