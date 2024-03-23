#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lptim_isr: LPTIM_ISR,
    lptim_icr: LPTIM_ICR,
    lptim_ier: LPTIM_IER,
    lptim_cfgr: LPTIM_CFGR,
    lptim_cr: LPTIM_CR,
    lptim_cmp: LPTIM_CMP,
    lptim_arr: LPTIM_ARR,
    lptim_cnt: LPTIM_CNT,
    _reserved8: [u8; 0x04],
    lptim_cfgr2: LPTIM_CFGR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt and Status Register"]
    #[inline(always)]
    pub const fn lptim_isr(&self) -> &LPTIM_ISR {
        &self.lptim_isr
    }
    #[doc = "0x04 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn lptim_icr(&self) -> &LPTIM_ICR {
        &self.lptim_icr
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn lptim_ier(&self) -> &LPTIM_IER {
        &self.lptim_ier
    }
    #[doc = "0x0c - Configuration Register"]
    #[inline(always)]
    pub const fn lptim_cfgr(&self) -> &LPTIM_CFGR {
        &self.lptim_cfgr
    }
    #[doc = "0x10 - Control Register"]
    #[inline(always)]
    pub const fn lptim_cr(&self) -> &LPTIM_CR {
        &self.lptim_cr
    }
    #[doc = "0x14 - Compare Register"]
    #[inline(always)]
    pub const fn lptim_cmp(&self) -> &LPTIM_CMP {
        &self.lptim_cmp
    }
    #[doc = "0x18 - Autoreload Register"]
    #[inline(always)]
    pub const fn lptim_arr(&self) -> &LPTIM_ARR {
        &self.lptim_arr
    }
    #[doc = "0x1c - Counter Register"]
    #[inline(always)]
    pub const fn lptim_cnt(&self) -> &LPTIM_CNT {
        &self.lptim_cnt
    }
    #[doc = "0x24 - LPTIM configuration register 2"]
    #[inline(always)]
    pub const fn lptim_cfgr2(&self) -> &LPTIM_CFGR2 {
        &self.lptim_cfgr2
    }
}
#[doc = "LPTIM_ISR (r) register accessor: Interrupt and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_isr`]
module"]
pub type LPTIM_ISR = crate::Reg<lptim_isr::LPTIM_ISRrs>;
#[doc = "Interrupt and Status Register"]
pub mod lptim_isr;
#[doc = "LPTIM_ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_icr`]
module"]
pub type LPTIM_ICR = crate::Reg<lptim_icr::LPTIM_ICRrs>;
#[doc = "Interrupt Clear Register"]
pub mod lptim_icr;
#[doc = "LPTIM_IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_ier`]
module"]
pub type LPTIM_IER = crate::Reg<lptim_ier::LPTIM_IERrs>;
#[doc = "Interrupt Enable Register"]
pub mod lptim_ier;
#[doc = "LPTIM_CFGR (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cfgr`]
module"]
pub type LPTIM_CFGR = crate::Reg<lptim_cfgr::LPTIM_CFGRrs>;
#[doc = "Configuration Register"]
pub mod lptim_cfgr;
#[doc = "LPTIM_CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cr`]
module"]
pub type LPTIM_CR = crate::Reg<lptim_cr::LPTIM_CRrs>;
#[doc = "Control Register"]
pub mod lptim_cr;
#[doc = "LPTIM_CMP (rw) register accessor: Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cmp`]
module"]
pub type LPTIM_CMP = crate::Reg<lptim_cmp::LPTIM_CMPrs>;
#[doc = "Compare Register"]
pub mod lptim_cmp;
#[doc = "LPTIM_ARR (rw) register accessor: Autoreload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_arr`]
module"]
pub type LPTIM_ARR = crate::Reg<lptim_arr::LPTIM_ARRrs>;
#[doc = "Autoreload Register"]
pub mod lptim_arr;
#[doc = "LPTIM_CNT (r) register accessor: Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cnt`]
module"]
pub type LPTIM_CNT = crate::Reg<lptim_cnt::LPTIM_CNTrs>;
#[doc = "Counter Register"]
pub mod lptim_cnt;
#[doc = "LPTIM_CFGR2 (rw) register accessor: LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cfgr2`]
module"]
pub type LPTIM_CFGR2 = crate::Reg<lptim_cfgr2::LPTIM_CFGR2rs>;
#[doc = "LPTIM configuration register 2"]
pub mod lptim_cfgr2;
