#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub spdifrx_cr: crate::Reg<spdifrx_cr::SPDIFRX_CR_SPEC>,
    #[doc = "0x04 - Interrupt mask register"]
    pub spdifrx_imr: crate::Reg<spdifrx_imr::SPDIFRX_IMR_SPEC>,
    #[doc = "0x08 - Status register"]
    pub spdifrx_sr: crate::Reg<spdifrx_sr::SPDIFRX_SR_SPEC>,
    #[doc = "0x0c - Interrupt flag clear register"]
    pub spdifrx_ifcr: crate::Reg<spdifrx_ifcr::SPDIFRX_IFCR_SPEC>,
    #[doc = "0x10 - This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:"]
    pub spdifrx_fmt0_dr: crate::Reg<spdifrx_fmt0_dr::SPDIFRX_FMT0_DR_SPEC>,
    #[doc = "0x14 - Channel status register"]
    pub spdifrx_csr: crate::Reg<spdifrx_csr::SPDIFRX_CSR_SPEC>,
    #[doc = "0x18 - Debug information register"]
    pub spdifrx_dir: crate::Reg<spdifrx_dir::SPDIFRX_DIR_SPEC>,
    _reserved7: [u8; 0x03d8],
    #[doc = "0x3f4 - SPDIFRX version register"]
    pub spdifrx_verr: crate::Reg<spdifrx_verr::SPDIFRX_VERR_SPEC>,
    #[doc = "0x3f8 - SPDIFRX identification register"]
    pub spdifrx_ipidr: crate::Reg<spdifrx_ipidr::SPDIFRX_IPIDR_SPEC>,
    #[doc = "0x3fc - SPDIFRX size identification register"]
    pub spdifrx_sidr: crate::Reg<spdifrx_sidr::SPDIFRX_SIDR_SPEC>,
}
#[doc = "SPDIFRX_CR register accessor: an alias for `Reg<SPDIFRX_CR_SPEC>`"]
pub type SPDIFRX_CR = crate::Reg<spdifrx_cr::SPDIFRX_CR_SPEC>;
#[doc = "Control register"]
pub mod spdifrx_cr;
#[doc = "SPDIFRX_IMR register accessor: an alias for `Reg<SPDIFRX_IMR_SPEC>`"]
pub type SPDIFRX_IMR = crate::Reg<spdifrx_imr::SPDIFRX_IMR_SPEC>;
#[doc = "Interrupt mask register"]
pub mod spdifrx_imr;
#[doc = "SPDIFRX_SR register accessor: an alias for `Reg<SPDIFRX_SR_SPEC>`"]
pub type SPDIFRX_SR = crate::Reg<spdifrx_sr::SPDIFRX_SR_SPEC>;
#[doc = "Status register"]
pub mod spdifrx_sr;
#[doc = "SPDIFRX_IFCR register accessor: an alias for `Reg<SPDIFRX_IFCR_SPEC>`"]
pub type SPDIFRX_IFCR = crate::Reg<spdifrx_ifcr::SPDIFRX_IFCR_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod spdifrx_ifcr;
#[doc = "SPDIFRX_FMT0_DR register accessor: an alias for `Reg<SPDIFRX_FMT0_DR_SPEC>`"]
pub type SPDIFRX_FMT0_DR = crate::Reg<spdifrx_fmt0_dr::SPDIFRX_FMT0_DR_SPEC>;
#[doc = "This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:"]
pub mod spdifrx_fmt0_dr;
#[doc = "SPDIFRX_CSR register accessor: an alias for `Reg<SPDIFRX_CSR_SPEC>`"]
pub type SPDIFRX_CSR = crate::Reg<spdifrx_csr::SPDIFRX_CSR_SPEC>;
#[doc = "Channel status register"]
pub mod spdifrx_csr;
#[doc = "SPDIFRX_DIR register accessor: an alias for `Reg<SPDIFRX_DIR_SPEC>`"]
pub type SPDIFRX_DIR = crate::Reg<spdifrx_dir::SPDIFRX_DIR_SPEC>;
#[doc = "Debug information register"]
pub mod spdifrx_dir;
#[doc = "SPDIFRX_VERR register accessor: an alias for `Reg<SPDIFRX_VERR_SPEC>`"]
pub type SPDIFRX_VERR = crate::Reg<spdifrx_verr::SPDIFRX_VERR_SPEC>;
#[doc = "SPDIFRX version register"]
pub mod spdifrx_verr;
#[doc = "SPDIFRX_IPIDR register accessor: an alias for `Reg<SPDIFRX_IPIDR_SPEC>`"]
pub type SPDIFRX_IPIDR = crate::Reg<spdifrx_ipidr::SPDIFRX_IPIDR_SPEC>;
#[doc = "SPDIFRX identification register"]
pub mod spdifrx_ipidr;
#[doc = "SPDIFRX_SIDR register accessor: an alias for `Reg<SPDIFRX_SIDR_SPEC>`"]
pub type SPDIFRX_SIDR = crate::Reg<spdifrx_sidr::SPDIFRX_SIDR_SPEC>;
#[doc = "SPDIFRX size identification register"]
pub mod spdifrx_sidr;
