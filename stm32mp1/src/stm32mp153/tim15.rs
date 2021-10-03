#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM15 control register 1"]
    pub tim15_cr1: crate::Reg<tim15_cr1::TIM15_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM15 control register 2"]
    pub tim15_cr2: crate::Reg<tim15_cr2::TIM15_CR2_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - slave mode control register"]
    pub timx_smcr: crate::Reg<timx_smcr::TIMX_SMCR_SPEC>,
    #[doc = "0x0c - TIM15 DMA/interrupt enable register"]
    pub tim15_dier: crate::Reg<tim15_dier::TIM15_DIER_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM15 status register"]
    pub tim15_sr: crate::Reg<tim15_sr::TIM15_SR_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - event generation register"]
    pub timx_egr: crate::Reg<timx_egr::TIMX_EGR_SPEC>,
    _reserved_6_timx_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - TIM15 capture/compare enable register"]
    pub tim15_ccer: crate::Reg<tim15_ccer::TIM15_CCER_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x24 - TIM15 counter"]
    pub tim15_cnt: crate::Reg<tim15_cnt::TIM15_CNT_SPEC>,
    #[doc = "0x28 - TIM15 prescaler"]
    pub tim15_psc: crate::Reg<tim15_psc::TIM15_PSC_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x2c - TIM15 auto-reload register"]
    pub tim15_arr: crate::Reg<tim15_arr::TIM15_ARR_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x30 - TIM15 repetition counter register"]
    pub tim15_rcr: crate::Reg<tim15_rcr::TIM15_RCR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x34 - TIM15 capture/compare register 1"]
    pub tim15_ccr1: crate::Reg<tim15_ccr1::TIM15_CCR1_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x38 - TIM15 capture/compare register 2"]
    pub tim15_ccr2: crate::Reg<tim15_ccr2::TIM15_CCR2_SPEC>,
    _reserved14: [u8; 0x0a],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub timx_bdtr: crate::Reg<timx_bdtr::TIMX_BDTR_SPEC>,
    #[doc = "0x48 - TIM15 DMA control register"]
    pub tim15_dcr: crate::Reg<tim15_dcr::TIM15_DCR_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x4c - TIM15 DMA address for full transfer"]
    pub tim15_dmar: crate::Reg<tim15_dmar::TIM15_DMAR_SPEC>,
    _reserved17: [u8; 0x12],
    #[doc = "0x60 - TIM15 alternate register 1"]
    pub tim15_af1: crate::Reg<tim15_af1::TIM15_AF1_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x68 - TIM15 input selection register"]
    pub tim15_tisel: crate::Reg<tim15_tisel::TIM15_TISEL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_input(&self) -> &crate::Reg<timx_ccmr1_input::TIMX_CCMR1_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<timx_ccmr1_input::TIMX_CCMR1_INPUT_SPEC>)
        }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_output(&self) -> &crate::Reg<timx_ccmr1_output::TIMX_CCMR1_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<timx_ccmr1_output::TIMX_CCMR1_OUTPUT_SPEC>)
        }
    }
}
#[doc = "TIM15_CR1 register accessor: an alias for `Reg<TIM15_CR1_SPEC>`"]
pub type TIM15_CR1 = crate::Reg<tim15_cr1::TIM15_CR1_SPEC>;
#[doc = "TIM15 control register 1"]
pub mod tim15_cr1;
#[doc = "TIM15_CR2 register accessor: an alias for `Reg<TIM15_CR2_SPEC>`"]
pub type TIM15_CR2 = crate::Reg<tim15_cr2::TIM15_CR2_SPEC>;
#[doc = "TIM15 control register 2"]
pub mod tim15_cr2;
#[doc = "TIMx_SMCR register accessor: an alias for `Reg<TIMX_SMCR_SPEC>`"]
pub type TIMX_SMCR = crate::Reg<timx_smcr::TIMX_SMCR_SPEC>;
#[doc = "slave mode control register"]
pub mod timx_smcr;
#[doc = "TIM15_DIER register accessor: an alias for `Reg<TIM15_DIER_SPEC>`"]
pub type TIM15_DIER = crate::Reg<tim15_dier::TIM15_DIER_SPEC>;
#[doc = "TIM15 DMA/interrupt enable register"]
pub mod tim15_dier;
#[doc = "TIM15_SR register accessor: an alias for `Reg<TIM15_SR_SPEC>`"]
pub type TIM15_SR = crate::Reg<tim15_sr::TIM15_SR_SPEC>;
#[doc = "TIM15 status register"]
pub mod tim15_sr;
#[doc = "TIMx_EGR register accessor: an alias for `Reg<TIMX_EGR_SPEC>`"]
pub type TIMX_EGR = crate::Reg<timx_egr::TIMX_EGR_SPEC>;
#[doc = "event generation register"]
pub mod timx_egr;
#[doc = "TIMx_CCMR1_Output register accessor: an alias for `Reg<TIMX_CCMR1_OUTPUT_SPEC>`"]
pub type TIMX_CCMR1_OUTPUT = crate::Reg<timx_ccmr1_output::TIMX_CCMR1_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod timx_ccmr1_output;
#[doc = "TIMx_CCMR1_Input register accessor: an alias for `Reg<TIMX_CCMR1_INPUT_SPEC>`"]
pub type TIMX_CCMR1_INPUT = crate::Reg<timx_ccmr1_input::TIMX_CCMR1_INPUT_SPEC>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod timx_ccmr1_input;
#[doc = "TIM15_CCER register accessor: an alias for `Reg<TIM15_CCER_SPEC>`"]
pub type TIM15_CCER = crate::Reg<tim15_ccer::TIM15_CCER_SPEC>;
#[doc = "TIM15 capture/compare enable register"]
pub mod tim15_ccer;
#[doc = "TIM15_CNT register accessor: an alias for `Reg<TIM15_CNT_SPEC>`"]
pub type TIM15_CNT = crate::Reg<tim15_cnt::TIM15_CNT_SPEC>;
#[doc = "TIM15 counter"]
pub mod tim15_cnt;
#[doc = "TIM15_PSC register accessor: an alias for `Reg<TIM15_PSC_SPEC>`"]
pub type TIM15_PSC = crate::Reg<tim15_psc::TIM15_PSC_SPEC>;
#[doc = "TIM15 prescaler"]
pub mod tim15_psc;
#[doc = "TIM15_ARR register accessor: an alias for `Reg<TIM15_ARR_SPEC>`"]
pub type TIM15_ARR = crate::Reg<tim15_arr::TIM15_ARR_SPEC>;
#[doc = "TIM15 auto-reload register"]
pub mod tim15_arr;
#[doc = "TIM15_RCR register accessor: an alias for `Reg<TIM15_RCR_SPEC>`"]
pub type TIM15_RCR = crate::Reg<tim15_rcr::TIM15_RCR_SPEC>;
#[doc = "TIM15 repetition counter register"]
pub mod tim15_rcr;
#[doc = "TIM15_CCR1 register accessor: an alias for `Reg<TIM15_CCR1_SPEC>`"]
pub type TIM15_CCR1 = crate::Reg<tim15_ccr1::TIM15_CCR1_SPEC>;
#[doc = "TIM15 capture/compare register 1"]
pub mod tim15_ccr1;
#[doc = "TIM15_CCR2 register accessor: an alias for `Reg<TIM15_CCR2_SPEC>`"]
pub type TIM15_CCR2 = crate::Reg<tim15_ccr2::TIM15_CCR2_SPEC>;
#[doc = "TIM15 capture/compare register 2"]
pub mod tim15_ccr2;
#[doc = "TIMx_BDTR register accessor: an alias for `Reg<TIMX_BDTR_SPEC>`"]
pub type TIMX_BDTR = crate::Reg<timx_bdtr::TIMX_BDTR_SPEC>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod timx_bdtr;
#[doc = "TIM15_DCR register accessor: an alias for `Reg<TIM15_DCR_SPEC>`"]
pub type TIM15_DCR = crate::Reg<tim15_dcr::TIM15_DCR_SPEC>;
#[doc = "TIM15 DMA control register"]
pub mod tim15_dcr;
#[doc = "TIM15_DMAR register accessor: an alias for `Reg<TIM15_DMAR_SPEC>`"]
pub type TIM15_DMAR = crate::Reg<tim15_dmar::TIM15_DMAR_SPEC>;
#[doc = "TIM15 DMA address for full transfer"]
pub mod tim15_dmar;
#[doc = "TIM15_AF1 register accessor: an alias for `Reg<TIM15_AF1_SPEC>`"]
pub type TIM15_AF1 = crate::Reg<tim15_af1::TIM15_AF1_SPEC>;
#[doc = "TIM15 alternate register 1"]
pub mod tim15_af1;
#[doc = "TIM15_TISEL register accessor: an alias for `Reg<TIM15_TISEL_SPEC>`"]
pub type TIM15_TISEL = crate::Reg<tim15_tisel::TIM15_TISEL_SPEC>;
#[doc = "TIM15 input selection register"]
pub mod tim15_tisel;
