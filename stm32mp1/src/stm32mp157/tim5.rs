#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM5 control register 1"]
    pub tim5_cr1: crate::Reg<tim5_cr1::TIM5_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM5 control register 2"]
    pub tim5_cr2: crate::Reg<tim5_cr2::TIM5_CR2_SPEC>,
    #[doc = "0x08 - TIM5 slave mode control register"]
    pub tim5_smcr: crate::Reg<tim5_smcr::TIM5_SMCR_SPEC>,
    #[doc = "0x0c - TIM5 DMA/interrupt enable register"]
    pub tim5_dier: crate::Reg<tim5_dier::TIM5_DIER_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM5 status register"]
    pub tim5_sr: crate::Reg<tim5_sr::TIM5_SR_SPEC>,
    #[doc = "0x14 - TIM5 event generation register"]
    pub tim5_egr: crate::Reg<tim5_egr::TIM5_EGR_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim5_ccmr1alternate5: crate::Reg<tim5_ccmr1alternate5::TIM5_CCMR1ALTERNATE5_SPEC>,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim5_ccmr2alternate21: crate::Reg<tim5_ccmr2alternate21::TIM5_CCMR2ALTERNATE21_SPEC>,
    #[doc = "0x20 - TIM5 capture/compare enable register"]
    pub tim5_ccer: crate::Reg<tim5_ccer::TIM5_CCER_SPEC>,
    #[doc = "0x24 - TIM5 counter"]
    pub tim5_cnt: crate::Reg<tim5_cnt::TIM5_CNT_SPEC>,
    #[doc = "0x28 - TIM5 prescaler"]
    pub tim5_psc: crate::Reg<tim5_psc::TIM5_PSC_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM5 auto-reload register"]
    pub tim5_arr: crate::Reg<tim5_arr::TIM5_ARR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - TIM5 repetition counter register"]
    pub tim5_rcr: crate::Reg<tim5_rcr::TIM5_RCR_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM5 capture/compare register 1"]
    pub tim5_ccr1: crate::Reg<tim5_ccr1::TIM5_CCR1_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - TIM5 capture/compare register 2"]
    pub tim5_ccr2: crate::Reg<tim5_ccr2::TIM5_CCR2_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - TIM5 capture/compare register 3"]
    pub tim5_ccr3: crate::Reg<tim5_ccr3::TIM5_CCR3_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - TIM5 capture/compare register 4"]
    pub tim5_ccr4: crate::Reg<tim5_ccr4::TIM5_CCR4_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim5_bdtr: crate::Reg<tim5_bdtr::TIM5_BDTR_SPEC>,
    #[doc = "0x48 - TIM5 DMA control register"]
    pub tim5_dcr: crate::Reg<tim5_dcr::TIM5_DCR_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - TIM5 DMA address for full transfer"]
    pub tim5_dmar: crate::Reg<tim5_dmar::TIM5_DMAR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim5_ccmr3: crate::Reg<tim5_ccmr3::TIM5_CCMR3_SPEC>,
    #[doc = "0x58 - TIM5 capture/compare register 5"]
    pub tim5_ccr5: crate::Reg<tim5_ccr5::TIM5_CCR5_SPEC>,
    #[doc = "0x5c - TIM5 capture/compare register 6"]
    pub tim5_ccr6: crate::Reg<tim5_ccr6::TIM5_CCR6_SPEC>,
}
#[doc = "TIM5_CR1 register accessor: an alias for `Reg<TIM5_CR1_SPEC>`"]
pub type TIM5_CR1 = crate::Reg<tim5_cr1::TIM5_CR1_SPEC>;
#[doc = "TIM5 control register 1"]
pub mod tim5_cr1;
#[doc = "TIM5_CR2 register accessor: an alias for `Reg<TIM5_CR2_SPEC>`"]
pub type TIM5_CR2 = crate::Reg<tim5_cr2::TIM5_CR2_SPEC>;
#[doc = "TIM5 control register 2"]
pub mod tim5_cr2;
#[doc = "TIM5_SMCR register accessor: an alias for `Reg<TIM5_SMCR_SPEC>`"]
pub type TIM5_SMCR = crate::Reg<tim5_smcr::TIM5_SMCR_SPEC>;
#[doc = "TIM5 slave mode control register"]
pub mod tim5_smcr;
#[doc = "TIM5_DIER register accessor: an alias for `Reg<TIM5_DIER_SPEC>`"]
pub type TIM5_DIER = crate::Reg<tim5_dier::TIM5_DIER_SPEC>;
#[doc = "TIM5 DMA/interrupt enable register"]
pub mod tim5_dier;
#[doc = "TIM5_SR register accessor: an alias for `Reg<TIM5_SR_SPEC>`"]
pub type TIM5_SR = crate::Reg<tim5_sr::TIM5_SR_SPEC>;
#[doc = "TIM5 status register"]
pub mod tim5_sr;
#[doc = "TIM5_EGR register accessor: an alias for `Reg<TIM5_EGR_SPEC>`"]
pub type TIM5_EGR = crate::Reg<tim5_egr::TIM5_EGR_SPEC>;
#[doc = "TIM5 event generation register"]
pub mod tim5_egr;
#[doc = "TIM5_CCMR1ALTERNATE5 register accessor: an alias for `Reg<TIM5_CCMR1ALTERNATE5_SPEC>`"]
pub type TIM5_CCMR1ALTERNATE5 = crate::Reg<tim5_ccmr1alternate5::TIM5_CCMR1ALTERNATE5_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim5_ccmr1alternate5;
#[doc = "TIM5_CCMR2ALTERNATE21 register accessor: an alias for `Reg<TIM5_CCMR2ALTERNATE21_SPEC>`"]
pub type TIM5_CCMR2ALTERNATE21 = crate::Reg<tim5_ccmr2alternate21::TIM5_CCMR2ALTERNATE21_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim5_ccmr2alternate21;
#[doc = "TIM5_CCER register accessor: an alias for `Reg<TIM5_CCER_SPEC>`"]
pub type TIM5_CCER = crate::Reg<tim5_ccer::TIM5_CCER_SPEC>;
#[doc = "TIM5 capture/compare enable register"]
pub mod tim5_ccer;
#[doc = "TIM5_CNT register accessor: an alias for `Reg<TIM5_CNT_SPEC>`"]
pub type TIM5_CNT = crate::Reg<tim5_cnt::TIM5_CNT_SPEC>;
#[doc = "TIM5 counter"]
pub mod tim5_cnt;
#[doc = "TIM5_PSC register accessor: an alias for `Reg<TIM5_PSC_SPEC>`"]
pub type TIM5_PSC = crate::Reg<tim5_psc::TIM5_PSC_SPEC>;
#[doc = "TIM5 prescaler"]
pub mod tim5_psc;
#[doc = "TIM5_ARR register accessor: an alias for `Reg<TIM5_ARR_SPEC>`"]
pub type TIM5_ARR = crate::Reg<tim5_arr::TIM5_ARR_SPEC>;
#[doc = "TIM5 auto-reload register"]
pub mod tim5_arr;
#[doc = "TIM5_RCR register accessor: an alias for `Reg<TIM5_RCR_SPEC>`"]
pub type TIM5_RCR = crate::Reg<tim5_rcr::TIM5_RCR_SPEC>;
#[doc = "TIM5 repetition counter register"]
pub mod tim5_rcr;
#[doc = "TIM5_CCR1 register accessor: an alias for `Reg<TIM5_CCR1_SPEC>`"]
pub type TIM5_CCR1 = crate::Reg<tim5_ccr1::TIM5_CCR1_SPEC>;
#[doc = "TIM5 capture/compare register 1"]
pub mod tim5_ccr1;
#[doc = "TIM5_CCR2 register accessor: an alias for `Reg<TIM5_CCR2_SPEC>`"]
pub type TIM5_CCR2 = crate::Reg<tim5_ccr2::TIM5_CCR2_SPEC>;
#[doc = "TIM5 capture/compare register 2"]
pub mod tim5_ccr2;
#[doc = "TIM5_CCR3 register accessor: an alias for `Reg<TIM5_CCR3_SPEC>`"]
pub type TIM5_CCR3 = crate::Reg<tim5_ccr3::TIM5_CCR3_SPEC>;
#[doc = "TIM5 capture/compare register 3"]
pub mod tim5_ccr3;
#[doc = "TIM5_CCR4 register accessor: an alias for `Reg<TIM5_CCR4_SPEC>`"]
pub type TIM5_CCR4 = crate::Reg<tim5_ccr4::TIM5_CCR4_SPEC>;
#[doc = "TIM5 capture/compare register 4"]
pub mod tim5_ccr4;
#[doc = "TIM5_BDTR register accessor: an alias for `Reg<TIM5_BDTR_SPEC>`"]
pub type TIM5_BDTR = crate::Reg<tim5_bdtr::TIM5_BDTR_SPEC>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim5_bdtr;
#[doc = "TIM5_DCR register accessor: an alias for `Reg<TIM5_DCR_SPEC>`"]
pub type TIM5_DCR = crate::Reg<tim5_dcr::TIM5_DCR_SPEC>;
#[doc = "TIM5 DMA control register"]
pub mod tim5_dcr;
#[doc = "TIM5_DMAR register accessor: an alias for `Reg<TIM5_DMAR_SPEC>`"]
pub type TIM5_DMAR = crate::Reg<tim5_dmar::TIM5_DMAR_SPEC>;
#[doc = "TIM5 DMA address for full transfer"]
pub mod tim5_dmar;
#[doc = "TIM5_CCMR3 register accessor: an alias for `Reg<TIM5_CCMR3_SPEC>`"]
pub type TIM5_CCMR3 = crate::Reg<tim5_ccmr3::TIM5_CCMR3_SPEC>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim5_ccmr3;
#[doc = "TIM5_CCR5 register accessor: an alias for `Reg<TIM5_CCR5_SPEC>`"]
pub type TIM5_CCR5 = crate::Reg<tim5_ccr5::TIM5_CCR5_SPEC>;
#[doc = "TIM5 capture/compare register 5"]
pub mod tim5_ccr5;
#[doc = "TIM5_CCR6 register accessor: an alias for `Reg<TIM5_CCR6_SPEC>`"]
pub type TIM5_CCR6 = crate::Reg<tim5_ccr6::TIM5_CCR6_SPEC>;
#[doc = "TIM5 capture/compare register 6"]
pub mod tim5_ccr6;
