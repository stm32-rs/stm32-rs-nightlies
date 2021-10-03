#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator control/status register"]
    pub comp_c1csr: crate::Reg<comp_c1csr::COMP_C1CSR_SPEC>,
    #[doc = "0x04 - Comparator control/status register"]
    pub comp_c2csr: crate::Reg<comp_c2csr::COMP_C2CSR_SPEC>,
    #[doc = "0x08 - Comparator control/status register"]
    pub comp_c3csr: crate::Reg<comp_c3csr::COMP_C3CSR_SPEC>,
    #[doc = "0x0c - Comparator control/status register"]
    pub comp_c4csr: crate::Reg<comp_c4csr::COMP_C4CSR_SPEC>,
    #[doc = "0x10 - Comparator control/status register"]
    pub comp_c5csr: crate::Reg<comp_c5csr::COMP_C5CSR_SPEC>,
    #[doc = "0x14 - Comparator control/status register"]
    pub comp_c6csr: crate::Reg<comp_c6csr::COMP_C6CSR_SPEC>,
    #[doc = "0x18 - Comparator control/status register"]
    pub comp_c7csr: crate::Reg<comp_c7csr::COMP_C7CSR_SPEC>,
}
#[doc = "COMP_C1CSR register accessor: an alias for `Reg<COMP_C1CSR_SPEC>`"]
pub type COMP_C1CSR = crate::Reg<comp_c1csr::COMP_C1CSR_SPEC>;
#[doc = "Comparator control/status register"]
pub mod comp_c1csr;
#[doc = "COMP_C2CSR register accessor: an alias for `Reg<COMP_C2CSR_SPEC>`"]
pub type COMP_C2CSR = crate::Reg<comp_c2csr::COMP_C2CSR_SPEC>;
#[doc = "Comparator control/status register"]
pub mod comp_c2csr;
#[doc = "COMP_C3CSR register accessor: an alias for `Reg<COMP_C3CSR_SPEC>`"]
pub type COMP_C3CSR = crate::Reg<comp_c3csr::COMP_C3CSR_SPEC>;
#[doc = "Comparator control/status register"]
pub mod comp_c3csr;
#[doc = "COMP_C4CSR register accessor: an alias for `Reg<COMP_C4CSR_SPEC>`"]
pub type COMP_C4CSR = crate::Reg<comp_c4csr::COMP_C4CSR_SPEC>;
#[doc = "Comparator control/status register"]
pub mod comp_c4csr;
#[doc = "COMP_C5CSR register accessor: an alias for `Reg<COMP_C5CSR_SPEC>`"]
pub type COMP_C5CSR = crate::Reg<comp_c5csr::COMP_C5CSR_SPEC>;
#[doc = "Comparator control/status register"]
pub mod comp_c5csr;
#[doc = "COMP_C6CSR register accessor: an alias for `Reg<COMP_C6CSR_SPEC>`"]
pub type COMP_C6CSR = crate::Reg<comp_c6csr::COMP_C6CSR_SPEC>;
#[doc = "Comparator control/status register"]
pub mod comp_c6csr;
#[doc = "COMP_C7CSR register accessor: an alias for `Reg<COMP_C7CSR_SPEC>`"]
pub type COMP_C7CSR = crate::Reg<comp_c7csr::COMP_C7CSR_SPEC>;
#[doc = "Comparator control/status register"]
pub mod comp_c7csr;
