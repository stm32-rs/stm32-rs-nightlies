#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM4 control register 1"]
    pub tim4_cr1: crate::Reg<tim4_cr1::TIM4_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM4 control register 2"]
    pub tim4_cr2: crate::Reg<tim4_cr2::TIM4_CR2_SPEC>,
    #[doc = "0x08 - TIM4 slave mode control register"]
    pub tim4_smcr: crate::Reg<tim4_smcr::TIM4_SMCR_SPEC>,
    #[doc = "0x0c - TIM4 DMA/interrupt enable register"]
    pub tim4_dier: crate::Reg<tim4_dier::TIM4_DIER_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM4 status register"]
    pub tim4_sr: crate::Reg<tim4_sr::TIM4_SR_SPEC>,
    #[doc = "0x14 - TIM4 event generation register"]
    pub tim4_egr: crate::Reg<tim4_egr::TIM4_EGR_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim4_ccmr1alternate4: crate::Reg<tim4_ccmr1alternate4::TIM4_CCMR1ALTERNATE4_SPEC>,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim4_ccmr2alternate20: crate::Reg<tim4_ccmr2alternate20::TIM4_CCMR2ALTERNATE20_SPEC>,
    #[doc = "0x20 - TIM4 capture/compare enable register"]
    pub tim4_ccer: crate::Reg<tim4_ccer::TIM4_CCER_SPEC>,
    #[doc = "0x24 - TIM4 counter"]
    pub tim4_cnt: crate::Reg<tim4_cnt::TIM4_CNT_SPEC>,
    #[doc = "0x28 - TIM4 prescaler"]
    pub tim4_psc: crate::Reg<tim4_psc::TIM4_PSC_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM4 auto-reload register"]
    pub tim4_arr: crate::Reg<tim4_arr::TIM4_ARR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - TIM4 repetition counter register"]
    pub tim4_rcr: crate::Reg<tim4_rcr::TIM4_RCR_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM4 capture/compare register 1"]
    pub tim4_ccr1: crate::Reg<tim4_ccr1::TIM4_CCR1_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - TIM4 capture/compare register 2"]
    pub tim4_ccr2: crate::Reg<tim4_ccr2::TIM4_CCR2_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - TIM4 capture/compare register 3"]
    pub tim4_ccr3: crate::Reg<tim4_ccr3::TIM4_CCR3_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - TIM4 capture/compare register 4"]
    pub tim4_ccr4: crate::Reg<tim4_ccr4::TIM4_CCR4_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim4_bdtr: crate::Reg<tim4_bdtr::TIM4_BDTR_SPEC>,
    #[doc = "0x48 - TIM4 DMA control register"]
    pub tim4_dcr: crate::Reg<tim4_dcr::TIM4_DCR_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - TIM4 DMA address for full transfer"]
    pub tim4_dmar: crate::Reg<tim4_dmar::TIM4_DMAR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim4_ccmr3: crate::Reg<tim4_ccmr3::TIM4_CCMR3_SPEC>,
    #[doc = "0x58 - TIM4 capture/compare register 5"]
    pub tim4_ccr5: crate::Reg<tim4_ccr5::TIM4_CCR5_SPEC>,
    #[doc = "0x5c - TIM4 capture/compare register 6"]
    pub tim4_ccr6: crate::Reg<tim4_ccr6::TIM4_CCR6_SPEC>,
}
#[doc = "TIM4_CR1 register accessor: an alias for `Reg<TIM4_CR1_SPEC>`"]
pub type TIM4_CR1 = crate::Reg<tim4_cr1::TIM4_CR1_SPEC>;
#[doc = "TIM4 control register 1"]
pub mod tim4_cr1;
#[doc = "TIM4_CR2 register accessor: an alias for `Reg<TIM4_CR2_SPEC>`"]
pub type TIM4_CR2 = crate::Reg<tim4_cr2::TIM4_CR2_SPEC>;
#[doc = "TIM4 control register 2"]
pub mod tim4_cr2;
#[doc = "TIM4_SMCR register accessor: an alias for `Reg<TIM4_SMCR_SPEC>`"]
pub type TIM4_SMCR = crate::Reg<tim4_smcr::TIM4_SMCR_SPEC>;
#[doc = "TIM4 slave mode control register"]
pub mod tim4_smcr;
#[doc = "TIM4_DIER register accessor: an alias for `Reg<TIM4_DIER_SPEC>`"]
pub type TIM4_DIER = crate::Reg<tim4_dier::TIM4_DIER_SPEC>;
#[doc = "TIM4 DMA/interrupt enable register"]
pub mod tim4_dier;
#[doc = "TIM4_SR register accessor: an alias for `Reg<TIM4_SR_SPEC>`"]
pub type TIM4_SR = crate::Reg<tim4_sr::TIM4_SR_SPEC>;
#[doc = "TIM4 status register"]
pub mod tim4_sr;
#[doc = "TIM4_EGR register accessor: an alias for `Reg<TIM4_EGR_SPEC>`"]
pub type TIM4_EGR = crate::Reg<tim4_egr::TIM4_EGR_SPEC>;
#[doc = "TIM4 event generation register"]
pub mod tim4_egr;
#[doc = "TIM4_CCMR1ALTERNATE4 register accessor: an alias for `Reg<TIM4_CCMR1ALTERNATE4_SPEC>`"]
pub type TIM4_CCMR1ALTERNATE4 = crate::Reg<tim4_ccmr1alternate4::TIM4_CCMR1ALTERNATE4_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim4_ccmr1alternate4;
#[doc = "TIM4_CCMR2ALTERNATE20 register accessor: an alias for `Reg<TIM4_CCMR2ALTERNATE20_SPEC>`"]
pub type TIM4_CCMR2ALTERNATE20 = crate::Reg<tim4_ccmr2alternate20::TIM4_CCMR2ALTERNATE20_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim4_ccmr2alternate20;
#[doc = "TIM4_CCER register accessor: an alias for `Reg<TIM4_CCER_SPEC>`"]
pub type TIM4_CCER = crate::Reg<tim4_ccer::TIM4_CCER_SPEC>;
#[doc = "TIM4 capture/compare enable register"]
pub mod tim4_ccer;
#[doc = "TIM4_CNT register accessor: an alias for `Reg<TIM4_CNT_SPEC>`"]
pub type TIM4_CNT = crate::Reg<tim4_cnt::TIM4_CNT_SPEC>;
#[doc = "TIM4 counter"]
pub mod tim4_cnt;
#[doc = "TIM4_PSC register accessor: an alias for `Reg<TIM4_PSC_SPEC>`"]
pub type TIM4_PSC = crate::Reg<tim4_psc::TIM4_PSC_SPEC>;
#[doc = "TIM4 prescaler"]
pub mod tim4_psc;
#[doc = "TIM4_ARR register accessor: an alias for `Reg<TIM4_ARR_SPEC>`"]
pub type TIM4_ARR = crate::Reg<tim4_arr::TIM4_ARR_SPEC>;
#[doc = "TIM4 auto-reload register"]
pub mod tim4_arr;
#[doc = "TIM4_RCR register accessor: an alias for `Reg<TIM4_RCR_SPEC>`"]
pub type TIM4_RCR = crate::Reg<tim4_rcr::TIM4_RCR_SPEC>;
#[doc = "TIM4 repetition counter register"]
pub mod tim4_rcr;
#[doc = "TIM4_CCR1 register accessor: an alias for `Reg<TIM4_CCR1_SPEC>`"]
pub type TIM4_CCR1 = crate::Reg<tim4_ccr1::TIM4_CCR1_SPEC>;
#[doc = "TIM4 capture/compare register 1"]
pub mod tim4_ccr1;
#[doc = "TIM4_CCR2 register accessor: an alias for `Reg<TIM4_CCR2_SPEC>`"]
pub type TIM4_CCR2 = crate::Reg<tim4_ccr2::TIM4_CCR2_SPEC>;
#[doc = "TIM4 capture/compare register 2"]
pub mod tim4_ccr2;
#[doc = "TIM4_CCR3 register accessor: an alias for `Reg<TIM4_CCR3_SPEC>`"]
pub type TIM4_CCR3 = crate::Reg<tim4_ccr3::TIM4_CCR3_SPEC>;
#[doc = "TIM4 capture/compare register 3"]
pub mod tim4_ccr3;
#[doc = "TIM4_CCR4 register accessor: an alias for `Reg<TIM4_CCR4_SPEC>`"]
pub type TIM4_CCR4 = crate::Reg<tim4_ccr4::TIM4_CCR4_SPEC>;
#[doc = "TIM4 capture/compare register 4"]
pub mod tim4_ccr4;
#[doc = "TIM4_BDTR register accessor: an alias for `Reg<TIM4_BDTR_SPEC>`"]
pub type TIM4_BDTR = crate::Reg<tim4_bdtr::TIM4_BDTR_SPEC>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim4_bdtr;
#[doc = "TIM4_DCR register accessor: an alias for `Reg<TIM4_DCR_SPEC>`"]
pub type TIM4_DCR = crate::Reg<tim4_dcr::TIM4_DCR_SPEC>;
#[doc = "TIM4 DMA control register"]
pub mod tim4_dcr;
#[doc = "TIM4_DMAR register accessor: an alias for `Reg<TIM4_DMAR_SPEC>`"]
pub type TIM4_DMAR = crate::Reg<tim4_dmar::TIM4_DMAR_SPEC>;
#[doc = "TIM4 DMA address for full transfer"]
pub mod tim4_dmar;
#[doc = "TIM4_CCMR3 register accessor: an alias for `Reg<TIM4_CCMR3_SPEC>`"]
pub type TIM4_CCMR3 = crate::Reg<tim4_ccmr3::TIM4_CCMR3_SPEC>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim4_ccmr3;
#[doc = "TIM4_CCR5 register accessor: an alias for `Reg<TIM4_CCR5_SPEC>`"]
pub type TIM4_CCR5 = crate::Reg<tim4_ccr5::TIM4_CCR5_SPEC>;
#[doc = "TIM4 capture/compare register 5"]
pub mod tim4_ccr5;
#[doc = "TIM4_CCR6 register accessor: an alias for `Reg<TIM4_CCR6_SPEC>`"]
pub type TIM4_CCR6 = crate::Reg<tim4_ccr6::TIM4_CCR6_SPEC>;
#[doc = "TIM4 capture/compare register 6"]
pub mod tim4_ccr6;
