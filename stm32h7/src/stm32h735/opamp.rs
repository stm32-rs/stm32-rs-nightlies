#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    pub opamp1_csr: crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>,
    #[doc = "0x04 - OPAMP1 offset trimming register in normal mode"]
    pub opamp1_otr: crate::Reg<opamp1_otr::OPAMP1_OTR_SPEC>,
    #[doc = "0x08 - OPAMP1 offset trimming register in low-power mode"]
    pub opamp1_hsotr: crate::Reg<opamp1_hsotr::OPAMP1_HSOTR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OPAMP2 control/status register"]
    pub opamp2_csr: crate::Reg<opamp2_csr::OPAMP2_CSR_SPEC>,
    #[doc = "0x14 - OPAMP2 offset trimming register in normal mode"]
    pub opamp2_otr: crate::Reg<opamp2_otr::OPAMP2_OTR_SPEC>,
    #[doc = "0x18 - OPAMP2 offset trimming register in low-power mode"]
    pub opamp2_hsotr: crate::Reg<opamp2_hsotr::OPAMP2_HSOTR_SPEC>,
}
#[doc = "OPAMP1_CSR register accessor: an alias for `Reg<OPAMP1_CSR_SPEC>`"]
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP1_OTR register accessor: an alias for `Reg<OPAMP1_OTR_SPEC>`"]
pub type OPAMP1_OTR = crate::Reg<opamp1_otr::OPAMP1_OTR_SPEC>;
#[doc = "OPAMP1 offset trimming register in normal mode"]
pub mod opamp1_otr;
#[doc = "OPAMP1_HSOTR register accessor: an alias for `Reg<OPAMP1_HSOTR_SPEC>`"]
pub type OPAMP1_HSOTR = crate::Reg<opamp1_hsotr::OPAMP1_HSOTR_SPEC>;
#[doc = "OPAMP1 offset trimming register in low-power mode"]
pub mod opamp1_hsotr;
#[doc = "OPAMP2_CSR register accessor: an alias for `Reg<OPAMP2_CSR_SPEC>`"]
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSR_SPEC>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP2_OTR register accessor: an alias for `Reg<OPAMP2_OTR_SPEC>`"]
pub type OPAMP2_OTR = crate::Reg<opamp2_otr::OPAMP2_OTR_SPEC>;
#[doc = "OPAMP2 offset trimming register in normal mode"]
pub mod opamp2_otr;
#[doc = "OPAMP2_HSOTR register accessor: an alias for `Reg<OPAMP2_HSOTR_SPEC>`"]
pub type OPAMP2_HSOTR = crate::Reg<opamp2_hsotr::OPAMP2_HSOTR_SPEC>;
#[doc = "OPAMP2 offset trimming register in low-power mode"]
pub mod opamp2_hsotr;
