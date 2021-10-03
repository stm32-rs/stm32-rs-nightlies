#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CORDIC Control Status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x04 - CORDIC argument register"]
    pub wdata: crate::Reg<wdata::WDATA_SPEC>,
    #[doc = "0x08 - CORDIC result register"]
    pub rdata: crate::Reg<rdata::RDATA_SPEC>,
}
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CORDIC Control Status register"]
pub mod csr;
#[doc = "WDATA register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "CORDIC argument register"]
pub mod wdata;
#[doc = "RDATA register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "CORDIC result register"]
pub mod rdata;
