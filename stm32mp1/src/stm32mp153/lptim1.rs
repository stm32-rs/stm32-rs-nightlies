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
    _reserved9: [u8; 0x03c8],
    lptim1_hwcfgr: LPTIM1_HWCFGR,
    lptim_verr: LPTIM_VERR,
    lptim_pidr: LPTIM_PIDR,
    lptim_sidr: LPTIM_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - LPTIM interrupt and status register"]
    #[inline(always)]
    pub const fn lptim_isr(&self) -> &LPTIM_ISR {
        &self.lptim_isr
    }
    #[doc = "0x04 - LPTIM interrupt clear register"]
    #[inline(always)]
    pub const fn lptim_icr(&self) -> &LPTIM_ICR {
        &self.lptim_icr
    }
    #[doc = "0x08 - LPTIM interrupt enable register"]
    #[inline(always)]
    pub const fn lptim_ier(&self) -> &LPTIM_IER {
        &self.lptim_ier
    }
    #[doc = "0x0c - LPTIM configuration register"]
    #[inline(always)]
    pub const fn lptim_cfgr(&self) -> &LPTIM_CFGR {
        &self.lptim_cfgr
    }
    #[doc = "0x10 - LPTIM control register"]
    #[inline(always)]
    pub const fn lptim_cr(&self) -> &LPTIM_CR {
        &self.lptim_cr
    }
    #[doc = "0x14 - LPTIM compare register"]
    #[inline(always)]
    pub const fn lptim_cmp(&self) -> &LPTIM_CMP {
        &self.lptim_cmp
    }
    #[doc = "0x18 - LPTIM autoreload register"]
    #[inline(always)]
    pub const fn lptim_arr(&self) -> &LPTIM_ARR {
        &self.lptim_arr
    }
    #[doc = "0x1c - LPTIM counter register"]
    #[inline(always)]
    pub const fn lptim_cnt(&self) -> &LPTIM_CNT {
        &self.lptim_cnt
    }
    #[doc = "0x24 - LPTIM configuration register 2"]
    #[inline(always)]
    pub const fn lptim_cfgr2(&self) -> &LPTIM_CFGR2 {
        &self.lptim_cfgr2
    }
    #[doc = "0x3f0 - LPTIM 1 peripheral hardware configuration register"]
    #[inline(always)]
    pub const fn lptim1_hwcfgr(&self) -> &LPTIM1_HWCFGR {
        &self.lptim1_hwcfgr
    }
    #[doc = "0x3f4 - LPTIM peripheral version identification register"]
    #[inline(always)]
    pub const fn lptim_verr(&self) -> &LPTIM_VERR {
        &self.lptim_verr
    }
    #[doc = "0x3f8 - LPTIM peripheral type identification register"]
    #[inline(always)]
    pub const fn lptim_pidr(&self) -> &LPTIM_PIDR {
        &self.lptim_pidr
    }
    #[doc = "0x3fc - LPTIM registers map size identification register"]
    #[inline(always)]
    pub const fn lptim_sidr(&self) -> &LPTIM_SIDR {
        &self.lptim_sidr
    }
}
#[doc = "LPTIM_ISR (r) register accessor: LPTIM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_isr`]
module"]
pub type LPTIM_ISR = crate::Reg<lptim_isr::LPTIM_ISRrs>;
#[doc = "LPTIM interrupt and status register"]
pub mod lptim_isr;
#[doc = "LPTIM_ICR (w) register accessor: LPTIM interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_icr`]
module"]
pub type LPTIM_ICR = crate::Reg<lptim_icr::LPTIM_ICRrs>;
#[doc = "LPTIM interrupt clear register"]
pub mod lptim_icr;
#[doc = "LPTIM_IER (rw) register accessor: LPTIM interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_ier`]
module"]
pub type LPTIM_IER = crate::Reg<lptim_ier::LPTIM_IERrs>;
#[doc = "LPTIM interrupt enable register"]
pub mod lptim_ier;
#[doc = "LPTIM_CFGR (rw) register accessor: LPTIM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cfgr`]
module"]
pub type LPTIM_CFGR = crate::Reg<lptim_cfgr::LPTIM_CFGRrs>;
#[doc = "LPTIM configuration register"]
pub mod lptim_cfgr;
#[doc = "LPTIM_CR (rw) register accessor: LPTIM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cr`]
module"]
pub type LPTIM_CR = crate::Reg<lptim_cr::LPTIM_CRrs>;
#[doc = "LPTIM control register"]
pub mod lptim_cr;
#[doc = "LPTIM_CMP (rw) register accessor: LPTIM compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cmp`]
module"]
pub type LPTIM_CMP = crate::Reg<lptim_cmp::LPTIM_CMPrs>;
#[doc = "LPTIM compare register"]
pub mod lptim_cmp;
#[doc = "LPTIM_ARR (rw) register accessor: LPTIM autoreload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_arr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_arr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_arr`]
module"]
pub type LPTIM_ARR = crate::Reg<lptim_arr::LPTIM_ARRrs>;
#[doc = "LPTIM autoreload register"]
pub mod lptim_arr;
#[doc = "LPTIM_CNT (r) register accessor: LPTIM counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cnt`]
module"]
pub type LPTIM_CNT = crate::Reg<lptim_cnt::LPTIM_CNTrs>;
#[doc = "LPTIM counter register"]
pub mod lptim_cnt;
#[doc = "LPTIM_CFGR2 (rw) register accessor: LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_cfgr2`]
module"]
pub type LPTIM_CFGR2 = crate::Reg<lptim_cfgr2::LPTIM_CFGR2rs>;
#[doc = "LPTIM configuration register 2"]
pub mod lptim_cfgr2;
#[doc = "LPTIM1_HWCFGR (r) register accessor: LPTIM 1 peripheral hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim1_hwcfgr`]
module"]
pub type LPTIM1_HWCFGR = crate::Reg<lptim1_hwcfgr::LPTIM1_HWCFGRrs>;
#[doc = "LPTIM 1 peripheral hardware configuration register"]
pub mod lptim1_hwcfgr;
#[doc = "LPTIM_VERR (r) register accessor: LPTIM peripheral version identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_verr`]
module"]
pub type LPTIM_VERR = crate::Reg<lptim_verr::LPTIM_VERRrs>;
#[doc = "LPTIM peripheral version identification register"]
pub mod lptim_verr;
#[doc = "LPTIM_PIDR (r) register accessor: LPTIM peripheral type identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_pidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_pidr`]
module"]
pub type LPTIM_PIDR = crate::Reg<lptim_pidr::LPTIM_PIDRrs>;
#[doc = "LPTIM peripheral type identification register"]
pub mod lptim_pidr;
#[doc = "LPTIM_SIDR (r) register accessor: LPTIM registers map size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptim_sidr`]
module"]
pub type LPTIM_SIDR = crate::Reg<lptim_sidr::LPTIM_SIDRrs>;
#[doc = "LPTIM registers map size identification register"]
pub mod lptim_sidr;
