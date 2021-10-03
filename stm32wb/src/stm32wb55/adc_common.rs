#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC common status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - ADC common control register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
}
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "ADC common status register"]
pub mod csr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ADC common control register"]
pub mod ccr;
