#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Common status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - ADC common control register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x0c - ADC common regular data register for dual and triple modes"]
    pub cdr: crate::Reg<cdr::CDR_SPEC>,
    #[doc = "0x10 - ADC x common regular data register for 32-bit dual mode"]
    pub cdr2: crate::Reg<cdr2::CDR2_SPEC>,
}
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "ADC Common status register"]
pub mod csr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ADC common control register"]
pub mod ccr;
#[doc = "CDR register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "ADC common regular data register for dual and triple modes"]
pub mod cdr;
#[doc = "CDR2 register accessor: an alias for `Reg<CDR2_SPEC>`"]
pub type CDR2 = crate::Reg<cdr2::CDR2_SPEC>;
#[doc = "ADC x common regular data register for 32-bit dual mode"]
pub mod cdr2;
