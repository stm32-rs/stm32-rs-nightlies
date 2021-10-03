#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM6 control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - TIM6 control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - TIM6 DMA/Interrupt enable register"]
    pub dier: crate::Reg<dier::DIER_SPEC>,
    #[doc = "0x10 - TIM6 status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - TIM6 event generation register"]
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x24 - TIM6 counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - TIM6 prescaler"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x2c - TIM6 auto-reload register"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM6 control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "TIM6 control register 2"]
pub mod cr2;
#[doc = "DIER register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "TIM6 DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TIM6 status register"]
pub mod sr;
#[doc = "EGR register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM6 event generation register"]
pub mod egr;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "TIM6 counter"]
pub mod cnt;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "TIM6 prescaler"]
pub mod psc;
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "TIM6 auto-reload register"]
pub mod arr;
