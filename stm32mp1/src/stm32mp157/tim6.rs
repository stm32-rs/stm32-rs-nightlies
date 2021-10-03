#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM6 control register 1"]
    pub tim6_cr1: crate::Reg<tim6_cr1::TIM6_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM6 control register 2"]
    pub tim6_cr2: crate::Reg<tim6_cr2::TIM6_CR2_SPEC>,
    #[doc = "0x08 - TIM6 slave mode control register"]
    pub tim6_smcr: crate::Reg<tim6_smcr::TIM6_SMCR_SPEC>,
    #[doc = "0x0c - TIM6 DMA/interrupt enable register"]
    pub tim6_dier: crate::Reg<tim6_dier::TIM6_DIER_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM6 status register"]
    pub tim6_sr: crate::Reg<tim6_sr::TIM6_SR_SPEC>,
    #[doc = "0x14 - TIM6 event generation register"]
    pub tim6_egr: crate::Reg<tim6_egr::TIM6_EGR_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim6_ccmr1alternate6: crate::Reg<tim6_ccmr1alternate6::TIM6_CCMR1ALTERNATE6_SPEC>,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim6_ccmr2alternate22: crate::Reg<tim6_ccmr2alternate22::TIM6_CCMR2ALTERNATE22_SPEC>,
    #[doc = "0x20 - TIM6 capture/compare enable register"]
    pub tim6_ccer: crate::Reg<tim6_ccer::TIM6_CCER_SPEC>,
    #[doc = "0x24 - TIM6 counter"]
    pub tim6_cnt: crate::Reg<tim6_cnt::TIM6_CNT_SPEC>,
    #[doc = "0x28 - TIM6 prescaler"]
    pub tim6_psc: crate::Reg<tim6_psc::TIM6_PSC_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM6 auto-reload register"]
    pub tim6_arr: crate::Reg<tim6_arr::TIM6_ARR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - TIM6 repetition counter register"]
    pub tim6_rcr: crate::Reg<tim6_rcr::TIM6_RCR_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM6 capture/compare register 1"]
    pub tim6_ccr1: crate::Reg<tim6_ccr1::TIM6_CCR1_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - TIM6 capture/compare register 2"]
    pub tim6_ccr2: crate::Reg<tim6_ccr2::TIM6_CCR2_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - TIM6 capture/compare register 3"]
    pub tim6_ccr3: crate::Reg<tim6_ccr3::TIM6_CCR3_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - TIM6 capture/compare register 4"]
    pub tim6_ccr4: crate::Reg<tim6_ccr4::TIM6_CCR4_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim6_bdtr: crate::Reg<tim6_bdtr::TIM6_BDTR_SPEC>,
    #[doc = "0x48 - TIM6 DMA control register"]
    pub tim6_dcr: crate::Reg<tim6_dcr::TIM6_DCR_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - TIM6 DMA address for full transfer"]
    pub tim6_dmar: crate::Reg<tim6_dmar::TIM6_DMAR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim6_ccmr3: crate::Reg<tim6_ccmr3::TIM6_CCMR3_SPEC>,
    #[doc = "0x58 - TIM6 capture/compare register 5"]
    pub tim6_ccr5: crate::Reg<tim6_ccr5::TIM6_CCR5_SPEC>,
    #[doc = "0x5c - TIM6 capture/compare register 6"]
    pub tim6_ccr6: crate::Reg<tim6_ccr6::TIM6_CCR6_SPEC>,
}
#[doc = "TIM6_CR1 register accessor: an alias for `Reg<TIM6_CR1_SPEC>`"]
pub type TIM6_CR1 = crate::Reg<tim6_cr1::TIM6_CR1_SPEC>;
#[doc = "TIM6 control register 1"]
pub mod tim6_cr1;
#[doc = "TIM6_CR2 register accessor: an alias for `Reg<TIM6_CR2_SPEC>`"]
pub type TIM6_CR2 = crate::Reg<tim6_cr2::TIM6_CR2_SPEC>;
#[doc = "TIM6 control register 2"]
pub mod tim6_cr2;
#[doc = "TIM6_SMCR register accessor: an alias for `Reg<TIM6_SMCR_SPEC>`"]
pub type TIM6_SMCR = crate::Reg<tim6_smcr::TIM6_SMCR_SPEC>;
#[doc = "TIM6 slave mode control register"]
pub mod tim6_smcr;
#[doc = "TIM6_DIER register accessor: an alias for `Reg<TIM6_DIER_SPEC>`"]
pub type TIM6_DIER = crate::Reg<tim6_dier::TIM6_DIER_SPEC>;
#[doc = "TIM6 DMA/interrupt enable register"]
pub mod tim6_dier;
#[doc = "TIM6_SR register accessor: an alias for `Reg<TIM6_SR_SPEC>`"]
pub type TIM6_SR = crate::Reg<tim6_sr::TIM6_SR_SPEC>;
#[doc = "TIM6 status register"]
pub mod tim6_sr;
#[doc = "TIM6_EGR register accessor: an alias for `Reg<TIM6_EGR_SPEC>`"]
pub type TIM6_EGR = crate::Reg<tim6_egr::TIM6_EGR_SPEC>;
#[doc = "TIM6 event generation register"]
pub mod tim6_egr;
#[doc = "TIM6_CCMR1ALTERNATE6 register accessor: an alias for `Reg<TIM6_CCMR1ALTERNATE6_SPEC>`"]
pub type TIM6_CCMR1ALTERNATE6 = crate::Reg<tim6_ccmr1alternate6::TIM6_CCMR1ALTERNATE6_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim6_ccmr1alternate6;
#[doc = "TIM6_CCMR2ALTERNATE22 register accessor: an alias for `Reg<TIM6_CCMR2ALTERNATE22_SPEC>`"]
pub type TIM6_CCMR2ALTERNATE22 = crate::Reg<tim6_ccmr2alternate22::TIM6_CCMR2ALTERNATE22_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim6_ccmr2alternate22;
#[doc = "TIM6_CCER register accessor: an alias for `Reg<TIM6_CCER_SPEC>`"]
pub type TIM6_CCER = crate::Reg<tim6_ccer::TIM6_CCER_SPEC>;
#[doc = "TIM6 capture/compare enable register"]
pub mod tim6_ccer;
#[doc = "TIM6_CNT register accessor: an alias for `Reg<TIM6_CNT_SPEC>`"]
pub type TIM6_CNT = crate::Reg<tim6_cnt::TIM6_CNT_SPEC>;
#[doc = "TIM6 counter"]
pub mod tim6_cnt;
#[doc = "TIM6_PSC register accessor: an alias for `Reg<TIM6_PSC_SPEC>`"]
pub type TIM6_PSC = crate::Reg<tim6_psc::TIM6_PSC_SPEC>;
#[doc = "TIM6 prescaler"]
pub mod tim6_psc;
#[doc = "TIM6_ARR register accessor: an alias for `Reg<TIM6_ARR_SPEC>`"]
pub type TIM6_ARR = crate::Reg<tim6_arr::TIM6_ARR_SPEC>;
#[doc = "TIM6 auto-reload register"]
pub mod tim6_arr;
#[doc = "TIM6_RCR register accessor: an alias for `Reg<TIM6_RCR_SPEC>`"]
pub type TIM6_RCR = crate::Reg<tim6_rcr::TIM6_RCR_SPEC>;
#[doc = "TIM6 repetition counter register"]
pub mod tim6_rcr;
#[doc = "TIM6_CCR1 register accessor: an alias for `Reg<TIM6_CCR1_SPEC>`"]
pub type TIM6_CCR1 = crate::Reg<tim6_ccr1::TIM6_CCR1_SPEC>;
#[doc = "TIM6 capture/compare register 1"]
pub mod tim6_ccr1;
#[doc = "TIM6_CCR2 register accessor: an alias for `Reg<TIM6_CCR2_SPEC>`"]
pub type TIM6_CCR2 = crate::Reg<tim6_ccr2::TIM6_CCR2_SPEC>;
#[doc = "TIM6 capture/compare register 2"]
pub mod tim6_ccr2;
#[doc = "TIM6_CCR3 register accessor: an alias for `Reg<TIM6_CCR3_SPEC>`"]
pub type TIM6_CCR3 = crate::Reg<tim6_ccr3::TIM6_CCR3_SPEC>;
#[doc = "TIM6 capture/compare register 3"]
pub mod tim6_ccr3;
#[doc = "TIM6_CCR4 register accessor: an alias for `Reg<TIM6_CCR4_SPEC>`"]
pub type TIM6_CCR4 = crate::Reg<tim6_ccr4::TIM6_CCR4_SPEC>;
#[doc = "TIM6 capture/compare register 4"]
pub mod tim6_ccr4;
#[doc = "TIM6_BDTR register accessor: an alias for `Reg<TIM6_BDTR_SPEC>`"]
pub type TIM6_BDTR = crate::Reg<tim6_bdtr::TIM6_BDTR_SPEC>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim6_bdtr;
#[doc = "TIM6_DCR register accessor: an alias for `Reg<TIM6_DCR_SPEC>`"]
pub type TIM6_DCR = crate::Reg<tim6_dcr::TIM6_DCR_SPEC>;
#[doc = "TIM6 DMA control register"]
pub mod tim6_dcr;
#[doc = "TIM6_DMAR register accessor: an alias for `Reg<TIM6_DMAR_SPEC>`"]
pub type TIM6_DMAR = crate::Reg<tim6_dmar::TIM6_DMAR_SPEC>;
#[doc = "TIM6 DMA address for full transfer"]
pub mod tim6_dmar;
#[doc = "TIM6_CCMR3 register accessor: an alias for `Reg<TIM6_CCMR3_SPEC>`"]
pub type TIM6_CCMR3 = crate::Reg<tim6_ccmr3::TIM6_CCMR3_SPEC>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim6_ccmr3;
#[doc = "TIM6_CCR5 register accessor: an alias for `Reg<TIM6_CCR5_SPEC>`"]
pub type TIM6_CCR5 = crate::Reg<tim6_ccr5::TIM6_CCR5_SPEC>;
#[doc = "TIM6 capture/compare register 5"]
pub mod tim6_ccr5;
#[doc = "TIM6_CCR6 register accessor: an alias for `Reg<TIM6_CCR6_SPEC>`"]
pub type TIM6_CCR6 = crate::Reg<tim6_ccr6::TIM6_CCR6_SPEC>;
#[doc = "TIM6 capture/compare register 6"]
pub mod tim6_ccr6;
