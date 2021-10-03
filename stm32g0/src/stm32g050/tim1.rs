#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x08 - slave mode control register"]
    pub smcr: crate::Reg<smcr::SMCR_SPEC>,
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dier: crate::Reg<dier::DIER_SPEC>,
    #[doc = "0x10 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - event generation register"]
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    #[doc = "0x20 - capture/compare enable register"]
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    #[doc = "0x24 - counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - prescaler"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x2c - auto-reload register"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
    #[doc = "0x30 - repetition counter register"]
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
    #[doc = "0x34 - capture/compare register 1"]
    pub ccr1: crate::Reg<ccr1::CCR1_SPEC>,
    #[doc = "0x38 - capture/compare register 2"]
    pub ccr2: crate::Reg<ccr2::CCR2_SPEC>,
    #[doc = "0x3c - capture/compare register 3"]
    pub ccr3: crate::Reg<ccr3::CCR3_SPEC>,
    #[doc = "0x40 - capture/compare register 4"]
    pub ccr4: crate::Reg<ccr4::CCR4_SPEC>,
    #[doc = "0x44 - break and dead-time register"]
    pub bdtr: crate::Reg<bdtr::BDTR_SPEC>,
    #[doc = "0x48 - DMA control register"]
    pub dcr: crate::Reg<dcr::DCR_SPEC>,
    #[doc = "0x4c - DMA address for full transfer"]
    pub dmar: crate::Reg<dmar::DMAR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - capture/compare mode register 2 (output mode)"]
    pub ccmr3_output: crate::Reg<ccmr3_output::CCMR3_OUTPUT_SPEC>,
    #[doc = "0x58 - capture/compare register 4"]
    pub ccr5: crate::Reg<ccr5::CCR5_SPEC>,
    #[doc = "0x5c - capture/compare register 6"]
    pub ccr6: crate::Reg<ccr6::CCR6_SPEC>,
    #[doc = "0x60 - TIM1 alternate function option register 1"]
    pub af1: crate::Reg<af1::AF1_SPEC>,
    #[doc = "0x64 - TIM1 alternate function option register 2"]
    pub af2: crate::Reg<af2::AF2_SPEC>,
    #[doc = "0x68 - TIM1 timer input selection register"]
    pub tisel: crate::Reg<tisel::TISEL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>)
        }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>)
        }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_input(&self) -> &crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>)
        }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_output(&self) -> &crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>)
        }
    }
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMCR register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "slave mode control register"]
pub mod smcr;
#[doc = "DIER register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "EGR register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "event generation register"]
pub mod egr;
#[doc = "CCMR1_Output register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`"]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`"]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_input;
#[doc = "CCMR2_Output register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`"]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_output;
#[doc = "CCMR2_Input register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`"]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_input;
#[doc = "CCER register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "RCR register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "repetition counter register"]
pub mod rcr;
#[doc = "CCR1 register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "capture/compare register 1"]
pub mod ccr1;
#[doc = "CCR2 register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "capture/compare register 2"]
pub mod ccr2;
#[doc = "CCR3 register accessor: an alias for `Reg<CCR3_SPEC>`"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "capture/compare register 3"]
pub mod ccr3;
#[doc = "CCR4 register accessor: an alias for `Reg<CCR4_SPEC>`"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "capture/compare register 4"]
pub mod ccr4;
#[doc = "BDTR register accessor: an alias for `Reg<BDTR_SPEC>`"]
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
#[doc = "break and dead-time register"]
pub mod bdtr;
#[doc = "DCR register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "DMA control register"]
pub mod dcr;
#[doc = "DMAR register accessor: an alias for `Reg<DMAR_SPEC>`"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "DMA address for full transfer"]
pub mod dmar;
#[doc = "CCMR3_Output register accessor: an alias for `Reg<CCMR3_OUTPUT_SPEC>`"]
pub type CCMR3_OUTPUT = crate::Reg<ccmr3_output::CCMR3_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr3_output;
#[doc = "CCR5 register accessor: an alias for `Reg<CCR5_SPEC>`"]
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
#[doc = "capture/compare register 4"]
pub mod ccr5;
#[doc = "CCR6 register accessor: an alias for `Reg<CCR6_SPEC>`"]
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
#[doc = "capture/compare register 6"]
pub mod ccr6;
#[doc = "AF1 register accessor: an alias for `Reg<AF1_SPEC>`"]
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
#[doc = "TIM1 alternate function option register 1"]
pub mod af1;
#[doc = "AF2 register accessor: an alias for `Reg<AF2_SPEC>`"]
pub type AF2 = crate::Reg<af2::AF2_SPEC>;
#[doc = "TIM1 alternate function option register 2"]
pub mod af2;
#[doc = "TISEL register accessor: an alias for `Reg<TISEL_SPEC>`"]
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
#[doc = "TIM1 timer input selection register"]
pub mod tisel;
