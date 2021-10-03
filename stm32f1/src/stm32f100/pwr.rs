#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register (PWR_CR)"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Power control register (PWR_CR)"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Power control register (PWR_CR)"]
pub mod cr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Power control register (PWR_CR)"]
pub mod csr;
