#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM16/TIM17 control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - TIM16/TIM17 control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - TIM16/TIM17 DMA/interrupt enable register"]
    pub dier: crate::Reg<dier::DIER_SPEC>,
    #[doc = "0x10 - TIM16/TIM17 status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - TIM16/TIM17 event generation register"]
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved_5_ccmr1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - TIM16/TIM17 capture/compare enable register"]
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    #[doc = "0x24 - TIM16/TIM17 counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - TIM16/TIM17 prescaler"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x2c - TIM16/TIM17 auto-reload register"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
    #[doc = "0x30 - TIM16/TIM17 repetition counter register"]
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
    #[doc = "0x34 - TIM16/TIM17 capture/compare register 1"]
    pub ccr1: crate::Reg<ccr1::CCR1_SPEC>,
    _reserved12: [u8; 0x0c],
    #[doc = "0x44 - TIM16/TIM17 break and dead-time register"]
    pub bdtr: crate::Reg<bdtr::BDTR_SPEC>,
    #[doc = "0x48 - TIM16/TIM17 DMA control register"]
    pub dcr: crate::Reg<dcr::DCR_SPEC>,
    #[doc = "0x4c - TIM16/TIM17 DMA address for full transfer"]
    pub dmar: crate::Reg<dmar::DMAR_SPEC>,
    #[doc = "0x50 - TIM16 option register 1"]
    pub or1: crate::Reg<or1::OR1_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x60 - TIM16 alternate function register 1"]
    pub af1: crate::Reg<af1::AF1_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x68 - TIM16 input selection register"]
    pub tisel: crate::Reg<tisel::TISEL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM16/TIM17 capture/compare mode register 1"]
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>)
        }
    }
    #[doc = "0x18 - TIM16/TIM17 capture/compare mode register 1"]
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>)
        }
    }
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM16/TIM17 control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "TIM16/TIM17 control register 2"]
pub mod cr2;
#[doc = "DIER register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "TIM16/TIM17 DMA/interrupt enable register"]
pub mod dier;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TIM16/TIM17 status register"]
pub mod sr;
#[doc = "EGR register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM16/TIM17 event generation register"]
pub mod egr;
#[doc = "CCMR1_Output register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`"]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "TIM16/TIM17 capture/compare mode register 1"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`"]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "TIM16/TIM17 capture/compare mode register 1"]
pub mod ccmr1_input;
#[doc = "CCER register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "TIM16/TIM17 capture/compare enable register"]
pub mod ccer;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "TIM16/TIM17 counter"]
pub mod cnt;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "TIM16/TIM17 prescaler"]
pub mod psc;
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "TIM16/TIM17 auto-reload register"]
pub mod arr;
#[doc = "RCR register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "TIM16/TIM17 repetition counter register"]
pub mod rcr;
#[doc = "CCR1 register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "TIM16/TIM17 capture/compare register 1"]
pub mod ccr1;
#[doc = "BDTR register accessor: an alias for `Reg<BDTR_SPEC>`"]
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
#[doc = "TIM16/TIM17 break and dead-time register"]
pub mod bdtr;
#[doc = "DCR register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "TIM16/TIM17 DMA control register"]
pub mod dcr;
#[doc = "DMAR register accessor: an alias for `Reg<DMAR_SPEC>`"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "TIM16/TIM17 DMA address for full transfer"]
pub mod dmar;
#[doc = "OR1 register accessor: an alias for `Reg<OR1_SPEC>`"]
pub type OR1 = crate::Reg<or1::OR1_SPEC>;
#[doc = "TIM16 option register 1"]
pub mod or1;
#[doc = "AF1 register accessor: an alias for `Reg<AF1_SPEC>`"]
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
#[doc = "TIM16 alternate function register 1"]
pub mod af1;
#[doc = "TISEL register accessor: an alias for `Reg<TISEL_SPEC>`"]
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
#[doc = "TIM16 input selection register"]
pub mod tisel;
