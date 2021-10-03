#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator 1 control and status register"]
    pub comp1_csr: crate::Reg<comp1_csr::COMP1_CSR_SPEC>,
    #[doc = "0x04 - Comparator 2 control and status register"]
    pub comp2_csr: crate::Reg<comp2_csr::COMP2_CSR_SPEC>,
}
#[doc = "COMP1_CSR register accessor: an alias for `Reg<COMP1_CSR_SPEC>`"]
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSR_SPEC>;
#[doc = "Comparator 1 control and status register"]
pub mod comp1_csr;
#[doc = "COMP2_CSR register accessor: an alias for `Reg<COMP2_CSR_SPEC>`"]
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSR_SPEC>;
#[doc = "Comparator 2 control and status register"]
pub mod comp2_csr;
