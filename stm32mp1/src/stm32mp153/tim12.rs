#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM12 control register 1"]
    pub tim12_cr1: crate::Reg<tim12_cr1::TIM12_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM12 control register 2"]
    pub tim12_cr2: crate::Reg<tim12_cr2::TIM12_CR2_SPEC>,
    #[doc = "0x08 - TIM12 slave mode control register"]
    pub tim12_smcr: crate::Reg<tim12_smcr::TIM12_SMCR_SPEC>,
    #[doc = "0x0c - TIM12 interrupt enable register"]
    pub tim12_dier: crate::Reg<tim12_dier::TIM12_DIER_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM12 status register"]
    pub tim12_sr: crate::Reg<tim12_sr::TIM12_SR_SPEC>,
    #[doc = "0x14 - TIM12 event generation register"]
    pub tim12_egr: crate::Reg<tim12_egr::TIM12_EGR_SPEC>,
    _reserved6: [u8; 0x02],
    _reserved_6_tim12_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - TIM12 capture/compare enable register"]
    pub tim12_ccer: crate::Reg<tim12_ccer::TIM12_CCER_SPEC>,
    #[doc = "0x24 - TIM12 counter"]
    pub tim12_cnt: crate::Reg<tim12_cnt::TIM12_CNT_SPEC>,
    #[doc = "0x28 - TIM12 prescaler"]
    pub tim12_psc: crate::Reg<tim12_psc::TIM12_PSC_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x2c - TIM12 auto-reload register"]
    pub tim12_arr: crate::Reg<tim12_arr::TIM12_ARR_SPEC>,
    _reserved11: [u8; 0x06],
    #[doc = "0x34 - TIM12 capture/compare register 1"]
    pub tim12_ccr1: crate::Reg<tim12_ccr1::TIM12_CCR1_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x38 - TIM12 capture/compare register 2"]
    pub tim12_ccr2: crate::Reg<tim12_ccr2::TIM12_CCR2_SPEC>,
    _reserved13: [u8; 0x2e],
    #[doc = "0x68 - TIM12 timer input selection register"]
    pub tim12_tisel: crate::Reg<tim12_tisel::TIM12_TISEL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM12 capture/compare mode register 1"]
    #[inline(always)]
    pub fn tim12_ccmr1_output(&self) -> &crate::Reg<tim12_ccmr1_output::TIM12_CCMR1_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<tim12_ccmr1_output::TIM12_CCMR1_OUTPUT_SPEC>)
        }
    }
    #[doc = "0x18 - TIM12 capture/compare mode register 1"]
    #[inline(always)]
    pub fn tim12_ccmr1_input(&self) -> &crate::Reg<tim12_ccmr1_input::TIM12_CCMR1_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<tim12_ccmr1_input::TIM12_CCMR1_INPUT_SPEC>)
        }
    }
}
#[doc = "TIM12_CR1 register accessor: an alias for `Reg<TIM12_CR1_SPEC>`"]
pub type TIM12_CR1 = crate::Reg<tim12_cr1::TIM12_CR1_SPEC>;
#[doc = "TIM12 control register 1"]
pub mod tim12_cr1;
#[doc = "TIM12_CR2 register accessor: an alias for `Reg<TIM12_CR2_SPEC>`"]
pub type TIM12_CR2 = crate::Reg<tim12_cr2::TIM12_CR2_SPEC>;
#[doc = "TIM12 control register 2"]
pub mod tim12_cr2;
#[doc = "TIM12_SMCR register accessor: an alias for `Reg<TIM12_SMCR_SPEC>`"]
pub type TIM12_SMCR = crate::Reg<tim12_smcr::TIM12_SMCR_SPEC>;
#[doc = "TIM12 slave mode control register"]
pub mod tim12_smcr;
#[doc = "TIM12_DIER register accessor: an alias for `Reg<TIM12_DIER_SPEC>`"]
pub type TIM12_DIER = crate::Reg<tim12_dier::TIM12_DIER_SPEC>;
#[doc = "TIM12 interrupt enable register"]
pub mod tim12_dier;
#[doc = "TIM12_SR register accessor: an alias for `Reg<TIM12_SR_SPEC>`"]
pub type TIM12_SR = crate::Reg<tim12_sr::TIM12_SR_SPEC>;
#[doc = "TIM12 status register"]
pub mod tim12_sr;
#[doc = "TIM12_EGR register accessor: an alias for `Reg<TIM12_EGR_SPEC>`"]
pub type TIM12_EGR = crate::Reg<tim12_egr::TIM12_EGR_SPEC>;
#[doc = "TIM12 event generation register"]
pub mod tim12_egr;
#[doc = "TIM12_CCMR1_input register accessor: an alias for `Reg<TIM12_CCMR1_INPUT_SPEC>`"]
pub type TIM12_CCMR1_INPUT = crate::Reg<tim12_ccmr1_input::TIM12_CCMR1_INPUT_SPEC>;
#[doc = "TIM12 capture/compare mode register 1"]
pub mod tim12_ccmr1_input;
#[doc = "TIM12_CCMR1_output register accessor: an alias for `Reg<TIM12_CCMR1_OUTPUT_SPEC>`"]
pub type TIM12_CCMR1_OUTPUT = crate::Reg<tim12_ccmr1_output::TIM12_CCMR1_OUTPUT_SPEC>;
#[doc = "TIM12 capture/compare mode register 1"]
pub mod tim12_ccmr1_output;
#[doc = "TIM12_CCER register accessor: an alias for `Reg<TIM12_CCER_SPEC>`"]
pub type TIM12_CCER = crate::Reg<tim12_ccer::TIM12_CCER_SPEC>;
#[doc = "TIM12 capture/compare enable register"]
pub mod tim12_ccer;
#[doc = "TIM12_CNT register accessor: an alias for `Reg<TIM12_CNT_SPEC>`"]
pub type TIM12_CNT = crate::Reg<tim12_cnt::TIM12_CNT_SPEC>;
#[doc = "TIM12 counter"]
pub mod tim12_cnt;
#[doc = "TIM12_PSC register accessor: an alias for `Reg<TIM12_PSC_SPEC>`"]
pub type TIM12_PSC = crate::Reg<tim12_psc::TIM12_PSC_SPEC>;
#[doc = "TIM12 prescaler"]
pub mod tim12_psc;
#[doc = "TIM12_ARR register accessor: an alias for `Reg<TIM12_ARR_SPEC>`"]
pub type TIM12_ARR = crate::Reg<tim12_arr::TIM12_ARR_SPEC>;
#[doc = "TIM12 auto-reload register"]
pub mod tim12_arr;
#[doc = "TIM12_CCR1 register accessor: an alias for `Reg<TIM12_CCR1_SPEC>`"]
pub type TIM12_CCR1 = crate::Reg<tim12_ccr1::TIM12_CCR1_SPEC>;
#[doc = "TIM12 capture/compare register 1"]
pub mod tim12_ccr1;
#[doc = "TIM12_CCR2 register accessor: an alias for `Reg<TIM12_CCR2_SPEC>`"]
pub type TIM12_CCR2 = crate::Reg<tim12_ccr2::TIM12_CCR2_SPEC>;
#[doc = "TIM12 capture/compare register 2"]
pub mod tim12_ccr2;
#[doc = "TIM12_TISEL register accessor: an alias for `Reg<TIM12_TISEL_SPEC>`"]
pub type TIM12_TISEL = crate::Reg<tim12_tisel::TIM12_TISEL_SPEC>;
#[doc = "TIM12 timer input selection register"]
pub mod tim12_tisel;
