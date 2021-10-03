#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control/status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x04 - offset trimming register for normal mode"]
    pub otr: crate::Reg<otr::OTR_SPEC>,
    #[doc = "0x08 - OPAMP offset trimming register for low power mode"]
    pub lpotr: crate::Reg<lpotr::LPOTR_SPEC>,
}
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "control/status register"]
pub mod csr;
#[doc = "OTR register accessor: an alias for `Reg<OTR_SPEC>`"]
pub type OTR = crate::Reg<otr::OTR_SPEC>;
#[doc = "offset trimming register for normal mode"]
pub mod otr;
#[doc = "LPOTR register accessor: an alias for `Reg<LPOTR_SPEC>`"]
pub type LPOTR = crate::Reg<lpotr::LPOTR_SPEC>;
#[doc = "OPAMP offset trimming register for low power mode"]
pub mod lpotr;
