#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x28 - Backup data register (BKP_DR)"]
    pub dr: [crate::Reg<dr::DR_SPEC>; 10],
    #[doc = "0x28 - RTC clock calibration register (BKP_RTCCR)"]
    pub rtccr: crate::Reg<rtccr::RTCCR_SPEC>,
    #[doc = "0x2c - Backup control register (BKP_CR)"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x30 - BKP_CSR control/status register (BKP_CSR)"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x3c..0xbc - Backup data register (BKP_DR)"]
    pub bkp_dr: [crate::Reg<bkp_dr::BKP_DR_SPEC>; 32],
}
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr;
#[doc = "BKP_DR register accessor: an alias for `Reg<BKP_DR_SPEC>`"]
pub type BKP_DR = crate::Reg<bkp_dr::BKP_DR_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod bkp_dr;
#[doc = "RTCCR register accessor: an alias for `Reg<RTCCR_SPEC>`"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "RTC clock calibration register (BKP_RTCCR)"]
pub mod rtccr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Backup control register (BKP_CR)"]
pub mod cr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "BKP_CSR control/status register (BKP_CSR)"]
pub mod csr;
