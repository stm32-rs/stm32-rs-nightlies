#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM2 control register 1"]
    pub tim2_cr1: crate::Reg<tim2_cr1::TIM2_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM2 control register 2"]
    pub tim2_cr2: crate::Reg<tim2_cr2::TIM2_CR2_SPEC>,
    #[doc = "0x08 - TIM2 slave mode control register"]
    pub tim2_smcr: crate::Reg<tim2_smcr::TIM2_SMCR_SPEC>,
    #[doc = "0x0c - TIM2 DMA/interrupt enable register"]
    pub tim2_dier: crate::Reg<tim2_dier::TIM2_DIER_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM2 status register"]
    pub tim2_sr: crate::Reg<tim2_sr::TIM2_SR_SPEC>,
    #[doc = "0x14 - TIM2 event generation register"]
    pub tim2_egr: crate::Reg<tim2_egr::TIM2_EGR_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim2_ccmr1alternate2: crate::Reg<tim2_ccmr1alternate2::TIM2_CCMR1ALTERNATE2_SPEC>,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim2_ccmr2alternate18: crate::Reg<tim2_ccmr2alternate18::TIM2_CCMR2ALTERNATE18_SPEC>,
    #[doc = "0x20 - TIM2 capture/compare enable register"]
    pub tim2_ccer: crate::Reg<tim2_ccer::TIM2_CCER_SPEC>,
    #[doc = "0x24 - TIM2 counter"]
    pub tim2_cnt: crate::Reg<tim2_cnt::TIM2_CNT_SPEC>,
    #[doc = "0x28 - TIM2 prescaler"]
    pub tim2_psc: crate::Reg<tim2_psc::TIM2_PSC_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM2 auto-reload register"]
    pub tim2_arr: crate::Reg<tim2_arr::TIM2_ARR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - TIM2 repetition counter register"]
    pub tim2_rcr: crate::Reg<tim2_rcr::TIM2_RCR_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM2 capture/compare register 1"]
    pub tim2_ccr1: crate::Reg<tim2_ccr1::TIM2_CCR1_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - TIM2 capture/compare register 2"]
    pub tim2_ccr2: crate::Reg<tim2_ccr2::TIM2_CCR2_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - TIM2 capture/compare register 3"]
    pub tim2_ccr3: crate::Reg<tim2_ccr3::TIM2_CCR3_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - TIM2 capture/compare register 4"]
    pub tim2_ccr4: crate::Reg<tim2_ccr4::TIM2_CCR4_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim2_bdtr: crate::Reg<tim2_bdtr::TIM2_BDTR_SPEC>,
    #[doc = "0x48 - TIM2 DMA control register"]
    pub tim2_dcr: crate::Reg<tim2_dcr::TIM2_DCR_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - TIM2 DMA address for full transfer"]
    pub tim2_dmar: crate::Reg<tim2_dmar::TIM2_DMAR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim2_ccmr3: crate::Reg<tim2_ccmr3::TIM2_CCMR3_SPEC>,
    #[doc = "0x58 - TIM2 capture/compare register 5"]
    pub tim2_ccr5: crate::Reg<tim2_ccr5::TIM2_CCR5_SPEC>,
    #[doc = "0x5c - TIM2 capture/compare register 6"]
    pub tim2_ccr6: crate::Reg<tim2_ccr6::TIM2_CCR6_SPEC>,
}
#[doc = "TIM2_CR1 register accessor: an alias for `Reg<TIM2_CR1_SPEC>`"]
pub type TIM2_CR1 = crate::Reg<tim2_cr1::TIM2_CR1_SPEC>;
#[doc = "TIM2 control register 1"]
pub mod tim2_cr1;
#[doc = "TIM2_CR2 register accessor: an alias for `Reg<TIM2_CR2_SPEC>`"]
pub type TIM2_CR2 = crate::Reg<tim2_cr2::TIM2_CR2_SPEC>;
#[doc = "TIM2 control register 2"]
pub mod tim2_cr2;
#[doc = "TIM2_SMCR register accessor: an alias for `Reg<TIM2_SMCR_SPEC>`"]
pub type TIM2_SMCR = crate::Reg<tim2_smcr::TIM2_SMCR_SPEC>;
#[doc = "TIM2 slave mode control register"]
pub mod tim2_smcr;
#[doc = "TIM2_DIER register accessor: an alias for `Reg<TIM2_DIER_SPEC>`"]
pub type TIM2_DIER = crate::Reg<tim2_dier::TIM2_DIER_SPEC>;
#[doc = "TIM2 DMA/interrupt enable register"]
pub mod tim2_dier;
#[doc = "TIM2_SR register accessor: an alias for `Reg<TIM2_SR_SPEC>`"]
pub type TIM2_SR = crate::Reg<tim2_sr::TIM2_SR_SPEC>;
#[doc = "TIM2 status register"]
pub mod tim2_sr;
#[doc = "TIM2_EGR register accessor: an alias for `Reg<TIM2_EGR_SPEC>`"]
pub type TIM2_EGR = crate::Reg<tim2_egr::TIM2_EGR_SPEC>;
#[doc = "TIM2 event generation register"]
pub mod tim2_egr;
#[doc = "TIM2_CCMR1ALTERNATE2 register accessor: an alias for `Reg<TIM2_CCMR1ALTERNATE2_SPEC>`"]
pub type TIM2_CCMR1ALTERNATE2 = crate::Reg<tim2_ccmr1alternate2::TIM2_CCMR1ALTERNATE2_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim2_ccmr1alternate2;
#[doc = "TIM2_CCMR2ALTERNATE18 register accessor: an alias for `Reg<TIM2_CCMR2ALTERNATE18_SPEC>`"]
pub type TIM2_CCMR2ALTERNATE18 = crate::Reg<tim2_ccmr2alternate18::TIM2_CCMR2ALTERNATE18_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim2_ccmr2alternate18;
#[doc = "TIM2_CCER register accessor: an alias for `Reg<TIM2_CCER_SPEC>`"]
pub type TIM2_CCER = crate::Reg<tim2_ccer::TIM2_CCER_SPEC>;
#[doc = "TIM2 capture/compare enable register"]
pub mod tim2_ccer;
#[doc = "TIM2_CNT register accessor: an alias for `Reg<TIM2_CNT_SPEC>`"]
pub type TIM2_CNT = crate::Reg<tim2_cnt::TIM2_CNT_SPEC>;
#[doc = "TIM2 counter"]
pub mod tim2_cnt;
#[doc = "TIM2_PSC register accessor: an alias for `Reg<TIM2_PSC_SPEC>`"]
pub type TIM2_PSC = crate::Reg<tim2_psc::TIM2_PSC_SPEC>;
#[doc = "TIM2 prescaler"]
pub mod tim2_psc;
#[doc = "TIM2_ARR register accessor: an alias for `Reg<TIM2_ARR_SPEC>`"]
pub type TIM2_ARR = crate::Reg<tim2_arr::TIM2_ARR_SPEC>;
#[doc = "TIM2 auto-reload register"]
pub mod tim2_arr;
#[doc = "TIM2_RCR register accessor: an alias for `Reg<TIM2_RCR_SPEC>`"]
pub type TIM2_RCR = crate::Reg<tim2_rcr::TIM2_RCR_SPEC>;
#[doc = "TIM2 repetition counter register"]
pub mod tim2_rcr;
#[doc = "TIM2_CCR1 register accessor: an alias for `Reg<TIM2_CCR1_SPEC>`"]
pub type TIM2_CCR1 = crate::Reg<tim2_ccr1::TIM2_CCR1_SPEC>;
#[doc = "TIM2 capture/compare register 1"]
pub mod tim2_ccr1;
#[doc = "TIM2_CCR2 register accessor: an alias for `Reg<TIM2_CCR2_SPEC>`"]
pub type TIM2_CCR2 = crate::Reg<tim2_ccr2::TIM2_CCR2_SPEC>;
#[doc = "TIM2 capture/compare register 2"]
pub mod tim2_ccr2;
#[doc = "TIM2_CCR3 register accessor: an alias for `Reg<TIM2_CCR3_SPEC>`"]
pub type TIM2_CCR3 = crate::Reg<tim2_ccr3::TIM2_CCR3_SPEC>;
#[doc = "TIM2 capture/compare register 3"]
pub mod tim2_ccr3;
#[doc = "TIM2_CCR4 register accessor: an alias for `Reg<TIM2_CCR4_SPEC>`"]
pub type TIM2_CCR4 = crate::Reg<tim2_ccr4::TIM2_CCR4_SPEC>;
#[doc = "TIM2 capture/compare register 4"]
pub mod tim2_ccr4;
#[doc = "TIM2_BDTR register accessor: an alias for `Reg<TIM2_BDTR_SPEC>`"]
pub type TIM2_BDTR = crate::Reg<tim2_bdtr::TIM2_BDTR_SPEC>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim2_bdtr;
#[doc = "TIM2_DCR register accessor: an alias for `Reg<TIM2_DCR_SPEC>`"]
pub type TIM2_DCR = crate::Reg<tim2_dcr::TIM2_DCR_SPEC>;
#[doc = "TIM2 DMA control register"]
pub mod tim2_dcr;
#[doc = "TIM2_DMAR register accessor: an alias for `Reg<TIM2_DMAR_SPEC>`"]
pub type TIM2_DMAR = crate::Reg<tim2_dmar::TIM2_DMAR_SPEC>;
#[doc = "TIM2 DMA address for full transfer"]
pub mod tim2_dmar;
#[doc = "TIM2_CCMR3 register accessor: an alias for `Reg<TIM2_CCMR3_SPEC>`"]
pub type TIM2_CCMR3 = crate::Reg<tim2_ccmr3::TIM2_CCMR3_SPEC>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim2_ccmr3;
#[doc = "TIM2_CCR5 register accessor: an alias for `Reg<TIM2_CCR5_SPEC>`"]
pub type TIM2_CCR5 = crate::Reg<tim2_ccr5::TIM2_CCR5_SPEC>;
#[doc = "TIM2 capture/compare register 5"]
pub mod tim2_ccr5;
#[doc = "TIM2_CCR6 register accessor: an alias for `Reg<TIM2_CCR6_SPEC>`"]
pub type TIM2_CCR6 = crate::Reg<tim2_ccr6::TIM2_CCR6_SPEC>;
#[doc = "TIM2 capture/compare register 6"]
pub mod tim2_ccr6;
