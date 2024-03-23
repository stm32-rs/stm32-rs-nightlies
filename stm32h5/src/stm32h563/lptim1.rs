#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_isr: [u8; 0x04],
    _reserved_1_icr: [u8; 0x04],
    _reserved_2_dier: [u8; 0x04],
    cfgr: CFGR,
    cr: CR,
    ccr1: CCR1,
    arr: ARR,
    cnt: CNT,
    _reserved8: [u8; 0x04],
    cfgr2: CFGR2,
    rcr: RCR,
    ccmr1: CCMR1,
    _reserved11: [u8; 0x04],
    ccr2: CCR2,
}
impl RegisterBlock {
    #[doc = "0x00 - LPTIM interrupt and status register"]
    #[inline(always)]
    pub const fn isr_intput(&self) -> &ISR_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - LPTIM interrupt and status register"]
    #[inline(always)]
    pub const fn isr_output(&self) -> &ISR_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - LPTIM interrupt clear register"]
    #[inline(always)]
    pub const fn icr_intput(&self) -> &ICR_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - LPTIM interrupt clear register"]
    #[inline(always)]
    pub const fn icr_output(&self) -> &ICR_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - LPTIM interrupt enable register"]
    #[inline(always)]
    pub const fn dier_intput(&self) -> &DIER_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - LPTIM interrupt enable register"]
    #[inline(always)]
    pub const fn dier_output(&self) -> &DIER_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - LPTIM configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x10 - LPTIM control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x14 - LPTIM compare register 1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    #[doc = "0x18 - LPTIM autoreload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    #[doc = "0x1c - LPTIM counter register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x24 - LPTIM configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x28 - LPTIM repetition register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    #[doc = "0x2c - LPTIM capture/compare mode register 1"]
    #[inline(always)]
    pub const fn ccmr1(&self) -> &CCMR1 {
        &self.ccmr1
    }
    #[doc = "0x34 - LPTIM compare register 2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
}
#[doc = "ISR_output (r) register accessor: LPTIM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr_output::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr_output`]
module"]
#[doc(alias = "ISR_output")]
pub type ISR_OUTPUT = crate::Reg<isr_output::ISR_OUTPUTrs>;
#[doc = "LPTIM interrupt and status register"]
pub mod isr_output;
#[doc = "ISR_intput (r) register accessor: LPTIM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr_intput::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr_intput`]
module"]
#[doc(alias = "ISR_intput")]
pub type ISR_INTPUT = crate::Reg<isr_intput::ISR_INTPUTrs>;
#[doc = "LPTIM interrupt and status register"]
pub mod isr_intput;
#[doc = "ICR_output (w) register accessor: LPTIM interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr_output`]
module"]
#[doc(alias = "ICR_output")]
pub type ICR_OUTPUT = crate::Reg<icr_output::ICR_OUTPUTrs>;
#[doc = "LPTIM interrupt clear register"]
pub mod icr_output;
#[doc = "ICR_intput (w) register accessor: LPTIM interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr_intput::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr_intput`]
module"]
#[doc(alias = "ICR_intput")]
pub type ICR_INTPUT = crate::Reg<icr_intput::ICR_INTPUTrs>;
#[doc = "LPTIM interrupt clear register"]
pub mod icr_intput;
#[doc = "DIER_output (rw) register accessor: LPTIM interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier_output`]
module"]
#[doc(alias = "DIER_output")]
pub type DIER_OUTPUT = crate::Reg<dier_output::DIER_OUTPUTrs>;
#[doc = "LPTIM interrupt enable register"]
pub mod dier_output;
#[doc = "DIER_intput (rw) register accessor: LPTIM interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier_intput::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier_intput::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier_intput`]
module"]
#[doc(alias = "DIER_intput")]
pub type DIER_INTPUT = crate::Reg<dier_intput::DIER_INTPUTrs>;
#[doc = "LPTIM interrupt enable register"]
pub mod dier_intput;
#[doc = "CFGR (rw) register accessor: LPTIM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "LPTIM configuration register"]
pub mod cfgr;
#[doc = "CR (rw) register accessor: LPTIM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "LPTIM control register"]
pub mod cr;
#[doc = "CCR1 (rw) register accessor: LPTIM compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
#[doc = "LPTIM compare register 1"]
pub mod ccr1;
#[doc = "ARR (rw) register accessor: LPTIM autoreload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::Reg<arr::ARRrs>;
#[doc = "LPTIM autoreload register"]
pub mod arr;
#[doc = "CNT (r) register accessor: LPTIM counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNTrs>;
#[doc = "LPTIM counter register"]
pub mod cnt;
#[doc = "CFGR2 (rw) register accessor: LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "LPTIM configuration register 2"]
pub mod cfgr2;
#[doc = "RCR (rw) register accessor: LPTIM repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCRrs>;
#[doc = "LPTIM repetition register"]
pub mod rcr;
#[doc = "CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1`]
module"]
pub type CCMR1 = crate::Reg<ccmr1::CCMR1rs>;
#[doc = "LPTIM capture/compare mode register 1"]
pub mod ccmr1;
#[doc = "CCR2 (rw) register accessor: LPTIM compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
#[doc = "LPTIM compare register 2"]
pub mod ccr2;
