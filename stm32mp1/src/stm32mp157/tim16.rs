#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM16/TIM17 control register 1"]
    pub timx_cr1: crate::Reg<timx_cr1::TIMX_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM16/TIM17 control register 2"]
    pub timx_cr2: crate::Reg<timx_cr2::TIMX_CR2_SPEC>,
    _reserved2: [u8; 0x06],
    #[doc = "0x0c - TIM16/TIM17 DMA/interrupt enable register"]
    pub timx_dier: crate::Reg<timx_dier::TIMX_DIER_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - TIM16/TIM17 status register"]
    pub timx_sr: crate::Reg<timx_sr::TIMX_SR_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - event generation register"]
    pub timx_egr: crate::Reg<timx_egr::TIMX_EGR_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - TIM16/TIM17 capture/compare enable register"]
    pub timx_ccer: crate::Reg<timx_ccer::TIMX_CCER_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x24 - TIM16/TIM17 counter"]
    pub timx_cnt: crate::Reg<timx_cnt::TIMX_CNT_SPEC>,
    #[doc = "0x28 - TIM16/TIM17 prescaler"]
    pub timx_psc: crate::Reg<timx_psc::TIMX_PSC_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x2c - TIM16/TIM17 auto-reload register"]
    pub timx_arr: crate::Reg<timx_arr::TIMX_ARR_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x30 - TIM16/TIM17 repetition counter register"]
    pub timx_rcr: crate::Reg<timx_rcr::TIMX_RCR_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x34 - TIM16/TIM17 capture/compare register 1"]
    pub timx_ccr1: crate::Reg<timx_ccr1::TIMX_CCR1_SPEC>,
    _reserved11: [u8; 0x0e],
    #[doc = "0x44 - As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub timx_bdtr: crate::Reg<timx_bdtr::TIMX_BDTR_SPEC>,
    #[doc = "0x48 - TIM16/TIM17 DMA control register"]
    pub timx_dcr: crate::Reg<timx_dcr::TIMX_DCR_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x4c - TIM16/TIM17 DMA address for full transfer"]
    pub timx_dmar: crate::Reg<timx_dmar::TIMX_DMAR_SPEC>,
    _reserved14: [u8; 0x12],
    #[doc = "0x60 - TIM17 alternate function register 1"]
    pub timx_af1: crate::Reg<timx_af1::TIMX_AF1_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x68 - TIM17 input selection register"]
    pub timx_tisel: crate::Reg<timx_tisel::TIMX_TISEL_SPEC>,
}
#[doc = "TIMx_CR1 register accessor: an alias for `Reg<TIMX_CR1_SPEC>`"]
pub type TIMX_CR1 = crate::Reg<timx_cr1::TIMX_CR1_SPEC>;
#[doc = "TIM16/TIM17 control register 1"]
pub mod timx_cr1;
#[doc = "TIMx_CR2 register accessor: an alias for `Reg<TIMX_CR2_SPEC>`"]
pub type TIMX_CR2 = crate::Reg<timx_cr2::TIMX_CR2_SPEC>;
#[doc = "TIM16/TIM17 control register 2"]
pub mod timx_cr2;
#[doc = "TIMx_DIER register accessor: an alias for `Reg<TIMX_DIER_SPEC>`"]
pub type TIMX_DIER = crate::Reg<timx_dier::TIMX_DIER_SPEC>;
#[doc = "TIM16/TIM17 DMA/interrupt enable register"]
pub mod timx_dier;
#[doc = "TIMx_SR register accessor: an alias for `Reg<TIMX_SR_SPEC>`"]
pub type TIMX_SR = crate::Reg<timx_sr::TIMX_SR_SPEC>;
#[doc = "TIM16/TIM17 status register"]
pub mod timx_sr;
#[doc = "TIMx_EGR register accessor: an alias for `Reg<TIMX_EGR_SPEC>`"]
pub type TIMX_EGR = crate::Reg<timx_egr::TIMX_EGR_SPEC>;
#[doc = "event generation register"]
pub mod timx_egr;
#[doc = "TIMx_CCER register accessor: an alias for `Reg<TIMX_CCER_SPEC>`"]
pub type TIMX_CCER = crate::Reg<timx_ccer::TIMX_CCER_SPEC>;
#[doc = "TIM16/TIM17 capture/compare enable register"]
pub mod timx_ccer;
#[doc = "TIMx_CNT register accessor: an alias for `Reg<TIMX_CNT_SPEC>`"]
pub type TIMX_CNT = crate::Reg<timx_cnt::TIMX_CNT_SPEC>;
#[doc = "TIM16/TIM17 counter"]
pub mod timx_cnt;
#[doc = "TIMx_PSC register accessor: an alias for `Reg<TIMX_PSC_SPEC>`"]
pub type TIMX_PSC = crate::Reg<timx_psc::TIMX_PSC_SPEC>;
#[doc = "TIM16/TIM17 prescaler"]
pub mod timx_psc;
#[doc = "TIMx_ARR register accessor: an alias for `Reg<TIMX_ARR_SPEC>`"]
pub type TIMX_ARR = crate::Reg<timx_arr::TIMX_ARR_SPEC>;
#[doc = "TIM16/TIM17 auto-reload register"]
pub mod timx_arr;
#[doc = "TIMx_RCR register accessor: an alias for `Reg<TIMX_RCR_SPEC>`"]
pub type TIMX_RCR = crate::Reg<timx_rcr::TIMX_RCR_SPEC>;
#[doc = "TIM16/TIM17 repetition counter register"]
pub mod timx_rcr;
#[doc = "TIMx_CCR1 register accessor: an alias for `Reg<TIMX_CCR1_SPEC>`"]
pub type TIMX_CCR1 = crate::Reg<timx_ccr1::TIMX_CCR1_SPEC>;
#[doc = "TIM16/TIM17 capture/compare register 1"]
pub mod timx_ccr1;
#[doc = "TIMx_BDTR register accessor: an alias for `Reg<TIMX_BDTR_SPEC>`"]
pub type TIMX_BDTR = crate::Reg<timx_bdtr::TIMX_BDTR_SPEC>;
#[doc = "As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod timx_bdtr;
#[doc = "TIMx_DCR register accessor: an alias for `Reg<TIMX_DCR_SPEC>`"]
pub type TIMX_DCR = crate::Reg<timx_dcr::TIMX_DCR_SPEC>;
#[doc = "TIM16/TIM17 DMA control register"]
pub mod timx_dcr;
#[doc = "TIMx_DMAR register accessor: an alias for `Reg<TIMX_DMAR_SPEC>`"]
pub type TIMX_DMAR = crate::Reg<timx_dmar::TIMX_DMAR_SPEC>;
#[doc = "TIM16/TIM17 DMA address for full transfer"]
pub mod timx_dmar;
#[doc = "TIMx_AF1 register accessor: an alias for `Reg<TIMX_AF1_SPEC>`"]
pub type TIMX_AF1 = crate::Reg<timx_af1::TIMX_AF1_SPEC>;
#[doc = "TIM17 alternate function register 1"]
pub mod timx_af1;
#[doc = "TIMx_TISEL register accessor: an alias for `Reg<TIMX_TISEL_SPEC>`"]
pub type TIMX_TISEL = crate::Reg<timx_tisel::TIMX_TISEL_SPEC>;
#[doc = "TIM17 input selection register"]
pub mod timx_tisel;
