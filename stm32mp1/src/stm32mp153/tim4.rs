#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim4_cr1: TIM4_CR1,
    _reserved1: [u8; 0x02],
    tim4_cr2: TIM4_CR2,
    tim4_smcr: TIM4_SMCR,
    tim4_dier: TIM4_DIER,
    _reserved4: [u8; 0x02],
    tim4_sr: TIM4_SR,
    tim4_egr: TIM4_EGR,
    _reserved6: [u8; 0x02],
    tim4_ccmr1alternate4: TIM4_CCMR1ALTERNATE4,
    tim4_ccmr2alternate20: TIM4_CCMR2ALTERNATE20,
    tim4_ccer: TIM4_CCER,
    tim4_cnt: TIM4_CNT,
    tim4_psc: TIM4_PSC,
    _reserved11: [u8; 0x02],
    tim4_arr: TIM4_ARR,
    _reserved12: [u8; 0x02],
    tim4_rcr: TIM4_RCR,
    _reserved13: [u8; 0x02],
    tim4_ccr1: TIM4_CCR1,
    _reserved14: [u8; 0x02],
    tim4_ccr2: TIM4_CCR2,
    _reserved15: [u8; 0x02],
    tim4_ccr3: TIM4_CCR3,
    _reserved16: [u8; 0x02],
    tim4_ccr4: TIM4_CCR4,
    _reserved17: [u8; 0x02],
    tim4_bdtr: TIM4_BDTR,
    tim4_dcr: TIM4_DCR,
    _reserved19: [u8; 0x02],
    tim4_dmar: TIM4_DMAR,
    _reserved20: [u8; 0x04],
    tim4_ccmr3: TIM4_CCMR3,
    tim4_ccr5: TIM4_CCR5,
    tim4_ccr6: TIM4_CCR6,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM4 control register 1"]
    #[inline(always)]
    pub const fn tim4_cr1(&self) -> &TIM4_CR1 {
        &self.tim4_cr1
    }
    #[doc = "0x04 - TIM4 control register 2"]
    #[inline(always)]
    pub const fn tim4_cr2(&self) -> &TIM4_CR2 {
        &self.tim4_cr2
    }
    #[doc = "0x08 - TIM4 slave mode control register"]
    #[inline(always)]
    pub const fn tim4_smcr(&self) -> &TIM4_SMCR {
        &self.tim4_smcr
    }
    #[doc = "0x0c - TIM4 DMA/interrupt enable register"]
    #[inline(always)]
    pub const fn tim4_dier(&self) -> &TIM4_DIER {
        &self.tim4_dier
    }
    #[doc = "0x10 - TIM4 status register"]
    #[inline(always)]
    pub const fn tim4_sr(&self) -> &TIM4_SR {
        &self.tim4_sr
    }
    #[doc = "0x14 - TIM4 event generation register"]
    #[inline(always)]
    pub const fn tim4_egr(&self) -> &TIM4_EGR {
        &self.tim4_egr
    }
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    #[inline(always)]
    pub const fn tim4_ccmr1alternate4(&self) -> &TIM4_CCMR1ALTERNATE4 {
        &self.tim4_ccmr1alternate4
    }
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    #[inline(always)]
    pub const fn tim4_ccmr2alternate20(&self) -> &TIM4_CCMR2ALTERNATE20 {
        &self.tim4_ccmr2alternate20
    }
    #[doc = "0x20 - TIM4 capture/compare enable register"]
    #[inline(always)]
    pub const fn tim4_ccer(&self) -> &TIM4_CCER {
        &self.tim4_ccer
    }
    #[doc = "0x24 - TIM4 counter"]
    #[inline(always)]
    pub const fn tim4_cnt(&self) -> &TIM4_CNT {
        &self.tim4_cnt
    }
    #[doc = "0x28 - TIM4 prescaler"]
    #[inline(always)]
    pub const fn tim4_psc(&self) -> &TIM4_PSC {
        &self.tim4_psc
    }
    #[doc = "0x2c - TIM4 auto-reload register"]
    #[inline(always)]
    pub const fn tim4_arr(&self) -> &TIM4_ARR {
        &self.tim4_arr
    }
    #[doc = "0x30 - TIM4 repetition counter register"]
    #[inline(always)]
    pub const fn tim4_rcr(&self) -> &TIM4_RCR {
        &self.tim4_rcr
    }
    #[doc = "0x34 - TIM4 capture/compare register 1"]
    #[inline(always)]
    pub const fn tim4_ccr1(&self) -> &TIM4_CCR1 {
        &self.tim4_ccr1
    }
    #[doc = "0x38 - TIM4 capture/compare register 2"]
    #[inline(always)]
    pub const fn tim4_ccr2(&self) -> &TIM4_CCR2 {
        &self.tim4_ccr2
    }
    #[doc = "0x3c - TIM4 capture/compare register 3"]
    #[inline(always)]
    pub const fn tim4_ccr3(&self) -> &TIM4_CCR3 {
        &self.tim4_ccr3
    }
    #[doc = "0x40 - TIM4 capture/compare register 4"]
    #[inline(always)]
    pub const fn tim4_ccr4(&self) -> &TIM4_CCR4 {
        &self.tim4_ccr4
    }
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    #[inline(always)]
    pub const fn tim4_bdtr(&self) -> &TIM4_BDTR {
        &self.tim4_bdtr
    }
    #[doc = "0x48 - TIM4 DMA control register"]
    #[inline(always)]
    pub const fn tim4_dcr(&self) -> &TIM4_DCR {
        &self.tim4_dcr
    }
    #[doc = "0x4c - TIM4 DMA address for full transfer"]
    #[inline(always)]
    pub const fn tim4_dmar(&self) -> &TIM4_DMAR {
        &self.tim4_dmar
    }
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    #[inline(always)]
    pub const fn tim4_ccmr3(&self) -> &TIM4_CCMR3 {
        &self.tim4_ccmr3
    }
    #[doc = "0x58 - TIM4 capture/compare register 5"]
    #[inline(always)]
    pub const fn tim4_ccr5(&self) -> &TIM4_CCR5 {
        &self.tim4_ccr5
    }
    #[doc = "0x5c - TIM4 capture/compare register 6"]
    #[inline(always)]
    pub const fn tim4_ccr6(&self) -> &TIM4_CCR6 {
        &self.tim4_ccr6
    }
}
#[doc = "TIM4_CR1 (rw) register accessor: TIM4 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_cr1`]
module"]
pub type TIM4_CR1 = crate::Reg<tim4_cr1::TIM4_CR1rs>;
#[doc = "TIM4 control register 1"]
pub mod tim4_cr1;
#[doc = "TIM4_CR2 (rw) register accessor: TIM4 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_cr2`]
module"]
pub type TIM4_CR2 = crate::Reg<tim4_cr2::TIM4_CR2rs>;
#[doc = "TIM4 control register 2"]
pub mod tim4_cr2;
#[doc = "TIM4_SMCR (rw) register accessor: TIM4 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_smcr`]
module"]
pub type TIM4_SMCR = crate::Reg<tim4_smcr::TIM4_SMCRrs>;
#[doc = "TIM4 slave mode control register"]
pub mod tim4_smcr;
#[doc = "TIM4_DIER (rw) register accessor: TIM4 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_dier`]
module"]
pub type TIM4_DIER = crate::Reg<tim4_dier::TIM4_DIERrs>;
#[doc = "TIM4 DMA/interrupt enable register"]
pub mod tim4_dier;
#[doc = "TIM4_SR (rw) register accessor: TIM4 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_sr`]
module"]
pub type TIM4_SR = crate::Reg<tim4_sr::TIM4_SRrs>;
#[doc = "TIM4 status register"]
pub mod tim4_sr;
#[doc = "TIM4_EGR (w) register accessor: TIM4 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_egr`]
module"]
pub type TIM4_EGR = crate::Reg<tim4_egr::TIM4_EGRrs>;
#[doc = "TIM4 event generation register"]
pub mod tim4_egr;
#[doc = "TIM4_CCMR1ALTERNATE4 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccmr1alternate4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccmr1alternate4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccmr1alternate4`]
module"]
pub type TIM4_CCMR1ALTERNATE4 = crate::Reg<tim4_ccmr1alternate4::TIM4_CCMR1ALTERNATE4rs>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim4_ccmr1alternate4;
#[doc = "TIM4_CCMR2ALTERNATE20 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccmr2alternate20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccmr2alternate20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccmr2alternate20`]
module"]
pub type TIM4_CCMR2ALTERNATE20 = crate::Reg<tim4_ccmr2alternate20::TIM4_CCMR2ALTERNATE20rs>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim4_ccmr2alternate20;
#[doc = "TIM4_CCER (rw) register accessor: TIM4 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccer`]
module"]
pub type TIM4_CCER = crate::Reg<tim4_ccer::TIM4_CCERrs>;
#[doc = "TIM4 capture/compare enable register"]
pub mod tim4_ccer;
#[doc = "TIM4_CNT (rw) register accessor: TIM4 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_cnt`]
module"]
pub type TIM4_CNT = crate::Reg<tim4_cnt::TIM4_CNTrs>;
#[doc = "TIM4 counter"]
pub mod tim4_cnt;
#[doc = "TIM4_PSC (rw) register accessor: TIM4 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_psc`]
module"]
pub type TIM4_PSC = crate::Reg<tim4_psc::TIM4_PSCrs>;
#[doc = "TIM4 prescaler"]
pub mod tim4_psc;
#[doc = "TIM4_ARR (rw) register accessor: TIM4 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_arr`]
module"]
pub type TIM4_ARR = crate::Reg<tim4_arr::TIM4_ARRrs>;
#[doc = "TIM4 auto-reload register"]
pub mod tim4_arr;
#[doc = "TIM4_RCR (rw) register accessor: TIM4 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_rcr`]
module"]
pub type TIM4_RCR = crate::Reg<tim4_rcr::TIM4_RCRrs>;
#[doc = "TIM4 repetition counter register"]
pub mod tim4_rcr;
#[doc = "TIM4_CCR1 (rw) register accessor: TIM4 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccr1`]
module"]
pub type TIM4_CCR1 = crate::Reg<tim4_ccr1::TIM4_CCR1rs>;
#[doc = "TIM4 capture/compare register 1"]
pub mod tim4_ccr1;
#[doc = "TIM4_CCR2 (rw) register accessor: TIM4 capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccr2`]
module"]
pub type TIM4_CCR2 = crate::Reg<tim4_ccr2::TIM4_CCR2rs>;
#[doc = "TIM4 capture/compare register 2"]
pub mod tim4_ccr2;
#[doc = "TIM4_CCR3 (rw) register accessor: TIM4 capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccr3`]
module"]
pub type TIM4_CCR3 = crate::Reg<tim4_ccr3::TIM4_CCR3rs>;
#[doc = "TIM4 capture/compare register 3"]
pub mod tim4_ccr3;
#[doc = "TIM4_CCR4 (rw) register accessor: TIM4 capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccr4`]
module"]
pub type TIM4_CCR4 = crate::Reg<tim4_ccr4::TIM4_CCR4rs>;
#[doc = "TIM4 capture/compare register 4"]
pub mod tim4_ccr4;
#[doc = "TIM4_BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_bdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_bdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_bdtr`]
module"]
pub type TIM4_BDTR = crate::Reg<tim4_bdtr::TIM4_BDTRrs>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim4_bdtr;
#[doc = "TIM4_DCR (rw) register accessor: TIM4 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_dcr`]
module"]
pub type TIM4_DCR = crate::Reg<tim4_dcr::TIM4_DCRrs>;
#[doc = "TIM4 DMA control register"]
pub mod tim4_dcr;
#[doc = "TIM4_DMAR (rw) register accessor: TIM4 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_dmar`]
module"]
pub type TIM4_DMAR = crate::Reg<tim4_dmar::TIM4_DMARrs>;
#[doc = "TIM4 DMA address for full transfer"]
pub mod tim4_dmar;
#[doc = "TIM4_CCMR3 (rw) register accessor: The channels 5 and 6 can only be configured in output. Output compare mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccmr3`]
module"]
pub type TIM4_CCMR3 = crate::Reg<tim4_ccmr3::TIM4_CCMR3rs>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim4_ccmr3;
#[doc = "TIM4_CCR5 (rw) register accessor: TIM4 capture/compare register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccr5`]
module"]
pub type TIM4_CCR5 = crate::Reg<tim4_ccr5::TIM4_CCR5rs>;
#[doc = "TIM4 capture/compare register 5"]
pub mod tim4_ccr5;
#[doc = "TIM4_CCR6 (rw) register accessor: TIM4 capture/compare register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim4_ccr6`]
module"]
pub type TIM4_CCR6 = crate::Reg<tim4_ccr6::TIM4_CCR6rs>;
#[doc = "TIM4 capture/compare register 6"]
pub mod tim4_ccr6;
