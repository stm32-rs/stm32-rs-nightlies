#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim15_cr1: TIM15_CR1,
    _reserved1: [u8; 0x02],
    tim15_cr2: TIM15_CR2,
    _reserved2: [u8; 0x02],
    timx_smcr: TIMX_SMCR,
    tim15_dier: TIM15_DIER,
    _reserved4: [u8; 0x02],
    tim15_sr: TIM15_SR,
    _reserved5: [u8; 0x02],
    timx_egr: TIMX_EGR,
    _reserved_6_timx_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    tim15_ccer: TIM15_CCER,
    _reserved8: [u8; 0x02],
    tim15_cnt: TIM15_CNT,
    tim15_psc: TIM15_PSC,
    _reserved10: [u8; 0x02],
    tim15_arr: TIM15_ARR,
    _reserved11: [u8; 0x02],
    tim15_rcr: TIM15_RCR,
    _reserved12: [u8; 0x02],
    tim15_ccr1: TIM15_CCR1,
    _reserved13: [u8; 0x02],
    tim15_ccr2: TIM15_CCR2,
    _reserved14: [u8; 0x0a],
    timx_bdtr: TIMX_BDTR,
    tim15_dcr: TIM15_DCR,
    _reserved16: [u8; 0x02],
    tim15_dmar: TIM15_DMAR,
    _reserved17: [u8; 0x12],
    tim15_af1: TIM15_AF1,
    _reserved18: [u8; 0x04],
    tim15_tisel: TIM15_TISEL,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM15 control register 1"]
    #[inline(always)]
    pub const fn tim15_cr1(&self) -> &TIM15_CR1 {
        &self.tim15_cr1
    }
    #[doc = "0x04 - TIM15 control register 2"]
    #[inline(always)]
    pub const fn tim15_cr2(&self) -> &TIM15_CR2 {
        &self.tim15_cr2
    }
    #[doc = "0x08 - slave mode control register"]
    #[inline(always)]
    pub const fn timx_smcr(&self) -> &TIMX_SMCR {
        &self.timx_smcr
    }
    #[doc = "0x0c - TIM15 DMA/interrupt enable register"]
    #[inline(always)]
    pub const fn tim15_dier(&self) -> &TIM15_DIER {
        &self.tim15_dier
    }
    #[doc = "0x10 - TIM15 status register"]
    #[inline(always)]
    pub const fn tim15_sr(&self) -> &TIM15_SR {
        &self.tim15_sr
    }
    #[doc = "0x14 - event generation register"]
    #[inline(always)]
    pub const fn timx_egr(&self) -> &TIMX_EGR {
        &self.timx_egr
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn timx_ccmr1_input(&self) -> &TIMX_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn timx_ccmr1_output(&self) -> &TIMX_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - TIM15 capture/compare enable register"]
    #[inline(always)]
    pub const fn tim15_ccer(&self) -> &TIM15_CCER {
        &self.tim15_ccer
    }
    #[doc = "0x24 - TIM15 counter"]
    #[inline(always)]
    pub const fn tim15_cnt(&self) -> &TIM15_CNT {
        &self.tim15_cnt
    }
    #[doc = "0x28 - TIM15 prescaler"]
    #[inline(always)]
    pub const fn tim15_psc(&self) -> &TIM15_PSC {
        &self.tim15_psc
    }
    #[doc = "0x2c - TIM15 auto-reload register"]
    #[inline(always)]
    pub const fn tim15_arr(&self) -> &TIM15_ARR {
        &self.tim15_arr
    }
    #[doc = "0x30 - TIM15 repetition counter register"]
    #[inline(always)]
    pub const fn tim15_rcr(&self) -> &TIM15_RCR {
        &self.tim15_rcr
    }
    #[doc = "0x34 - TIM15 capture/compare register 1"]
    #[inline(always)]
    pub const fn tim15_ccr1(&self) -> &TIM15_CCR1 {
        &self.tim15_ccr1
    }
    #[doc = "0x38 - TIM15 capture/compare register 2"]
    #[inline(always)]
    pub const fn tim15_ccr2(&self) -> &TIM15_CCR2 {
        &self.tim15_ccr2
    }
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    #[inline(always)]
    pub const fn timx_bdtr(&self) -> &TIMX_BDTR {
        &self.timx_bdtr
    }
    #[doc = "0x48 - TIM15 DMA control register"]
    #[inline(always)]
    pub const fn tim15_dcr(&self) -> &TIM15_DCR {
        &self.tim15_dcr
    }
    #[doc = "0x4c - TIM15 DMA address for full transfer"]
    #[inline(always)]
    pub const fn tim15_dmar(&self) -> &TIM15_DMAR {
        &self.tim15_dmar
    }
    #[doc = "0x60 - TIM15 alternate register 1"]
    #[inline(always)]
    pub const fn tim15_af1(&self) -> &TIM15_AF1 {
        &self.tim15_af1
    }
    #[doc = "0x68 - TIM15 input selection register"]
    #[inline(always)]
    pub const fn tim15_tisel(&self) -> &TIM15_TISEL {
        &self.tim15_tisel
    }
}
#[doc = "TIM15_CR1 (rw) register accessor: TIM15 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_cr1`]
module"]
pub type TIM15_CR1 = crate::Reg<tim15_cr1::TIM15_CR1rs>;
#[doc = "TIM15 control register 1"]
pub mod tim15_cr1;
#[doc = "TIM15_CR2 (rw) register accessor: TIM15 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_cr2`]
module"]
pub type TIM15_CR2 = crate::Reg<tim15_cr2::TIM15_CR2rs>;
#[doc = "TIM15 control register 2"]
pub mod tim15_cr2;
#[doc = "TIMx_SMCR (rw) register accessor: slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_smcr`]
module"]
#[doc(alias = "TIMx_SMCR")]
pub type TIMX_SMCR = crate::Reg<timx_smcr::TIMX_SMCRrs>;
#[doc = "slave mode control register"]
pub mod timx_smcr;
#[doc = "TIM15_DIER (rw) register accessor: TIM15 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_dier`]
module"]
pub type TIM15_DIER = crate::Reg<tim15_dier::TIM15_DIERrs>;
#[doc = "TIM15 DMA/interrupt enable register"]
pub mod tim15_dier;
#[doc = "TIM15_SR (rw) register accessor: TIM15 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_sr`]
module"]
pub type TIM15_SR = crate::Reg<tim15_sr::TIM15_SRrs>;
#[doc = "TIM15 status register"]
pub mod tim15_sr;
#[doc = "TIMx_EGR (w) register accessor: event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_egr`]
module"]
#[doc(alias = "TIMx_EGR")]
pub type TIMX_EGR = crate::Reg<timx_egr::TIMX_EGRrs>;
#[doc = "event generation register"]
pub mod timx_egr;
#[doc = "TIMx_CCMR1_Output (rw) register accessor: capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_ccmr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_ccmr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_ccmr1_output`]
module"]
#[doc(alias = "TIMx_CCMR1_Output")]
pub type TIMX_CCMR1_OUTPUT = crate::Reg<timx_ccmr1_output::TIMX_CCMR1_OUTPUTrs>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod timx_ccmr1_output;
#[doc = "TIMx_CCMR1_Input (rw) register accessor: capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_ccmr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_ccmr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_ccmr1_input`]
module"]
#[doc(alias = "TIMx_CCMR1_Input")]
pub type TIMX_CCMR1_INPUT = crate::Reg<timx_ccmr1_input::TIMX_CCMR1_INPUTrs>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod timx_ccmr1_input;
#[doc = "TIM15_CCER (rw) register accessor: TIM15 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_ccer`]
module"]
pub type TIM15_CCER = crate::Reg<tim15_ccer::TIM15_CCERrs>;
#[doc = "TIM15 capture/compare enable register"]
pub mod tim15_ccer;
#[doc = "TIM15_CNT (rw) register accessor: TIM15 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_cnt`]
module"]
pub type TIM15_CNT = crate::Reg<tim15_cnt::TIM15_CNTrs>;
#[doc = "TIM15 counter"]
pub mod tim15_cnt;
#[doc = "TIM15_PSC (rw) register accessor: TIM15 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_psc`]
module"]
pub type TIM15_PSC = crate::Reg<tim15_psc::TIM15_PSCrs>;
#[doc = "TIM15 prescaler"]
pub mod tim15_psc;
#[doc = "TIM15_ARR (rw) register accessor: TIM15 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_arr`]
module"]
pub type TIM15_ARR = crate::Reg<tim15_arr::TIM15_ARRrs>;
#[doc = "TIM15 auto-reload register"]
pub mod tim15_arr;
#[doc = "TIM15_RCR (rw) register accessor: TIM15 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_rcr`]
module"]
pub type TIM15_RCR = crate::Reg<tim15_rcr::TIM15_RCRrs>;
#[doc = "TIM15 repetition counter register"]
pub mod tim15_rcr;
#[doc = "TIM15_CCR1 (rw) register accessor: TIM15 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_ccr1`]
module"]
pub type TIM15_CCR1 = crate::Reg<tim15_ccr1::TIM15_CCR1rs>;
#[doc = "TIM15 capture/compare register 1"]
pub mod tim15_ccr1;
#[doc = "TIM15_CCR2 (rw) register accessor: TIM15 capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_ccr2`]
module"]
pub type TIM15_CCR2 = crate::Reg<tim15_ccr2::TIM15_CCR2rs>;
#[doc = "TIM15 capture/compare register 2"]
pub mod tim15_ccr2;
#[doc = "TIMx_BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_bdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_bdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timx_bdtr`]
module"]
#[doc(alias = "TIMx_BDTR")]
pub type TIMX_BDTR = crate::Reg<timx_bdtr::TIMX_BDTRrs>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod timx_bdtr;
#[doc = "TIM15_DCR (rw) register accessor: TIM15 DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_dcr`]
module"]
pub type TIM15_DCR = crate::Reg<tim15_dcr::TIM15_DCRrs>;
#[doc = "TIM15 DMA control register"]
pub mod tim15_dcr;
#[doc = "TIM15_DMAR (rw) register accessor: TIM15 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_dmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_dmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_dmar`]
module"]
pub type TIM15_DMAR = crate::Reg<tim15_dmar::TIM15_DMARrs>;
#[doc = "TIM15 DMA address for full transfer"]
pub mod tim15_dmar;
#[doc = "TIM15_AF1 (rw) register accessor: TIM15 alternate register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_af1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_af1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_af1`]
module"]
pub type TIM15_AF1 = crate::Reg<tim15_af1::TIM15_AF1rs>;
#[doc = "TIM15 alternate register 1"]
pub mod tim15_af1;
#[doc = "TIM15_TISEL (rw) register accessor: TIM15 input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_tisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_tisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim15_tisel`]
module"]
pub type TIM15_TISEL = crate::Reg<tim15_tisel::TIM15_TISELrs>;
#[doc = "TIM15 input selection register"]
pub mod tim15_tisel;
