#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timx_cr1: TIMX_CR1,
    _reserved1: [u8; 0x02],
    timx_cr2: TIMX_CR2,
    _reserved2: [u8; 0x06],
    timx_dier: TIMX_DIER,
    _reserved3: [u8; 0x02],
    timx_sr: TIMX_SR,
    _reserved4: [u8; 0x02],
    timx_egr: TIMX_EGR,
    _reserved5: [u8; 0x08],
    timx_ccer: TIMX_CCER,
    _reserved6: [u8; 0x02],
    timx_cnt: TIMX_CNT,
    timx_psc: TIMX_PSC,
    _reserved8: [u8; 0x02],
    timx_arr: TIMX_ARR,
    _reserved9: [u8; 0x02],
    timx_rcr: TIMX_RCR,
    _reserved10: [u8; 0x02],
    timx_ccr1: TIMX_CCR1,
    _reserved11: [u8; 0x0e],
    timx_bdtr: TIMX_BDTR,
    timx_dcr: TIMX_DCR,
    _reserved13: [u8; 0x02],
    timx_dmar: TIMX_DMAR,
    _reserved14: [u8; 0x12],
    timx_af1: TIMX_AF1,
    _reserved15: [u8; 0x04],
    timx_tisel: TIMX_TISEL,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM16/TIM17 control register 1"]
    #[inline(always)]
    pub const fn timx_cr1(&self) -> &TIMX_CR1 {
        &self.timx_cr1
    }
    #[doc = "0x04 - TIM16/TIM17 control register 2"]
    #[inline(always)]
    pub const fn timx_cr2(&self) -> &TIMX_CR2 {
        &self.timx_cr2
    }
    #[doc = "0x0c - TIM16/TIM17 DMA/interrupt enable register"]
    #[inline(always)]
    pub const fn timx_dier(&self) -> &TIMX_DIER {
        &self.timx_dier
    }
    #[doc = "0x10 - TIM16/TIM17 status register"]
    #[inline(always)]
    pub const fn timx_sr(&self) -> &TIMX_SR {
        &self.timx_sr
    }
    #[doc = "0x14 - event generation register"]
    #[inline(always)]
    pub const fn timx_egr(&self) -> &TIMX_EGR {
        &self.timx_egr
    }
    #[doc = "0x20 - TIM16/TIM17 capture/compare enable register"]
    #[inline(always)]
    pub const fn timx_ccer(&self) -> &TIMX_CCER {
        &self.timx_ccer
    }
    #[doc = "0x24 - TIM16/TIM17 counter"]
    #[inline(always)]
    pub const fn timx_cnt(&self) -> &TIMX_CNT {
        &self.timx_cnt
    }
    #[doc = "0x28 - TIM16/TIM17 prescaler"]
    #[inline(always)]
    pub const fn timx_psc(&self) -> &TIMX_PSC {
        &self.timx_psc
    }
    #[doc = "0x2c - TIM16/TIM17 auto-reload register"]
    #[inline(always)]
    pub const fn timx_arr(&self) -> &TIMX_ARR {
        &self.timx_arr
    }
    #[doc = "0x30 - TIM16/TIM17 repetition counter register"]
    #[inline(always)]
    pub const fn timx_rcr(&self) -> &TIMX_RCR {
        &self.timx_rcr
    }
    #[doc = "0x34 - TIM16/TIM17 capture/compare register 1"]
    #[inline(always)]
    pub const fn timx_ccr1(&self) -> &TIMX_CCR1 {
        &self.timx_ccr1
    }
    #[doc = "0x44 - As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    #[inline(always)]
    pub const fn timx_bdtr(&self) -> &TIMX_BDTR {
        &self.timx_bdtr
    }
    #[doc = "0x48 - TIM16/TIM17 DMA control register"]
    #[inline(always)]
    pub const fn timx_dcr(&self) -> &TIMX_DCR {
        &self.timx_dcr
    }
    #[doc = "0x4c - TIM16/TIM17 DMA address for full transfer"]
    #[inline(always)]
    pub const fn timx_dmar(&self) -> &TIMX_DMAR {
        &self.timx_dmar
    }
    #[doc = "0x60 - TIM17 alternate function register 1"]
    #[inline(always)]
    pub const fn timx_af1(&self) -> &TIMX_AF1 {
        &self.timx_af1
    }
    #[doc = "0x68 - TIM17 input selection register"]
    #[inline(always)]
    pub const fn timx_tisel(&self) -> &TIMX_TISEL {
        &self.timx_tisel
    }
}
#[doc = "TIMx_CR1 (rw) register accessor: TIM16/TIM17 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_cr1`]
module"]
#[doc(alias = "TIMx_CR1")]
pub type TIMX_CR1 = crate::Reg<timx_cr1::TIMX_CR1rs>;
#[doc = "TIM16/TIM17 control register 1"]
pub mod timx_cr1;
#[doc = "TIMx_CR2 (rw) register accessor: TIM16/TIM17 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_cr2`]
module"]
#[doc(alias = "TIMx_CR2")]
pub type TIMX_CR2 = crate::Reg<timx_cr2::TIMX_CR2rs>;
#[doc = "TIM16/TIM17 control register 2"]
pub mod timx_cr2;
#[doc = "TIMx_DIER (rw) register accessor: TIM16/TIM17 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_dier`]
module"]
#[doc(alias = "TIMx_DIER")]
pub type TIMX_DIER = crate::Reg<timx_dier::TIMX_DIERrs>;
#[doc = "TIM16/TIM17 DMA/interrupt enable register"]
pub mod timx_dier;
#[doc = "TIMx_SR (rw) register accessor: TIM16/TIM17 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_sr`]
module"]
#[doc(alias = "TIMx_SR")]
pub type TIMX_SR = crate::Reg<timx_sr::TIMX_SRrs>;
#[doc = "TIM16/TIM17 status register"]
pub mod timx_sr;
#[doc = "TIMx_EGR (w) register accessor: event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_egr`]
module"]
#[doc(alias = "TIMx_EGR")]
pub type TIMX_EGR = crate::Reg<timx_egr::TIMX_EGRrs>;
#[doc = "event generation register"]
pub mod timx_egr;
#[doc = "TIMx_CCER (rw) register accessor: TIM16/TIM17 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_ccer`]
module"]
#[doc(alias = "TIMx_CCER")]
pub type TIMX_CCER = crate::Reg<timx_ccer::TIMX_CCERrs>;
#[doc = "TIM16/TIM17 capture/compare enable register"]
pub mod timx_ccer;
#[doc = "TIMx_CNT (rw) register accessor: TIM16/TIM17 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_cnt`]
module"]
#[doc(alias = "TIMx_CNT")]
pub type TIMX_CNT = crate::Reg<timx_cnt::TIMX_CNTrs>;
#[doc = "TIM16/TIM17 counter"]
pub mod timx_cnt;
#[doc = "TIMx_PSC (rw) register accessor: TIM16/TIM17 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_psc`]
module"]
#[doc(alias = "TIMx_PSC")]
pub type TIMX_PSC = crate::Reg<timx_psc::TIMX_PSCrs>;
#[doc = "TIM16/TIM17 prescaler"]
pub mod timx_psc;
#[doc = "TIMx_ARR (rw) register accessor: TIM16/TIM17 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_arr`]
module"]
#[doc(alias = "TIMx_ARR")]
pub type TIMX_ARR = crate::Reg<timx_arr::TIMX_ARRrs>;
#[doc = "TIM16/TIM17 auto-reload register"]
pub mod timx_arr;
#[doc = "TIMx_RCR (rw) register accessor: TIM16/TIM17 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_rcr`]
module"]
#[doc(alias = "TIMx_RCR")]
pub type TIMX_RCR = crate::Reg<timx_rcr::TIMX_RCRrs>;
#[doc = "TIM16/TIM17 repetition counter register"]
pub mod timx_rcr;
#[doc = "TIMx_CCR1 (rw) register accessor: TIM16/TIM17 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_ccr1`]
module"]
#[doc(alias = "TIMx_CCR1")]
pub type TIMX_CCR1 = crate::Reg<timx_ccr1::TIMX_CCR1rs>;
#[doc = "TIM16/TIM17 capture/compare register 1"]
pub mod timx_ccr1;
#[doc = "TIMx_BDTR (rw) register accessor: As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_bdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_bdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_bdtr`]
module"]
#[doc(alias = "TIMx_BDTR")]
pub type TIMX_BDTR = crate::Reg<timx_bdtr::TIMX_BDTRrs>;
#[doc = "As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod timx_bdtr;
#[doc = "TIMx_DCR (rw) register accessor: TIM16/TIM17 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_dcr`]
module"]
#[doc(alias = "TIMx_DCR")]
pub type TIMX_DCR = crate::Reg<timx_dcr::TIMX_DCRrs>;
#[doc = "TIM16/TIM17 DMA control register"]
pub mod timx_dcr;
#[doc = "TIMx_DMAR (rw) register accessor: TIM16/TIM17 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_dmar`]
module"]
#[doc(alias = "TIMx_DMAR")]
pub type TIMX_DMAR = crate::Reg<timx_dmar::TIMX_DMARrs>;
#[doc = "TIM16/TIM17 DMA address for full transfer"]
pub mod timx_dmar;
#[doc = "TIMx_AF1 (rw) register accessor: TIM17 alternate function register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_af1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_af1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_af1`]
module"]
#[doc(alias = "TIMx_AF1")]
pub type TIMX_AF1 = crate::Reg<timx_af1::TIMX_AF1rs>;
#[doc = "TIM17 alternate function register 1"]
pub mod timx_af1;
#[doc = "TIMx_TISEL (rw) register accessor: TIM17 input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_tisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_tisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_tisel`]
module"]
#[doc(alias = "TIMx_TISEL")]
pub type TIMX_TISEL = crate::Reg<timx_tisel::TIMX_TISELrs>;
#[doc = "TIM17 input selection register"]
pub mod timx_tisel;
