#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tim14_cr1: TIM14_CR1,
    _reserved1: [u8; 0x0a],
    tim14_dier: TIM14_DIER,
    _reserved2: [u8; 0x02],
    tim14_sr: TIM14_SR,
    _reserved3: [u8; 0x02],
    tim14_egr: TIM14_EGR,
    _reserved4: [u8; 0x02],
    tim14_ccmr1: TIM14_CCMR1,
    _reserved5: [u8; 0x04],
    tim14_ccer: TIM14_CCER,
    _reserved6: [u8; 0x02],
    tim14_cnt: TIM14_CNT,
    tim14_psc: TIM14_PSC,
    _reserved8: [u8; 0x02],
    tim14_arr: TIM14_ARR,
    _reserved9: [u8; 0x06],
    tim14_ccr1: TIM14_CCR1,
    _reserved10: [u8; 0x32],
    tim14_tisel: TIM14_TISEL,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM14 control register 1"]
    #[inline(always)]
    pub const fn tim14_cr1(&self) -> &TIM14_CR1 {
        &self.tim14_cr1
    }
    #[doc = "0x0c - TIM14 Interrupt enable register"]
    #[inline(always)]
    pub const fn tim14_dier(&self) -> &TIM14_DIER {
        &self.tim14_dier
    }
    #[doc = "0x10 - TIM14 status register"]
    #[inline(always)]
    pub const fn tim14_sr(&self) -> &TIM14_SR {
        &self.tim14_sr
    }
    #[doc = "0x14 - TIM14 event generation register"]
    #[inline(always)]
    pub const fn tim14_egr(&self) -> &TIM14_EGR {
        &self.tim14_egr
    }
    #[doc = "0x18 - The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode"]
    #[inline(always)]
    pub const fn tim14_ccmr1(&self) -> &TIM14_CCMR1 {
        &self.tim14_ccmr1
    }
    #[doc = "0x20 - TIM14 capture/compare enable register"]
    #[inline(always)]
    pub const fn tim14_ccer(&self) -> &TIM14_CCER {
        &self.tim14_ccer
    }
    #[doc = "0x24 - TIM14 counter"]
    #[inline(always)]
    pub const fn tim14_cnt(&self) -> &TIM14_CNT {
        &self.tim14_cnt
    }
    #[doc = "0x28 - TIM14 prescaler"]
    #[inline(always)]
    pub const fn tim14_psc(&self) -> &TIM14_PSC {
        &self.tim14_psc
    }
    #[doc = "0x2c - TIM14 auto-reload register"]
    #[inline(always)]
    pub const fn tim14_arr(&self) -> &TIM14_ARR {
        &self.tim14_arr
    }
    #[doc = "0x34 - TIM14 capture/compare register 1"]
    #[inline(always)]
    pub const fn tim14_ccr1(&self) -> &TIM14_CCR1 {
        &self.tim14_ccr1
    }
    #[doc = "0x68 - TIM14 timer input selection register"]
    #[inline(always)]
    pub const fn tim14_tisel(&self) -> &TIM14_TISEL {
        &self.tim14_tisel
    }
}
#[doc = "TIM14_CR1 (rw) register accessor: TIM14 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_cr1`]
module"]
pub type TIM14_CR1 = crate::Reg<tim14_cr1::TIM14_CR1rs>;
#[doc = "TIM14 control register 1"]
pub mod tim14_cr1;
#[doc = "TIM14_DIER (rw) register accessor: TIM14 Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_dier`]
module"]
pub type TIM14_DIER = crate::Reg<tim14_dier::TIM14_DIERrs>;
#[doc = "TIM14 Interrupt enable register"]
pub mod tim14_dier;
#[doc = "TIM14_SR (rw) register accessor: TIM14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_sr`]
module"]
pub type TIM14_SR = crate::Reg<tim14_sr::TIM14_SRrs>;
#[doc = "TIM14 status register"]
pub mod tim14_sr;
#[doc = "TIM14_EGR (w) register accessor: TIM14 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_egr`]
module"]
pub type TIM14_EGR = crate::Reg<tim14_egr::TIM14_EGRrs>;
#[doc = "TIM14 event generation register"]
pub mod tim14_egr;
#[doc = "TIM14_CCMR1 (rw) register accessor: The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_ccmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_ccmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_ccmr1`]
module"]
pub type TIM14_CCMR1 = crate::Reg<tim14_ccmr1::TIM14_CCMR1rs>;
#[doc = "The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode"]
pub mod tim14_ccmr1;
#[doc = "TIM14_CCER (rw) register accessor: TIM14 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_ccer`]
module"]
pub type TIM14_CCER = crate::Reg<tim14_ccer::TIM14_CCERrs>;
#[doc = "TIM14 capture/compare enable register"]
pub mod tim14_ccer;
#[doc = "TIM14_CNT (rw) register accessor: TIM14 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_cnt`]
module"]
pub type TIM14_CNT = crate::Reg<tim14_cnt::TIM14_CNTrs>;
#[doc = "TIM14 counter"]
pub mod tim14_cnt;
#[doc = "TIM14_PSC (rw) register accessor: TIM14 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_psc`]
module"]
pub type TIM14_PSC = crate::Reg<tim14_psc::TIM14_PSCrs>;
#[doc = "TIM14 prescaler"]
pub mod tim14_psc;
#[doc = "TIM14_ARR (rw) register accessor: TIM14 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_arr`]
module"]
pub type TIM14_ARR = crate::Reg<tim14_arr::TIM14_ARRrs>;
#[doc = "TIM14 auto-reload register"]
pub mod tim14_arr;
#[doc = "TIM14_CCR1 (rw) register accessor: TIM14 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_ccr1`]
module"]
pub type TIM14_CCR1 = crate::Reg<tim14_ccr1::TIM14_CCR1rs>;
#[doc = "TIM14 capture/compare register 1"]
pub mod tim14_ccr1;
#[doc = "TIM14_TISEL (rw) register accessor: TIM14 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_tisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_tisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim14_tisel`]
module"]
pub type TIM14_TISEL = crate::Reg<tim14_tisel::TIM14_TISELrs>;
#[doc = "TIM14 timer input selection register"]
pub mod tim14_tisel;
