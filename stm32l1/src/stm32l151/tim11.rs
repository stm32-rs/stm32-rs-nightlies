#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x04],
    smcr: SMCR,
    dier: DIER,
    sr: SR,
    egr: EGR,
    _reserved_5_ccmr1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    ccer: CCER,
    cnt: CNT,
    psc: PSC,
    arr: ARR,
    _reserved10: [u8; 0x04],
    ccr: [CCR; 1],
    _reserved11: [u8; 0x18],
    or: OR,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x08 - TIM 11 slave mode control register 1"]
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    #[doc = "0x0c - Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    #[doc = "0x10 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x14 - event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture/compare mode register"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    #[doc = "0x24 - counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x28 - prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    #[doc = "0x2c - auto-reload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    #[doc = "0x34 - capture/compare register"]
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34 - capture/compare register"]
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    #[doc = "0x34 - capture/compare register"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR {
        self.ccr(0)
    }
    #[doc = "0x50 - option register"]
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
}
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "DIER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`]
module"]
pub type DIER = crate::Reg<dier::DIERrs>;
#[doc = "Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`]
module"]
pub type EGR = crate::Reg<egr::EGRrs>;
#[doc = "event generation register"]
pub mod egr;
#[doc = "CCMR1_Output (rw) register accessor: capture/compare mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_output`]
module"]
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
#[doc = "capture/compare mode register"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input (rw) register accessor: capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_input`]
module"]
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "CCER (rw) register accessor: capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`]
module"]
pub type CCER = crate::Reg<ccer::CCERrs>;
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNTrs>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
pub type PSC = crate::Reg<psc::PSCrs>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::Reg<arr::ARRrs>;
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "CCR (rw) register accessor: capture/compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "capture/compare register"]
pub mod ccr;
#[doc = "OR (rw) register accessor: option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`]
module"]
pub type OR = crate::Reg<or::ORrs>;
#[doc = "option register"]
pub mod or;
#[doc = "SMCR (rw) register accessor: TIM 11 slave mode control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`]
module"]
pub type SMCR = crate::Reg<smcr::SMCRrs>;
#[doc = "TIM 11 slave mode control register 1"]
pub mod smcr;
