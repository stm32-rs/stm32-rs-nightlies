#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VREF_BUF Control and Status Register"]
    pub vrefbuf_csr: crate::Reg<vrefbuf_csr::VREFBUF_CSR_SPEC>,
    #[doc = "0x04 - VREF_BUF Calibration Control Register"]
    pub vrefbuf_ccr: crate::Reg<vrefbuf_ccr::VREFBUF_CCR_SPEC>,
}
#[doc = "VREFBUF_CSR register accessor: an alias for `Reg<VREFBUF_CSR_SPEC>`"]
pub type VREFBUF_CSR = crate::Reg<vrefbuf_csr::VREFBUF_CSR_SPEC>;
#[doc = "VREF_BUF Control and Status Register"]
pub mod vrefbuf_csr;
#[doc = "VREFBUF_CCR register accessor: an alias for `Reg<VREFBUF_CCR_SPEC>`"]
pub type VREFBUF_CCR = crate::Reg<vrefbuf_ccr::VREFBUF_CCR_SPEC>;
#[doc = "VREF_BUF Calibration Control Register"]
pub mod vrefbuf_ccr;
