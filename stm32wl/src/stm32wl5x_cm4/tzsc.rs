#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    seccfgr1: SECCFGR1,
    _reserved2: [u8; 0x0c],
    privcfgr1: PRIVCFGR1,
    _reserved3: [u8; 0x010c],
    mpcwm1_upwmr: MPCWM1_UPWMR,
    mpcwm1_upwwmr: MPCWM1_UPWWMR,
    mpcwm2_upwmr: MPCWM2_UPWMR,
    _reserved6: [u8; 0x04],
    mpcwm3_upwmr: MPCWM3_UPWMR,
}
impl RegisterBlock {
    #[doc = "0x00 - TZSC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x10 - TZSC security configuration register"]
    #[inline(always)]
    pub const fn seccfgr1(&self) -> &SECCFGR1 {
        &self.seccfgr1
    }
    #[doc = "0x20 - TZSC privilege configuration register 1"]
    #[inline(always)]
    pub const fn privcfgr1(&self) -> &PRIVCFGR1 {
        &self.privcfgr1
    }
    #[doc = "0x130 - Unprivileged Water Mark 1 register"]
    #[inline(always)]
    pub const fn mpcwm1_upwmr(&self) -> &MPCWM1_UPWMR {
        &self.mpcwm1_upwmr
    }
    #[doc = "0x134 - Unprivileged Writable Water Mark 1 register"]
    #[inline(always)]
    pub const fn mpcwm1_upwwmr(&self) -> &MPCWM1_UPWWMR {
        &self.mpcwm1_upwwmr
    }
    #[doc = "0x138 - Unprivileged Water Mark 2 register"]
    #[inline(always)]
    pub const fn mpcwm2_upwmr(&self) -> &MPCWM2_UPWMR {
        &self.mpcwm2_upwmr
    }
    #[doc = "0x140 - Unprivileged Water Mark 3 register"]
    #[inline(always)]
    pub const fn mpcwm3_upwmr(&self) -> &MPCWM3_UPWMR {
        &self.mpcwm3_upwmr
    }
}
#[doc = "CR (rw) register accessor: TZSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "TZSC control register"]
pub mod cr;
#[doc = "SECCFGR1 (rw) register accessor: TZSC security configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr1`]
module"]
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1rs>;
#[doc = "TZSC security configuration register"]
pub mod seccfgr1;
#[doc = "PRIVCFGR1 (rw) register accessor: TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr1`]
module"]
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1rs>;
#[doc = "TZSC privilege configuration register 1"]
pub mod privcfgr1;
#[doc = "MPCWM1_UPWMR (rw) register accessor: Unprivileged Water Mark 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm1_upwmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm1_upwmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm1_upwmr`]
module"]
pub type MPCWM1_UPWMR = crate::Reg<mpcwm1_upwmr::MPCWM1_UPWMRrs>;
#[doc = "Unprivileged Water Mark 1 register"]
pub mod mpcwm1_upwmr;
#[doc = "MPCWM1_UPWWMR (rw) register accessor: Unprivileged Writable Water Mark 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm1_upwwmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm1_upwwmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm1_upwwmr`]
module"]
pub type MPCWM1_UPWWMR = crate::Reg<mpcwm1_upwwmr::MPCWM1_UPWWMRrs>;
#[doc = "Unprivileged Writable Water Mark 1 register"]
pub mod mpcwm1_upwwmr;
#[doc = "MPCWM2_UPWMR (rw) register accessor: Unprivileged Water Mark 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm2_upwmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm2_upwmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm2_upwmr`]
module"]
pub type MPCWM2_UPWMR = crate::Reg<mpcwm2_upwmr::MPCWM2_UPWMRrs>;
#[doc = "Unprivileged Water Mark 2 register"]
pub mod mpcwm2_upwmr;
#[doc = "MPCWM3_UPWMR (rw) register accessor: Unprivileged Water Mark 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm3_upwmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm3_upwmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm3_upwmr`]
module"]
pub type MPCWM3_UPWMR = crate::Reg<mpcwm3_upwmr::MPCWM3_UPWMRrs>;
#[doc = "Unprivileged Water Mark 3 register"]
pub mod mpcwm3_upwmr;
