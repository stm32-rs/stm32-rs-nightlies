#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Interrupt mask register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x08 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - Interrupt Flag Clear register"]
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    _reserved_4_dr_: [u8; 0x04],
    #[doc = "0x14 - Channel Status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x18 - Debug Information register"]
    pub dir: crate::Reg<dir::DIR_SPEC>,
    _reserved7: [u8; 0x03d8],
    #[doc = "0x3f4 - SPDIFRX version register"]
    pub verr: crate::Reg<verr::VERR_SPEC>,
    #[doc = "0x3f8 - SPDIFRX identification register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x3fc - SPDIFRX size identification register"]
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_10(&self) -> &crate::Reg<dr_10::DR_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<dr_10::DR_10_SPEC>)
        }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_01(&self) -> &crate::Reg<dr_01::DR_01_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<dr_01::DR_01_SPEC>)
        }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_00(&self) -> &crate::Reg<dr_00::DR_00_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<dr_00::DR_00_SPEC>)
        }
    }
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IFCR register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Interrupt Flag Clear register"]
pub mod ifcr;
#[doc = "DR_00 register accessor: an alias for `Reg<DR_00_SPEC>`"]
pub type DR_00 = crate::Reg<dr_00::DR_00_SPEC>;
#[doc = "Data input register"]
pub mod dr_00;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Channel Status register"]
pub mod csr;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Debug Information register"]
pub mod dir;
#[doc = "VERR register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "SPDIFRX version register"]
pub mod verr;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "SPDIFRX identification register"]
pub mod idr;
#[doc = "SIDR register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "SPDIFRX size identification register"]
pub mod sidr;
#[doc = "DR_01 register accessor: an alias for `Reg<DR_01_SPEC>`"]
pub type DR_01 = crate::Reg<dr_01::DR_01_SPEC>;
#[doc = "Data input register"]
pub mod dr_01;
#[doc = "DR_10 register accessor: an alias for `Reg<DR_10_SPEC>`"]
pub type DR_10 = crate::Reg<dr_10::DR_10_SPEC>;
#[doc = "Data input register"]
pub mod dr_10;
