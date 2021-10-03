#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM1 control register 1"]
    pub tim1_cr1: crate::Reg<tim1_cr1::TIM1_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM1 control register 2"]
    pub tim1_cr2: crate::Reg<tim1_cr2::TIM1_CR2_SPEC>,
    #[doc = "0x08 - TIM1 slave mode control register"]
    pub tim1_smcr: crate::Reg<tim1_smcr::TIM1_SMCR_SPEC>,
    #[doc = "0x0c - TIM1 DMA/interrupt enable register"]
    pub tim1_dier: crate::Reg<tim1_dier::TIM1_DIER_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM1 status register"]
    pub tim1_sr: crate::Reg<tim1_sr::TIM1_SR_SPEC>,
    #[doc = "0x14 - TIM1 event generation register"]
    pub tim1_egr: crate::Reg<tim1_egr::TIM1_EGR_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim1_ccmr1alternate1: crate::Reg<tim1_ccmr1alternate1::TIM1_CCMR1ALTERNATE1_SPEC>,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim1_ccmr2alternate17: crate::Reg<tim1_ccmr2alternate17::TIM1_CCMR2ALTERNATE17_SPEC>,
    #[doc = "0x20 - TIM1 capture/compare enable register"]
    pub tim1_ccer: crate::Reg<tim1_ccer::TIM1_CCER_SPEC>,
    #[doc = "0x24 - TIM1 counter"]
    pub tim1_cnt: crate::Reg<tim1_cnt::TIM1_CNT_SPEC>,
    #[doc = "0x28 - TIM1 prescaler"]
    pub tim1_psc: crate::Reg<tim1_psc::TIM1_PSC_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM1 auto-reload register"]
    pub tim1_arr: crate::Reg<tim1_arr::TIM1_ARR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - TIM1 repetition counter register"]
    pub tim1_rcr: crate::Reg<tim1_rcr::TIM1_RCR_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM1 capture/compare register 1"]
    pub tim1_ccr1: crate::Reg<tim1_ccr1::TIM1_CCR1_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - TIM1 capture/compare register 2"]
    pub tim1_ccr2: crate::Reg<tim1_ccr2::TIM1_CCR2_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - TIM1 capture/compare register 3"]
    pub tim1_ccr3: crate::Reg<tim1_ccr3::TIM1_CCR3_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - TIM1 capture/compare register 4"]
    pub tim1_ccr4: crate::Reg<tim1_ccr4::TIM1_CCR4_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim1_bdtr: crate::Reg<tim1_bdtr::TIM1_BDTR_SPEC>,
    #[doc = "0x48 - TIM1 DMA control register"]
    pub tim1_dcr: crate::Reg<tim1_dcr::TIM1_DCR_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - TIM1 DMA address for full transfer"]
    pub tim1_dmar: crate::Reg<tim1_dmar::TIM1_DMAR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim1_ccmr3: crate::Reg<tim1_ccmr3::TIM1_CCMR3_SPEC>,
    #[doc = "0x58 - TIM1 capture/compare register 5"]
    pub tim1_ccr5: crate::Reg<tim1_ccr5::TIM1_CCR5_SPEC>,
    #[doc = "0x5c - TIM1 capture/compare register 6"]
    pub tim1_ccr6: crate::Reg<tim1_ccr6::TIM1_CCR6_SPEC>,
    _reserved23: [u8; 0x02],
    #[doc = "0x60 - TIM1 alternate function option register 1"]
    pub tim1_af1: crate::Reg<tim1_af1::TIM1_AF1_SPEC>,
    #[doc = "0x64 - TIM1 Alternate function register 2"]
    pub tim1_af2: crate::Reg<tim1_af2::TIM1_AF2_SPEC>,
    #[doc = "0x68 - TIM1 timer input selection register"]
    pub tim1_tisel: crate::Reg<tim1_tisel::TIM1_TISEL_SPEC>,
}
#[doc = "TIM1_CR1 register accessor: an alias for `Reg<TIM1_CR1_SPEC>`"]
pub type TIM1_CR1 = crate::Reg<tim1_cr1::TIM1_CR1_SPEC>;
#[doc = "TIM1 control register 1"]
pub mod tim1_cr1;
#[doc = "TIM1_CR2 register accessor: an alias for `Reg<TIM1_CR2_SPEC>`"]
pub type TIM1_CR2 = crate::Reg<tim1_cr2::TIM1_CR2_SPEC>;
#[doc = "TIM1 control register 2"]
pub mod tim1_cr2;
#[doc = "TIM1_SMCR register accessor: an alias for `Reg<TIM1_SMCR_SPEC>`"]
pub type TIM1_SMCR = crate::Reg<tim1_smcr::TIM1_SMCR_SPEC>;
#[doc = "TIM1 slave mode control register"]
pub mod tim1_smcr;
#[doc = "TIM1_DIER register accessor: an alias for `Reg<TIM1_DIER_SPEC>`"]
pub type TIM1_DIER = crate::Reg<tim1_dier::TIM1_DIER_SPEC>;
#[doc = "TIM1 DMA/interrupt enable register"]
pub mod tim1_dier;
#[doc = "TIM1_SR register accessor: an alias for `Reg<TIM1_SR_SPEC>`"]
pub type TIM1_SR = crate::Reg<tim1_sr::TIM1_SR_SPEC>;
#[doc = "TIM1 status register"]
pub mod tim1_sr;
#[doc = "TIM1_EGR register accessor: an alias for `Reg<TIM1_EGR_SPEC>`"]
pub type TIM1_EGR = crate::Reg<tim1_egr::TIM1_EGR_SPEC>;
#[doc = "TIM1 event generation register"]
pub mod tim1_egr;
#[doc = "TIM1_CCMR1ALTERNATE1 register accessor: an alias for `Reg<TIM1_CCMR1ALTERNATE1_SPEC>`"]
pub type TIM1_CCMR1ALTERNATE1 = crate::Reg<tim1_ccmr1alternate1::TIM1_CCMR1ALTERNATE1_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim1_ccmr1alternate1;
#[doc = "TIM1_CCMR2ALTERNATE17 register accessor: an alias for `Reg<TIM1_CCMR2ALTERNATE17_SPEC>`"]
pub type TIM1_CCMR2ALTERNATE17 = crate::Reg<tim1_ccmr2alternate17::TIM1_CCMR2ALTERNATE17_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim1_ccmr2alternate17;
#[doc = "TIM1_CCER register accessor: an alias for `Reg<TIM1_CCER_SPEC>`"]
pub type TIM1_CCER = crate::Reg<tim1_ccer::TIM1_CCER_SPEC>;
#[doc = "TIM1 capture/compare enable register"]
pub mod tim1_ccer;
#[doc = "TIM1_CNT register accessor: an alias for `Reg<TIM1_CNT_SPEC>`"]
pub type TIM1_CNT = crate::Reg<tim1_cnt::TIM1_CNT_SPEC>;
#[doc = "TIM1 counter"]
pub mod tim1_cnt;
#[doc = "TIM1_PSC register accessor: an alias for `Reg<TIM1_PSC_SPEC>`"]
pub type TIM1_PSC = crate::Reg<tim1_psc::TIM1_PSC_SPEC>;
#[doc = "TIM1 prescaler"]
pub mod tim1_psc;
#[doc = "TIM1_ARR register accessor: an alias for `Reg<TIM1_ARR_SPEC>`"]
pub type TIM1_ARR = crate::Reg<tim1_arr::TIM1_ARR_SPEC>;
#[doc = "TIM1 auto-reload register"]
pub mod tim1_arr;
#[doc = "TIM1_RCR register accessor: an alias for `Reg<TIM1_RCR_SPEC>`"]
pub type TIM1_RCR = crate::Reg<tim1_rcr::TIM1_RCR_SPEC>;
#[doc = "TIM1 repetition counter register"]
pub mod tim1_rcr;
#[doc = "TIM1_CCR1 register accessor: an alias for `Reg<TIM1_CCR1_SPEC>`"]
pub type TIM1_CCR1 = crate::Reg<tim1_ccr1::TIM1_CCR1_SPEC>;
#[doc = "TIM1 capture/compare register 1"]
pub mod tim1_ccr1;
#[doc = "TIM1_CCR2 register accessor: an alias for `Reg<TIM1_CCR2_SPEC>`"]
pub type TIM1_CCR2 = crate::Reg<tim1_ccr2::TIM1_CCR2_SPEC>;
#[doc = "TIM1 capture/compare register 2"]
pub mod tim1_ccr2;
#[doc = "TIM1_CCR3 register accessor: an alias for `Reg<TIM1_CCR3_SPEC>`"]
pub type TIM1_CCR3 = crate::Reg<tim1_ccr3::TIM1_CCR3_SPEC>;
#[doc = "TIM1 capture/compare register 3"]
pub mod tim1_ccr3;
#[doc = "TIM1_CCR4 register accessor: an alias for `Reg<TIM1_CCR4_SPEC>`"]
pub type TIM1_CCR4 = crate::Reg<tim1_ccr4::TIM1_CCR4_SPEC>;
#[doc = "TIM1 capture/compare register 4"]
pub mod tim1_ccr4;
#[doc = "TIM1_BDTR register accessor: an alias for `Reg<TIM1_BDTR_SPEC>`"]
pub type TIM1_BDTR = crate::Reg<tim1_bdtr::TIM1_BDTR_SPEC>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim1_bdtr;
#[doc = "TIM1_DCR register accessor: an alias for `Reg<TIM1_DCR_SPEC>`"]
pub type TIM1_DCR = crate::Reg<tim1_dcr::TIM1_DCR_SPEC>;
#[doc = "TIM1 DMA control register"]
pub mod tim1_dcr;
#[doc = "TIM1_DMAR register accessor: an alias for `Reg<TIM1_DMAR_SPEC>`"]
pub type TIM1_DMAR = crate::Reg<tim1_dmar::TIM1_DMAR_SPEC>;
#[doc = "TIM1 DMA address for full transfer"]
pub mod tim1_dmar;
#[doc = "TIM1_CCMR3 register accessor: an alias for `Reg<TIM1_CCMR3_SPEC>`"]
pub type TIM1_CCMR3 = crate::Reg<tim1_ccmr3::TIM1_CCMR3_SPEC>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim1_ccmr3;
#[doc = "TIM1_CCR5 register accessor: an alias for `Reg<TIM1_CCR5_SPEC>`"]
pub type TIM1_CCR5 = crate::Reg<tim1_ccr5::TIM1_CCR5_SPEC>;
#[doc = "TIM1 capture/compare register 5"]
pub mod tim1_ccr5;
#[doc = "TIM1_CCR6 register accessor: an alias for `Reg<TIM1_CCR6_SPEC>`"]
pub type TIM1_CCR6 = crate::Reg<tim1_ccr6::TIM1_CCR6_SPEC>;
#[doc = "TIM1 capture/compare register 6"]
pub mod tim1_ccr6;
#[doc = "TIM1_AF1 register accessor: an alias for `Reg<TIM1_AF1_SPEC>`"]
pub type TIM1_AF1 = crate::Reg<tim1_af1::TIM1_AF1_SPEC>;
#[doc = "TIM1 alternate function option register 1"]
pub mod tim1_af1;
#[doc = "TIM1_AF2 register accessor: an alias for `Reg<TIM1_AF2_SPEC>`"]
pub type TIM1_AF2 = crate::Reg<tim1_af2::TIM1_AF2_SPEC>;
#[doc = "TIM1 Alternate function register 2"]
pub mod tim1_af2;
#[doc = "TIM1_TISEL register accessor: an alias for `Reg<TIM1_TISEL_SPEC>`"]
pub type TIM1_TISEL = crate::Reg<tim1_tisel::TIM1_TISEL_SPEC>;
#[doc = "TIM1 timer input selection register"]
pub mod tim1_tisel;
