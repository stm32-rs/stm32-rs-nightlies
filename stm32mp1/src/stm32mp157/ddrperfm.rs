#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ddrperfm_ctl: DDRPERFM_CTL,
    ddrperfm_cfg: DDRPERFM_CFG,
    ddrperfm_status: DDRPERFM_STATUS,
    ddrperfm_ccr: DDRPERFM_CCR,
    ddrperfm_ier: DDRPERFM_IER,
    ddrperfm_isr: DDRPERFM_ISR,
    ddrperfm_icr: DDRPERFM_ICR,
    _reserved7: [u8; 0x04],
    ddrperfm_tcnt: DDRPERFM_TCNT,
    _reserved8: [u8; 0x3c],
    ddrperfm_cnt0: DDRPERFM_CNT0,
    _reserved9: [u8; 0x04],
    ddrperfm_cnt1: DDRPERFM_CNT1,
    _reserved10: [u8; 0x04],
    ddrperfm_cnt2: DDRPERFM_CNT2,
    _reserved11: [u8; 0x04],
    ddrperfm_cnt3: DDRPERFM_CNT3,
    _reserved12: [u8; 0x0374],
    ddrperfm_hwcfg: DDRPERFM_HWCFG,
    ddrperfm_ver: DDRPERFM_VER,
    ddrperfm_id: DDRPERFM_ID,
    ddrperfm_sid: DDRPERFM_SID,
}
impl RegisterBlock {
    #[doc = "0x00 - Write-only register. A read request returns all zeros."]
    #[inline(always)]
    pub const fn ddrperfm_ctl(&self) -> &DDRPERFM_CTL {
        &self.ddrperfm_ctl
    }
    #[doc = "0x04 - DDRPERFM configurationl register"]
    #[inline(always)]
    pub const fn ddrperfm_cfg(&self) -> &DDRPERFM_CFG {
        &self.ddrperfm_cfg
    }
    #[doc = "0x08 - DDRPERFM status register"]
    #[inline(always)]
    pub const fn ddrperfm_status(&self) -> &DDRPERFM_STATUS {
        &self.ddrperfm_status
    }
    #[doc = "0x0c - Write-only register. A read request returns all zeros"]
    #[inline(always)]
    pub const fn ddrperfm_ccr(&self) -> &DDRPERFM_CCR {
        &self.ddrperfm_ccr
    }
    #[doc = "0x10 - DDRPERFM interrupt enable register"]
    #[inline(always)]
    pub const fn ddrperfm_ier(&self) -> &DDRPERFM_IER {
        &self.ddrperfm_ier
    }
    #[doc = "0x14 - DDRPERFM interrupt status register"]
    #[inline(always)]
    pub const fn ddrperfm_isr(&self) -> &DDRPERFM_ISR {
        &self.ddrperfm_isr
    }
    #[doc = "0x18 - Write-only register. A read request returns all zeros"]
    #[inline(always)]
    pub const fn ddrperfm_icr(&self) -> &DDRPERFM_ICR {
        &self.ddrperfm_icr
    }
    #[doc = "0x20 - DDRPERFM time counter register"]
    #[inline(always)]
    pub const fn ddrperfm_tcnt(&self) -> &DDRPERFM_TCNT {
        &self.ddrperfm_tcnt
    }
    #[doc = "0x60 - DDRPERFM event counter 0 register"]
    #[inline(always)]
    pub const fn ddrperfm_cnt0(&self) -> &DDRPERFM_CNT0 {
        &self.ddrperfm_cnt0
    }
    #[doc = "0x68 - DDRPERFM event counter 1 register"]
    #[inline(always)]
    pub const fn ddrperfm_cnt1(&self) -> &DDRPERFM_CNT1 {
        &self.ddrperfm_cnt1
    }
    #[doc = "0x70 - DDRPERFM event counter 2 register"]
    #[inline(always)]
    pub const fn ddrperfm_cnt2(&self) -> &DDRPERFM_CNT2 {
        &self.ddrperfm_cnt2
    }
    #[doc = "0x78 - DDRPERFM event counter 3 register"]
    #[inline(always)]
    pub const fn ddrperfm_cnt3(&self) -> &DDRPERFM_CNT3 {
        &self.ddrperfm_cnt3
    }
    #[doc = "0x3f0 - DDRPERFM hardware configuration register"]
    #[inline(always)]
    pub const fn ddrperfm_hwcfg(&self) -> &DDRPERFM_HWCFG {
        &self.ddrperfm_hwcfg
    }
    #[doc = "0x3f4 - DDRPERFM version register"]
    #[inline(always)]
    pub const fn ddrperfm_ver(&self) -> &DDRPERFM_VER {
        &self.ddrperfm_ver
    }
    #[doc = "0x3f8 - DDRPERFM ID register"]
    #[inline(always)]
    pub const fn ddrperfm_id(&self) -> &DDRPERFM_ID {
        &self.ddrperfm_id
    }
    #[doc = "0x3fc - DDRPERFM magic ID register"]
    #[inline(always)]
    pub const fn ddrperfm_sid(&self) -> &DDRPERFM_SID {
        &self.ddrperfm_sid
    }
}
#[doc = "DDRPERFM_CTL (w) register accessor: Write-only register. A read request returns all zeros.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_ctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_ctl`]
module"]
pub type DDRPERFM_CTL = crate::Reg<ddrperfm_ctl::DDRPERFM_CTLrs>;
#[doc = "Write-only register. A read request returns all zeros."]
pub mod ddrperfm_ctl;
#[doc = "DDRPERFM_CFG (rw) register accessor: DDRPERFM configurationl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_cfg`]
module"]
pub type DDRPERFM_CFG = crate::Reg<ddrperfm_cfg::DDRPERFM_CFGrs>;
#[doc = "DDRPERFM configurationl register"]
pub mod ddrperfm_cfg;
#[doc = "DDRPERFM_STATUS (r) register accessor: DDRPERFM status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_status`]
module"]
pub type DDRPERFM_STATUS = crate::Reg<ddrperfm_status::DDRPERFM_STATUSrs>;
#[doc = "DDRPERFM status register"]
pub mod ddrperfm_status;
#[doc = "DDRPERFM_CCR (w) register accessor: Write-only register. A read request returns all zeros\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_ccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_ccr`]
module"]
pub type DDRPERFM_CCR = crate::Reg<ddrperfm_ccr::DDRPERFM_CCRrs>;
#[doc = "Write-only register. A read request returns all zeros"]
pub mod ddrperfm_ccr;
#[doc = "DDRPERFM_IER (rw) register accessor: DDRPERFM interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_ier`]
module"]
pub type DDRPERFM_IER = crate::Reg<ddrperfm_ier::DDRPERFM_IERrs>;
#[doc = "DDRPERFM interrupt enable register"]
pub mod ddrperfm_ier;
#[doc = "DDRPERFM_ISR (r) register accessor: DDRPERFM interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_isr`]
module"]
pub type DDRPERFM_ISR = crate::Reg<ddrperfm_isr::DDRPERFM_ISRrs>;
#[doc = "DDRPERFM interrupt status register"]
pub mod ddrperfm_isr;
#[doc = "DDRPERFM_ICR (w) register accessor: Write-only register. A read request returns all zeros\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_icr`]
module"]
pub type DDRPERFM_ICR = crate::Reg<ddrperfm_icr::DDRPERFM_ICRrs>;
#[doc = "Write-only register. A read request returns all zeros"]
pub mod ddrperfm_icr;
#[doc = "DDRPERFM_TCNT (r) register accessor: DDRPERFM time counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_tcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_tcnt`]
module"]
pub type DDRPERFM_TCNT = crate::Reg<ddrperfm_tcnt::DDRPERFM_TCNTrs>;
#[doc = "DDRPERFM time counter register"]
pub mod ddrperfm_tcnt;
#[doc = "DDRPERFM_CNT0 (r) register accessor: DDRPERFM event counter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_cnt0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_cnt0`]
module"]
pub type DDRPERFM_CNT0 = crate::Reg<ddrperfm_cnt0::DDRPERFM_CNT0rs>;
#[doc = "DDRPERFM event counter 0 register"]
pub mod ddrperfm_cnt0;
#[doc = "DDRPERFM_CNT1 (r) register accessor: DDRPERFM event counter 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_cnt1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_cnt1`]
module"]
pub type DDRPERFM_CNT1 = crate::Reg<ddrperfm_cnt1::DDRPERFM_CNT1rs>;
#[doc = "DDRPERFM event counter 1 register"]
pub mod ddrperfm_cnt1;
#[doc = "DDRPERFM_CNT2 (r) register accessor: DDRPERFM event counter 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_cnt2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_cnt2`]
module"]
pub type DDRPERFM_CNT2 = crate::Reg<ddrperfm_cnt2::DDRPERFM_CNT2rs>;
#[doc = "DDRPERFM event counter 2 register"]
pub mod ddrperfm_cnt2;
#[doc = "DDRPERFM_CNT3 (r) register accessor: DDRPERFM event counter 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_cnt3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_cnt3`]
module"]
pub type DDRPERFM_CNT3 = crate::Reg<ddrperfm_cnt3::DDRPERFM_CNT3rs>;
#[doc = "DDRPERFM event counter 3 register"]
pub mod ddrperfm_cnt3;
#[doc = "DDRPERFM_HWCFG (r) register accessor: DDRPERFM hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_hwcfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_hwcfg`]
module"]
pub type DDRPERFM_HWCFG = crate::Reg<ddrperfm_hwcfg::DDRPERFM_HWCFGrs>;
#[doc = "DDRPERFM hardware configuration register"]
pub mod ddrperfm_hwcfg;
#[doc = "DDRPERFM_VER (r) register accessor: DDRPERFM version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_ver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_ver`]
module"]
pub type DDRPERFM_VER = crate::Reg<ddrperfm_ver::DDRPERFM_VERrs>;
#[doc = "DDRPERFM version register"]
pub mod ddrperfm_ver;
#[doc = "DDRPERFM_ID (r) register accessor: DDRPERFM ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_id`]
module"]
pub type DDRPERFM_ID = crate::Reg<ddrperfm_id::DDRPERFM_IDrs>;
#[doc = "DDRPERFM ID register"]
pub mod ddrperfm_id;
#[doc = "DDRPERFM_SID (r) register accessor: DDRPERFM magic ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_sid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrperfm_sid`]
module"]
pub type DDRPERFM_SID = crate::Reg<ddrperfm_sid::DDRPERFM_SIDrs>;
#[doc = "DDRPERFM magic ID register"]
pub mod ddrperfm_sid;
