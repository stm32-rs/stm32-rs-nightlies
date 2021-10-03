#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt and Status Register"]
    pub lptim_isr: crate::Reg<lptim_isr::LPTIM_ISR_SPEC>,
    #[doc = "0x04 - Interrupt Clear Register"]
    pub lptim_icr: crate::Reg<lptim_icr::LPTIM_ICR_SPEC>,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub lptim_ier: crate::Reg<lptim_ier::LPTIM_IER_SPEC>,
    #[doc = "0x0c - Configuration Register"]
    pub lptim_cfgr: crate::Reg<lptim_cfgr::LPTIM_CFGR_SPEC>,
    #[doc = "0x10 - Control Register"]
    pub lptim_cr: crate::Reg<lptim_cr::LPTIM_CR_SPEC>,
    #[doc = "0x14 - Compare Register"]
    pub lptim_cmp: crate::Reg<lptim_cmp::LPTIM_CMP_SPEC>,
    #[doc = "0x18 - Autoreload Register"]
    pub lptim_arr: crate::Reg<lptim_arr::LPTIM_ARR_SPEC>,
    #[doc = "0x1c - Counter Register"]
    pub lptim_cnt: crate::Reg<lptim_cnt::LPTIM_CNT_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - LPTIM configuration register 2"]
    pub lptim_cfgr2: crate::Reg<lptim_cfgr2::LPTIM_CFGR2_SPEC>,
}
#[doc = "LPTIM_ISR register accessor: an alias for `Reg<LPTIM_ISR_SPEC>`"]
pub type LPTIM_ISR = crate::Reg<lptim_isr::LPTIM_ISR_SPEC>;
#[doc = "Interrupt and Status Register"]
pub mod lptim_isr;
#[doc = "LPTIM_ICR register accessor: an alias for `Reg<LPTIM_ICR_SPEC>`"]
pub type LPTIM_ICR = crate::Reg<lptim_icr::LPTIM_ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod lptim_icr;
#[doc = "LPTIM_IER register accessor: an alias for `Reg<LPTIM_IER_SPEC>`"]
pub type LPTIM_IER = crate::Reg<lptim_ier::LPTIM_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod lptim_ier;
#[doc = "LPTIM_CFGR register accessor: an alias for `Reg<LPTIM_CFGR_SPEC>`"]
pub type LPTIM_CFGR = crate::Reg<lptim_cfgr::LPTIM_CFGR_SPEC>;
#[doc = "Configuration Register"]
pub mod lptim_cfgr;
#[doc = "LPTIM_CR register accessor: an alias for `Reg<LPTIM_CR_SPEC>`"]
pub type LPTIM_CR = crate::Reg<lptim_cr::LPTIM_CR_SPEC>;
#[doc = "Control Register"]
pub mod lptim_cr;
#[doc = "LPTIM_CMP register accessor: an alias for `Reg<LPTIM_CMP_SPEC>`"]
pub type LPTIM_CMP = crate::Reg<lptim_cmp::LPTIM_CMP_SPEC>;
#[doc = "Compare Register"]
pub mod lptim_cmp;
#[doc = "LPTIM_ARR register accessor: an alias for `Reg<LPTIM_ARR_SPEC>`"]
pub type LPTIM_ARR = crate::Reg<lptim_arr::LPTIM_ARR_SPEC>;
#[doc = "Autoreload Register"]
pub mod lptim_arr;
#[doc = "LPTIM_CNT register accessor: an alias for `Reg<LPTIM_CNT_SPEC>`"]
pub type LPTIM_CNT = crate::Reg<lptim_cnt::LPTIM_CNT_SPEC>;
#[doc = "Counter Register"]
pub mod lptim_cnt;
#[doc = "LPTIM_CFGR2 register accessor: an alias for `Reg<LPTIM_CFGR2_SPEC>`"]
pub type LPTIM_CFGR2 = crate::Reg<lptim_cfgr2::LPTIM_CFGR2_SPEC>;
#[doc = "LPTIM configuration register 2"]
pub mod lptim_cfgr2;
