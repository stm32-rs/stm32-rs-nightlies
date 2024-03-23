#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: ISR,
    icr: ICR,
    dier: DIER,
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
    _reserved12: [u8; 0x03b4],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt and Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x08 - LPTIM interrupt Enable Register"]
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    #[doc = "0x0c - Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x10 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x14 - Compare Register"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    #[doc = "0x18 - Autoreload Register"]
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    #[doc = "0x1c - Counter Register"]
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
    #[doc = "0x34 - LPTIM Compare Register 2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    #[doc = "0x3ec - LPTIM peripheral hardware configuration register 2"]
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    #[doc = "0x3f0 - LPTIM peripheral hardware configuration register 1"]
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
}
#[doc = "ISR (r) register accessor: Interrupt and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "Interrupt and Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "DIER (rw) register accessor: LPTIM interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`]
module"]
pub type DIER = crate::Reg<dier::DIERrs>;
#[doc = "LPTIM interrupt Enable Register"]
pub mod dier;
#[doc = "CFGR (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "Configuration Register"]
pub mod cfgr;
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CCR1 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
#[doc = "Compare Register"]
pub mod ccr1;
#[doc = "ARR (rw) register accessor: Autoreload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`]
module"]
pub type ARR = crate::Reg<arr::ARRrs>;
#[doc = "Autoreload Register"]
pub mod arr;
#[doc = "CNT (r) register accessor: Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNTrs>;
#[doc = "Counter Register"]
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
#[doc = "CCR2 (rw) register accessor: LPTIM Compare Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
#[doc = "LPTIM Compare Register 2"]
pub mod ccr2;
#[doc = "HWCFGR2 (r) register accessor: LPTIM peripheral hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr2`]
module"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
#[doc = "LPTIM peripheral hardware configuration register 2"]
pub mod hwcfgr2;
#[doc = "HWCFGR1 (r) register accessor: LPTIM peripheral hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr1`]
module"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
#[doc = "LPTIM peripheral hardware configuration register 1"]
pub mod hwcfgr1;
