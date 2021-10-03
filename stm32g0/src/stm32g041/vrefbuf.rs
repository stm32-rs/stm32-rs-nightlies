#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VREFBUF control and status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x04 - VREFBUF calibration control register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
}
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "VREFBUF control and status register"]
pub mod csr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "VREFBUF calibration control register"]
pub mod ccr;
