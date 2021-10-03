#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - control and status register"]
    pub comp2_csr: crate::Reg<comp2_csr::COMP2_CSR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x28 - control and status register"]
    pub comp4_csr: crate::Reg<comp4_csr::COMP4_CSR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x30 - control and status register"]
    pub comp6_csr: crate::Reg<comp6_csr::COMP6_CSR_SPEC>,
}
#[doc = "COMP2_CSR register accessor: an alias for `Reg<COMP2_CSR_SPEC>`"]
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp2_csr;
#[doc = "COMP4_CSR register accessor: an alias for `Reg<COMP4_CSR_SPEC>`"]
pub type COMP4_CSR = crate::Reg<comp4_csr::COMP4_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp4_csr;
#[doc = "COMP6_CSR register accessor: an alias for `Reg<COMP6_CSR_SPEC>`"]
pub type COMP6_CSR = crate::Reg<comp6_csr::COMP6_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp6_csr;
