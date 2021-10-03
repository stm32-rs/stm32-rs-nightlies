#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM8 control register 1"]
    pub tim8_cr1: crate::Reg<tim8_cr1::TIM8_CR1_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM8 control register 2"]
    pub tim8_cr2: crate::Reg<tim8_cr2::TIM8_CR2_SPEC>,
    #[doc = "0x08 - TIM8 slave mode control register"]
    pub tim8_smcr: crate::Reg<tim8_smcr::TIM8_SMCR_SPEC>,
    #[doc = "0x0c - TIM8 DMA/interrupt enable register"]
    pub tim8_dier: crate::Reg<tim8_dier::TIM8_DIER_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM8 status register"]
    pub tim8_sr: crate::Reg<tim8_sr::TIM8_SR_SPEC>,
    #[doc = "0x14 - TIM8 event generation register"]
    pub tim8_egr: crate::Reg<tim8_egr::TIM8_EGR_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim8_ccmr1alternate8: crate::Reg<tim8_ccmr1alternate8::TIM8_CCMR1ALTERNATE8_SPEC>,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim8_ccmr2alternate24: crate::Reg<tim8_ccmr2alternate24::TIM8_CCMR2ALTERNATE24_SPEC>,
    #[doc = "0x20 - TIM8 capture/compare enable register"]
    pub tim8_ccer: crate::Reg<tim8_ccer::TIM8_CCER_SPEC>,
    #[doc = "0x24 - TIM8 counter"]
    pub tim8_cnt: crate::Reg<tim8_cnt::TIM8_CNT_SPEC>,
    #[doc = "0x28 - TIM8 prescaler"]
    pub tim8_psc: crate::Reg<tim8_psc::TIM8_PSC_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM8 auto-reload register"]
    pub tim8_arr: crate::Reg<tim8_arr::TIM8_ARR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - TIM8 repetition counter register"]
    pub tim8_rcr: crate::Reg<tim8_rcr::TIM8_RCR_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM8 capture/compare register 1"]
    pub tim8_ccr1: crate::Reg<tim8_ccr1::TIM8_CCR1_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - TIM8 capture/compare register 2"]
    pub tim8_ccr2: crate::Reg<tim8_ccr2::TIM8_CCR2_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - TIM8 capture/compare register 3"]
    pub tim8_ccr3: crate::Reg<tim8_ccr3::TIM8_CCR3_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - TIM8 capture/compare register 4"]
    pub tim8_ccr4: crate::Reg<tim8_ccr4::TIM8_CCR4_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim8_bdtr: crate::Reg<tim8_bdtr::TIM8_BDTR_SPEC>,
    #[doc = "0x48 - TIM8 DMA control register"]
    pub tim8_dcr: crate::Reg<tim8_dcr::TIM8_DCR_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - TIM8 DMA address for full transfer"]
    pub tim8_dmar: crate::Reg<tim8_dmar::TIM8_DMAR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim8_ccmr3: crate::Reg<tim8_ccmr3::TIM8_CCMR3_SPEC>,
    #[doc = "0x58 - TIM8 capture/compare register 5"]
    pub tim8_ccr5: crate::Reg<tim8_ccr5::TIM8_CCR5_SPEC>,
    #[doc = "0x5c - TIM8 capture/compare register 6"]
    pub tim8_ccr6: crate::Reg<tim8_ccr6::TIM8_CCR6_SPEC>,
    _reserved23: [u8; 0x02],
    #[doc = "0x60 - TIM8 Alternate function option register 1"]
    pub tim8_af1: crate::Reg<tim8_af1::TIM8_AF1_SPEC>,
    #[doc = "0x64 - TIM8 Alternate function option register 2"]
    pub tim8_af2: crate::Reg<tim8_af2::TIM8_AF2_SPEC>,
    #[doc = "0x68 - TIM8 timer input selection register"]
    pub tim8_tisel: crate::Reg<tim8_tisel::TIM8_TISEL_SPEC>,
}
#[doc = "TIM8_CR1 register accessor: an alias for `Reg<TIM8_CR1_SPEC>`"]
pub type TIM8_CR1 = crate::Reg<tim8_cr1::TIM8_CR1_SPEC>;
#[doc = "TIM8 control register 1"]
pub mod tim8_cr1;
#[doc = "TIM8_CR2 register accessor: an alias for `Reg<TIM8_CR2_SPEC>`"]
pub type TIM8_CR2 = crate::Reg<tim8_cr2::TIM8_CR2_SPEC>;
#[doc = "TIM8 control register 2"]
pub mod tim8_cr2;
#[doc = "TIM8_SMCR register accessor: an alias for `Reg<TIM8_SMCR_SPEC>`"]
pub type TIM8_SMCR = crate::Reg<tim8_smcr::TIM8_SMCR_SPEC>;
#[doc = "TIM8 slave mode control register"]
pub mod tim8_smcr;
#[doc = "TIM8_DIER register accessor: an alias for `Reg<TIM8_DIER_SPEC>`"]
pub type TIM8_DIER = crate::Reg<tim8_dier::TIM8_DIER_SPEC>;
#[doc = "TIM8 DMA/interrupt enable register"]
pub mod tim8_dier;
#[doc = "TIM8_SR register accessor: an alias for `Reg<TIM8_SR_SPEC>`"]
pub type TIM8_SR = crate::Reg<tim8_sr::TIM8_SR_SPEC>;
#[doc = "TIM8 status register"]
pub mod tim8_sr;
#[doc = "TIM8_EGR register accessor: an alias for `Reg<TIM8_EGR_SPEC>`"]
pub type TIM8_EGR = crate::Reg<tim8_egr::TIM8_EGR_SPEC>;
#[doc = "TIM8 event generation register"]
pub mod tim8_egr;
#[doc = "TIM8_CCMR1ALTERNATE8 register accessor: an alias for `Reg<TIM8_CCMR1ALTERNATE8_SPEC>`"]
pub type TIM8_CCMR1ALTERNATE8 = crate::Reg<tim8_ccmr1alternate8::TIM8_CCMR1ALTERNATE8_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim8_ccmr1alternate8;
#[doc = "TIM8_CCMR2ALTERNATE24 register accessor: an alias for `Reg<TIM8_CCMR2ALTERNATE24_SPEC>`"]
pub type TIM8_CCMR2ALTERNATE24 = crate::Reg<tim8_ccmr2alternate24::TIM8_CCMR2ALTERNATE24_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim8_ccmr2alternate24;
#[doc = "TIM8_CCER register accessor: an alias for `Reg<TIM8_CCER_SPEC>`"]
pub type TIM8_CCER = crate::Reg<tim8_ccer::TIM8_CCER_SPEC>;
#[doc = "TIM8 capture/compare enable register"]
pub mod tim8_ccer;
#[doc = "TIM8_CNT register accessor: an alias for `Reg<TIM8_CNT_SPEC>`"]
pub type TIM8_CNT = crate::Reg<tim8_cnt::TIM8_CNT_SPEC>;
#[doc = "TIM8 counter"]
pub mod tim8_cnt;
#[doc = "TIM8_PSC register accessor: an alias for `Reg<TIM8_PSC_SPEC>`"]
pub type TIM8_PSC = crate::Reg<tim8_psc::TIM8_PSC_SPEC>;
#[doc = "TIM8 prescaler"]
pub mod tim8_psc;
#[doc = "TIM8_ARR register accessor: an alias for `Reg<TIM8_ARR_SPEC>`"]
pub type TIM8_ARR = crate::Reg<tim8_arr::TIM8_ARR_SPEC>;
#[doc = "TIM8 auto-reload register"]
pub mod tim8_arr;
#[doc = "TIM8_RCR register accessor: an alias for `Reg<TIM8_RCR_SPEC>`"]
pub type TIM8_RCR = crate::Reg<tim8_rcr::TIM8_RCR_SPEC>;
#[doc = "TIM8 repetition counter register"]
pub mod tim8_rcr;
#[doc = "TIM8_CCR1 register accessor: an alias for `Reg<TIM8_CCR1_SPEC>`"]
pub type TIM8_CCR1 = crate::Reg<tim8_ccr1::TIM8_CCR1_SPEC>;
#[doc = "TIM8 capture/compare register 1"]
pub mod tim8_ccr1;
#[doc = "TIM8_CCR2 register accessor: an alias for `Reg<TIM8_CCR2_SPEC>`"]
pub type TIM8_CCR2 = crate::Reg<tim8_ccr2::TIM8_CCR2_SPEC>;
#[doc = "TIM8 capture/compare register 2"]
pub mod tim8_ccr2;
#[doc = "TIM8_CCR3 register accessor: an alias for `Reg<TIM8_CCR3_SPEC>`"]
pub type TIM8_CCR3 = crate::Reg<tim8_ccr3::TIM8_CCR3_SPEC>;
#[doc = "TIM8 capture/compare register 3"]
pub mod tim8_ccr3;
#[doc = "TIM8_CCR4 register accessor: an alias for `Reg<TIM8_CCR4_SPEC>`"]
pub type TIM8_CCR4 = crate::Reg<tim8_ccr4::TIM8_CCR4_SPEC>;
#[doc = "TIM8 capture/compare register 4"]
pub mod tim8_ccr4;
#[doc = "TIM8_BDTR register accessor: an alias for `Reg<TIM8_BDTR_SPEC>`"]
pub type TIM8_BDTR = crate::Reg<tim8_bdtr::TIM8_BDTR_SPEC>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim8_bdtr;
#[doc = "TIM8_DCR register accessor: an alias for `Reg<TIM8_DCR_SPEC>`"]
pub type TIM8_DCR = crate::Reg<tim8_dcr::TIM8_DCR_SPEC>;
#[doc = "TIM8 DMA control register"]
pub mod tim8_dcr;
#[doc = "TIM8_DMAR register accessor: an alias for `Reg<TIM8_DMAR_SPEC>`"]
pub type TIM8_DMAR = crate::Reg<tim8_dmar::TIM8_DMAR_SPEC>;
#[doc = "TIM8 DMA address for full transfer"]
pub mod tim8_dmar;
#[doc = "TIM8_CCMR3 register accessor: an alias for `Reg<TIM8_CCMR3_SPEC>`"]
pub type TIM8_CCMR3 = crate::Reg<tim8_ccmr3::TIM8_CCMR3_SPEC>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim8_ccmr3;
#[doc = "TIM8_CCR5 register accessor: an alias for `Reg<TIM8_CCR5_SPEC>`"]
pub type TIM8_CCR5 = crate::Reg<tim8_ccr5::TIM8_CCR5_SPEC>;
#[doc = "TIM8 capture/compare register 5"]
pub mod tim8_ccr5;
#[doc = "TIM8_CCR6 register accessor: an alias for `Reg<TIM8_CCR6_SPEC>`"]
pub type TIM8_CCR6 = crate::Reg<tim8_ccr6::TIM8_CCR6_SPEC>;
#[doc = "TIM8 capture/compare register 6"]
pub mod tim8_ccr6;
#[doc = "TIM8_AF1 register accessor: an alias for `Reg<TIM8_AF1_SPEC>`"]
pub type TIM8_AF1 = crate::Reg<tim8_af1::TIM8_AF1_SPEC>;
#[doc = "TIM8 Alternate function option register 1"]
pub mod tim8_af1;
#[doc = "TIM8_AF2 register accessor: an alias for `Reg<TIM8_AF2_SPEC>`"]
pub type TIM8_AF2 = crate::Reg<tim8_af2::TIM8_AF2_SPEC>;
#[doc = "TIM8 Alternate function option register 2"]
pub mod tim8_af2;
#[doc = "TIM8_TISEL register accessor: an alias for `Reg<TIM8_TISEL_SPEC>`"]
pub type TIM8_TISEL = crate::Reg<tim8_tisel::TIM8_TISEL_SPEC>;
#[doc = "TIM8 timer input selection register"]
pub mod tim8_tisel;
