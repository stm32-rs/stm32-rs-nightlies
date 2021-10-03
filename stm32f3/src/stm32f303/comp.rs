#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    #[doc = "0x1c - control and status register"]
    pub comp1_csr: crate::Reg<comp1_csr::COMP1_CSR_SPEC>,
    #[doc = "0x20 - control and status register"]
    pub comp2_csr: crate::Reg<comp2_csr::COMP2_CSR_SPEC>,
    #[doc = "0x24 - control and status register"]
    pub comp3_csr: crate::Reg<comp3_csr::COMP3_CSR_SPEC>,
    #[doc = "0x28 - control and status register"]
    pub comp4_csr: crate::Reg<comp4_csr::COMP4_CSR_SPEC>,
    #[doc = "0x2c - control and status register"]
    pub comp5_csr: crate::Reg<comp5_csr::COMP5_CSR_SPEC>,
    #[doc = "0x30 - control and status register"]
    pub comp6_csr: crate::Reg<comp6_csr::COMP6_CSR_SPEC>,
    #[doc = "0x34 - control and status register"]
    pub comp7_csr: crate::Reg<comp7_csr::COMP7_CSR_SPEC>,
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
#[doc = "COMP3_CSR register accessor: an alias for `Reg<COMP3_CSR_SPEC>`"]
pub type COMP3_CSR = crate::Reg<comp3_csr::COMP3_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp3_csr;
#[doc = "COMP5_CSR register accessor: an alias for `Reg<COMP5_CSR_SPEC>`"]
pub type COMP5_CSR = crate::Reg<comp5_csr::COMP5_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp5_csr;
#[doc = "COMP7_CSR register accessor: an alias for `Reg<COMP7_CSR_SPEC>`"]
pub type COMP7_CSR = crate::Reg<comp7_csr::COMP7_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp7_csr;
#[doc = "COMP1_CSR register accessor: an alias for `Reg<COMP1_CSR_SPEC>`"]
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp1_csr;
