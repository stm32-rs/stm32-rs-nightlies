#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x0a],
    dier: DIER,
    _reserved2: [u8; 0x02],
    sr: SR,
    _reserved3: [u8; 0x02],
    egr: EGR,
    _reserved4: [u8; 0x02],
    _reserved_4_ccmr1: [u8; 0x04],
    _reserved5: [u8; 0x04],
    ccer: CCER,
    _reserved6: [u8; 0x02],
    cnt: CNT,
    psc: PSC,
    _reserved8: [u8; 0x02],
    arr: ARR,
    _reserved9: [u8; 0x06],
    ccr1: CCR1,
    _reserved10: [u8; 0x32],
    tisel: TISEL,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM14 control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x0c - TIM14 Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    #[doc = "0x10 - TIM14 status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x14 - TIM14 event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    #[doc = "0x18 - TIM14 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - TIM14 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - TIM14 capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    #[doc = "0x24 - TIM14 counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x28 - TIM14 prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    #[doc = "0x2c - TIM14 auto-reload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    #[doc = "0x34 - TIM14 capture/compare register 1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    #[doc = "0x68 - TIM14 timer input selection register"]
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
}
#[doc = "CR1 (rw) register accessor: TIM14 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "TIM14 control register 1"]
pub mod cr1;
#[doc = "DIER (rw) register accessor: TIM14 Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`]
module"]
pub type DIER = crate::Reg<dier::DIERrs>;
#[doc = "TIM14 Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: TIM14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "TIM14 status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: TIM14 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`]
module"]
pub type EGR = crate::Reg<egr::EGRrs>;
#[doc = "TIM14 event generation register"]
pub mod egr;
#[doc = "CCMR1_input (rw) register accessor: TIM14 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_input`]
module"]
#[doc(alias = "CCMR1_input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
#[doc = "TIM14 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_input;
#[doc = "CCMR1_output (rw) register accessor: TIM14 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_output`]
module"]
#[doc(alias = "CCMR1_output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
#[doc = "TIM14 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_output;
#[doc = "CCER (rw) register accessor: TIM14 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`]
module"]
pub type CCER = crate::Reg<ccer::CCERrs>;
#[doc = "TIM14 capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: TIM14 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNTrs>;
#[doc = "TIM14 counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: TIM14 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
pub type PSC = crate::Reg<psc::PSCrs>;
#[doc = "TIM14 prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: TIM14 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::Reg<arr::ARRrs>;
#[doc = "TIM14 auto-reload register"]
pub mod arr;
#[doc = "CCR1 (rw) register accessor: TIM14 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
#[doc = "TIM14 capture/compare register 1"]
pub mod ccr1;
#[doc = "TISEL (rw) register accessor: TIM14 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisel`]
module"]
pub type TISEL = crate::Reg<tisel::TISELrs>;
#[doc = "TIM14 timer input selection register"]
pub mod tisel;
