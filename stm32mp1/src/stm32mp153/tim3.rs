#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim3_cr1: TIM3_CR1,
    _reserved1: [u8; 0x02],
    tim3_cr2: TIM3_CR2,
    tim3_smcr: TIM3_SMCR,
    tim3_dier: TIM3_DIER,
    _reserved4: [u8; 0x02],
    tim3_sr: TIM3_SR,
    tim3_egr: TIM3_EGR,
    _reserved6: [u8; 0x02],
    tim3_ccmr1alternate3: TIM3_CCMR1ALTERNATE3,
    tim3_ccmr2alternate19: TIM3_CCMR2ALTERNATE19,
    tim3_ccer: TIM3_CCER,
    tim3_cnt: TIM3_CNT,
    tim3_psc: TIM3_PSC,
    _reserved11: [u8; 0x02],
    tim3_arr: TIM3_ARR,
    _reserved12: [u8; 0x02],
    tim3_rcr: TIM3_RCR,
    _reserved13: [u8; 0x02],
    tim3_ccr1: TIM3_CCR1,
    _reserved14: [u8; 0x02],
    tim3_ccr2: TIM3_CCR2,
    _reserved15: [u8; 0x02],
    tim3_ccr3: TIM3_CCR3,
    _reserved16: [u8; 0x02],
    tim3_ccr4: TIM3_CCR4,
    _reserved17: [u8; 0x02],
    tim3_bdtr: TIM3_BDTR,
    tim3_dcr: TIM3_DCR,
    _reserved19: [u8; 0x02],
    tim3_dmar: TIM3_DMAR,
    _reserved20: [u8; 0x04],
    tim3_ccmr3: TIM3_CCMR3,
    tim3_ccr5: TIM3_CCR5,
    tim3_ccr6: TIM3_CCR6,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM3 control register 1"]
    #[inline(always)]
    pub const fn tim3_cr1(&self) -> &TIM3_CR1 {
        &self.tim3_cr1
    }
    #[doc = "0x04 - TIM3 control register 2"]
    #[inline(always)]
    pub const fn tim3_cr2(&self) -> &TIM3_CR2 {
        &self.tim3_cr2
    }
    #[doc = "0x08 - TIM3 slave mode control register"]
    #[inline(always)]
    pub const fn tim3_smcr(&self) -> &TIM3_SMCR {
        &self.tim3_smcr
    }
    #[doc = "0x0c - TIM3 DMA/interrupt enable register"]
    #[inline(always)]
    pub const fn tim3_dier(&self) -> &TIM3_DIER {
        &self.tim3_dier
    }
    #[doc = "0x10 - TIM3 status register"]
    #[inline(always)]
    pub const fn tim3_sr(&self) -> &TIM3_SR {
        &self.tim3_sr
    }
    #[doc = "0x14 - TIM3 event generation register"]
    #[inline(always)]
    pub const fn tim3_egr(&self) -> &TIM3_EGR {
        &self.tim3_egr
    }
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    #[inline(always)]
    pub const fn tim3_ccmr1alternate3(&self) -> &TIM3_CCMR1ALTERNATE3 {
        &self.tim3_ccmr1alternate3
    }
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    #[inline(always)]
    pub const fn tim3_ccmr2alternate19(&self) -> &TIM3_CCMR2ALTERNATE19 {
        &self.tim3_ccmr2alternate19
    }
    #[doc = "0x20 - TIM3 capture/compare enable register"]
    #[inline(always)]
    pub const fn tim3_ccer(&self) -> &TIM3_CCER {
        &self.tim3_ccer
    }
    #[doc = "0x24 - TIM3 counter"]
    #[inline(always)]
    pub const fn tim3_cnt(&self) -> &TIM3_CNT {
        &self.tim3_cnt
    }
    #[doc = "0x28 - TIM3 prescaler"]
    #[inline(always)]
    pub const fn tim3_psc(&self) -> &TIM3_PSC {
        &self.tim3_psc
    }
    #[doc = "0x2c - TIM3 auto-reload register"]
    #[inline(always)]
    pub const fn tim3_arr(&self) -> &TIM3_ARR {
        &self.tim3_arr
    }
    #[doc = "0x30 - TIM3 repetition counter register"]
    #[inline(always)]
    pub const fn tim3_rcr(&self) -> &TIM3_RCR {
        &self.tim3_rcr
    }
    #[doc = "0x34 - TIM3 capture/compare register 1"]
    #[inline(always)]
    pub const fn tim3_ccr1(&self) -> &TIM3_CCR1 {
        &self.tim3_ccr1
    }
    #[doc = "0x38 - TIM3 capture/compare register 2"]
    #[inline(always)]
    pub const fn tim3_ccr2(&self) -> &TIM3_CCR2 {
        &self.tim3_ccr2
    }
    #[doc = "0x3c - TIM3 capture/compare register 3"]
    #[inline(always)]
    pub const fn tim3_ccr3(&self) -> &TIM3_CCR3 {
        &self.tim3_ccr3
    }
    #[doc = "0x40 - TIM3 capture/compare register 4"]
    #[inline(always)]
    pub const fn tim3_ccr4(&self) -> &TIM3_CCR4 {
        &self.tim3_ccr4
    }
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    #[inline(always)]
    pub const fn tim3_bdtr(&self) -> &TIM3_BDTR {
        &self.tim3_bdtr
    }
    #[doc = "0x48 - TIM3 DMA control register"]
    #[inline(always)]
    pub const fn tim3_dcr(&self) -> &TIM3_DCR {
        &self.tim3_dcr
    }
    #[doc = "0x4c - TIM3 DMA address for full transfer"]
    #[inline(always)]
    pub const fn tim3_dmar(&self) -> &TIM3_DMAR {
        &self.tim3_dmar
    }
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    #[inline(always)]
    pub const fn tim3_ccmr3(&self) -> &TIM3_CCMR3 {
        &self.tim3_ccmr3
    }
    #[doc = "0x58 - TIM3 capture/compare register 5"]
    #[inline(always)]
    pub const fn tim3_ccr5(&self) -> &TIM3_CCR5 {
        &self.tim3_ccr5
    }
    #[doc = "0x5c - TIM3 capture/compare register 6"]
    #[inline(always)]
    pub const fn tim3_ccr6(&self) -> &TIM3_CCR6 {
        &self.tim3_ccr6
    }
}
#[doc = "TIM3_CR1 (rw) register accessor: TIM3 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_cr1`]
module"]
pub type TIM3_CR1 = crate::Reg<tim3_cr1::TIM3_CR1rs>;
#[doc = "TIM3 control register 1"]
pub mod tim3_cr1;
#[doc = "TIM3_CR2 (rw) register accessor: TIM3 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_cr2`]
module"]
pub type TIM3_CR2 = crate::Reg<tim3_cr2::TIM3_CR2rs>;
#[doc = "TIM3 control register 2"]
pub mod tim3_cr2;
#[doc = "TIM3_SMCR (rw) register accessor: TIM3 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_smcr`]
module"]
pub type TIM3_SMCR = crate::Reg<tim3_smcr::TIM3_SMCRrs>;
#[doc = "TIM3 slave mode control register"]
pub mod tim3_smcr;
#[doc = "TIM3_DIER (rw) register accessor: TIM3 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_dier`]
module"]
pub type TIM3_DIER = crate::Reg<tim3_dier::TIM3_DIERrs>;
#[doc = "TIM3 DMA/interrupt enable register"]
pub mod tim3_dier;
#[doc = "TIM3_SR (rw) register accessor: TIM3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_sr`]
module"]
pub type TIM3_SR = crate::Reg<tim3_sr::TIM3_SRrs>;
#[doc = "TIM3 status register"]
pub mod tim3_sr;
#[doc = "TIM3_EGR (w) register accessor: TIM3 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_egr`]
module"]
pub type TIM3_EGR = crate::Reg<tim3_egr::TIM3_EGRrs>;
#[doc = "TIM3 event generation register"]
pub mod tim3_egr;
#[doc = "TIM3_CCMR1ALTERNATE3 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccmr1alternate3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccmr1alternate3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccmr1alternate3`]
module"]
pub type TIM3_CCMR1ALTERNATE3 = crate::Reg<tim3_ccmr1alternate3::TIM3_CCMR1ALTERNATE3rs>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim3_ccmr1alternate3;
#[doc = "TIM3_CCMR2ALTERNATE19 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccmr2alternate19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccmr2alternate19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccmr2alternate19`]
module"]
pub type TIM3_CCMR2ALTERNATE19 = crate::Reg<tim3_ccmr2alternate19::TIM3_CCMR2ALTERNATE19rs>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim3_ccmr2alternate19;
#[doc = "TIM3_CCER (rw) register accessor: TIM3 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccer`]
module"]
pub type TIM3_CCER = crate::Reg<tim3_ccer::TIM3_CCERrs>;
#[doc = "TIM3 capture/compare enable register"]
pub mod tim3_ccer;
#[doc = "TIM3_CNT (rw) register accessor: TIM3 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_cnt`]
module"]
pub type TIM3_CNT = crate::Reg<tim3_cnt::TIM3_CNTrs>;
#[doc = "TIM3 counter"]
pub mod tim3_cnt;
#[doc = "TIM3_PSC (rw) register accessor: TIM3 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_psc`]
module"]
pub type TIM3_PSC = crate::Reg<tim3_psc::TIM3_PSCrs>;
#[doc = "TIM3 prescaler"]
pub mod tim3_psc;
#[doc = "TIM3_ARR (rw) register accessor: TIM3 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_arr`]
module"]
pub type TIM3_ARR = crate::Reg<tim3_arr::TIM3_ARRrs>;
#[doc = "TIM3 auto-reload register"]
pub mod tim3_arr;
#[doc = "TIM3_RCR (rw) register accessor: TIM3 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_rcr`]
module"]
pub type TIM3_RCR = crate::Reg<tim3_rcr::TIM3_RCRrs>;
#[doc = "TIM3 repetition counter register"]
pub mod tim3_rcr;
#[doc = "TIM3_CCR1 (rw) register accessor: TIM3 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccr1`]
module"]
pub type TIM3_CCR1 = crate::Reg<tim3_ccr1::TIM3_CCR1rs>;
#[doc = "TIM3 capture/compare register 1"]
pub mod tim3_ccr1;
#[doc = "TIM3_CCR2 (rw) register accessor: TIM3 capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccr2`]
module"]
pub type TIM3_CCR2 = crate::Reg<tim3_ccr2::TIM3_CCR2rs>;
#[doc = "TIM3 capture/compare register 2"]
pub mod tim3_ccr2;
#[doc = "TIM3_CCR3 (rw) register accessor: TIM3 capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccr3`]
module"]
pub type TIM3_CCR3 = crate::Reg<tim3_ccr3::TIM3_CCR3rs>;
#[doc = "TIM3 capture/compare register 3"]
pub mod tim3_ccr3;
#[doc = "TIM3_CCR4 (rw) register accessor: TIM3 capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccr4`]
module"]
pub type TIM3_CCR4 = crate::Reg<tim3_ccr4::TIM3_CCR4rs>;
#[doc = "TIM3 capture/compare register 4"]
pub mod tim3_ccr4;
#[doc = "TIM3_BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_bdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_bdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_bdtr`]
module"]
pub type TIM3_BDTR = crate::Reg<tim3_bdtr::TIM3_BDTRrs>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim3_bdtr;
#[doc = "TIM3_DCR (rw) register accessor: TIM3 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_dcr`]
module"]
pub type TIM3_DCR = crate::Reg<tim3_dcr::TIM3_DCRrs>;
#[doc = "TIM3 DMA control register"]
pub mod tim3_dcr;
#[doc = "TIM3_DMAR (rw) register accessor: TIM3 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_dmar`]
module"]
pub type TIM3_DMAR = crate::Reg<tim3_dmar::TIM3_DMARrs>;
#[doc = "TIM3 DMA address for full transfer"]
pub mod tim3_dmar;
#[doc = "TIM3_CCMR3 (rw) register accessor: The channels 5 and 6 can only be configured in output. Output compare mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccmr3`]
module"]
pub type TIM3_CCMR3 = crate::Reg<tim3_ccmr3::TIM3_CCMR3rs>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim3_ccmr3;
#[doc = "TIM3_CCR5 (rw) register accessor: TIM3 capture/compare register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccr5`]
module"]
pub type TIM3_CCR5 = crate::Reg<tim3_ccr5::TIM3_CCR5rs>;
#[doc = "TIM3 capture/compare register 5"]
pub mod tim3_ccr5;
#[doc = "TIM3_CCR6 (rw) register accessor: TIM3 capture/compare register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim3_ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim3_ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim3_ccr6`]
module"]
pub type TIM3_CCR6 = crate::Reg<tim3_ccr6::TIM3_CCR6rs>;
#[doc = "TIM3 capture/compare register 6"]
pub mod tim3_ccr6;
