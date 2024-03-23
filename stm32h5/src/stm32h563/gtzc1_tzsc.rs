#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    seccfgr1: SECCFGR1,
    seccfgr2: SECCFGR2,
    seccfgr3: SECCFGR3,
    _reserved4: [u8; 0x04],
    privcfgr1: PRIVCFGR1,
    privcfgr2: PRIVCFGR2,
    privcfgr3: PRIVCFGR3,
    _reserved7: [u8; 0x14],
    mpcwm1acfgr: MPCWM1ACFGR,
    mpcwm1ar: MPCWM1AR,
    mpcwm1bcfgr: MPCWM1BCFGR,
    mpcwm1br: MPCWM1BR,
    mpcwm2acfgr: MPCWM2ACFGR,
    mpcwm2ar: MPCWM2AR,
    mpcwm2bcfgr: MPCWM2BCFGR,
    mpcwm2br: MPCWM2BR,
    mpcwm3acfgr: MPCWM3ACFGR,
    mpcwm3ar: MPCWM3AR,
    mpcwm3bcfgr: MPCWM3BCFGR,
    mpcwm3br: MPCWM3BR,
    mpcwm4acfgr: MPCWM4ACFGR,
    mpcwm4ar: MPCWM4AR,
    mpcwm4bcfgr: MPCWM4BCFGR,
    mpcwm4br: MPCWM4BR,
}
impl RegisterBlock {
    #[doc = "0x00 - GTZC1 TZSC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x10 - GTZC1 TZSC secure configuration register 1"]
    #[inline(always)]
    pub const fn seccfgr1(&self) -> &SECCFGR1 {
        &self.seccfgr1
    }
    #[doc = "0x14 - GTZC1 TZSC secure configuration register 2"]
    #[inline(always)]
    pub const fn seccfgr2(&self) -> &SECCFGR2 {
        &self.seccfgr2
    }
    #[doc = "0x18 - GTZC1 TZSC secure configuration register 3"]
    #[inline(always)]
    pub const fn seccfgr3(&self) -> &SECCFGR3 {
        &self.seccfgr3
    }
    #[doc = "0x20 - GTZC1 TZSC privilege configuration register 1"]
    #[inline(always)]
    pub const fn privcfgr1(&self) -> &PRIVCFGR1 {
        &self.privcfgr1
    }
    #[doc = "0x24 - GTZC1 TZSC privilege configuration register 2"]
    #[inline(always)]
    pub const fn privcfgr2(&self) -> &PRIVCFGR2 {
        &self.privcfgr2
    }
    #[doc = "0x28 - GTZC1 TZSC privilege configuration register 3"]
    #[inline(always)]
    pub const fn privcfgr3(&self) -> &PRIVCFGR3 {
        &self.privcfgr3
    }
    #[doc = "0x40 - GTZC1 TZSC memory 1 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn mpcwm1acfgr(&self) -> &MPCWM1ACFGR {
        &self.mpcwm1acfgr
    }
    #[doc = "0x44 - GTZC1 TZSC memory 1 sub-region A watermark register"]
    #[inline(always)]
    pub const fn mpcwm1ar(&self) -> &MPCWM1AR {
        &self.mpcwm1ar
    }
    #[doc = "0x48 - GTZC1 TZSC memory 1 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn mpcwm1bcfgr(&self) -> &MPCWM1BCFGR {
        &self.mpcwm1bcfgr
    }
    #[doc = "0x4c - GTZC1 TZSC memory 1 sub-region B watermark register"]
    #[inline(always)]
    pub const fn mpcwm1br(&self) -> &MPCWM1BR {
        &self.mpcwm1br
    }
    #[doc = "0x50 - GTZC1 TZSC memory 2 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn mpcwm2acfgr(&self) -> &MPCWM2ACFGR {
        &self.mpcwm2acfgr
    }
    #[doc = "0x54 - GTZC1 TZSC memory 2 sub-region A watermark register"]
    #[inline(always)]
    pub const fn mpcwm2ar(&self) -> &MPCWM2AR {
        &self.mpcwm2ar
    }
    #[doc = "0x58 - GTZC1 TZSC memory 2 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn mpcwm2bcfgr(&self) -> &MPCWM2BCFGR {
        &self.mpcwm2bcfgr
    }
    #[doc = "0x5c - GTZC1 TZSC memory 2 sub-region B watermark register"]
    #[inline(always)]
    pub const fn mpcwm2br(&self) -> &MPCWM2BR {
        &self.mpcwm2br
    }
    #[doc = "0x60 - GTZC1 TZSC memory 3 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn mpcwm3acfgr(&self) -> &MPCWM3ACFGR {
        &self.mpcwm3acfgr
    }
    #[doc = "0x64 - GTZC1 TZSC memory 3 sub-region A watermark register"]
    #[inline(always)]
    pub const fn mpcwm3ar(&self) -> &MPCWM3AR {
        &self.mpcwm3ar
    }
    #[doc = "0x68 - GTZC1 TZSC memory 3 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn mpcwm3bcfgr(&self) -> &MPCWM3BCFGR {
        &self.mpcwm3bcfgr
    }
    #[doc = "0x6c - GTZC1 TZSC memory 3 sub-region B watermark register"]
    #[inline(always)]
    pub const fn mpcwm3br(&self) -> &MPCWM3BR {
        &self.mpcwm3br
    }
    #[doc = "0x70 - GTZC1 TZSC memory 4 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn mpcwm4acfgr(&self) -> &MPCWM4ACFGR {
        &self.mpcwm4acfgr
    }
    #[doc = "0x74 - GTZC1 TZSC memory 4 sub-region A watermark register"]
    #[inline(always)]
    pub const fn mpcwm4ar(&self) -> &MPCWM4AR {
        &self.mpcwm4ar
    }
    #[doc = "0x78 - GTZC1 TZSC memory 4 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn mpcwm4bcfgr(&self) -> &MPCWM4BCFGR {
        &self.mpcwm4bcfgr
    }
    #[doc = "0x7c - GTZC1 TZSC memory 4 sub-region B watermark register"]
    #[inline(always)]
    pub const fn mpcwm4br(&self) -> &MPCWM4BR {
        &self.mpcwm4br
    }
}
#[doc = "CR (rw) register accessor: GTZC1 TZSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "GTZC1 TZSC control register"]
pub mod cr;
#[doc = "SECCFGR1 (rw) register accessor: GTZC1 TZSC secure configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr1`]
module"]
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1rs>;
#[doc = "GTZC1 TZSC secure configuration register 1"]
pub mod seccfgr1;
#[doc = "SECCFGR2 (rw) register accessor: GTZC1 TZSC secure configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr2`]
module"]
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2rs>;
#[doc = "GTZC1 TZSC secure configuration register 2"]
pub mod seccfgr2;
#[doc = "SECCFGR3 (rw) register accessor: GTZC1 TZSC secure configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr3`]
module"]
pub type SECCFGR3 = crate::Reg<seccfgr3::SECCFGR3rs>;
#[doc = "GTZC1 TZSC secure configuration register 3"]
pub mod seccfgr3;
#[doc = "PRIVCFGR1 (rw) register accessor: GTZC1 TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr1`]
module"]
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1rs>;
#[doc = "GTZC1 TZSC privilege configuration register 1"]
pub mod privcfgr1;
#[doc = "PRIVCFGR2 (rw) register accessor: GTZC1 TZSC privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr2`]
module"]
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2rs>;
#[doc = "GTZC1 TZSC privilege configuration register 2"]
pub mod privcfgr2;
#[doc = "PRIVCFGR3 (rw) register accessor: GTZC1 TZSC privilege configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr3`]
module"]
pub type PRIVCFGR3 = crate::Reg<privcfgr3::PRIVCFGR3rs>;
#[doc = "GTZC1 TZSC privilege configuration register 3"]
pub mod privcfgr3;
#[doc = "MPCWM1ACFGR (rw) register accessor: GTZC1 TZSC memory 1 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm1acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm1acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm1acfgr`]
module"]
pub type MPCWM1ACFGR = crate::Reg<mpcwm1acfgr::MPCWM1ACFGRrs>;
#[doc = "GTZC1 TZSC memory 1 sub-region A watermark configuration register"]
pub mod mpcwm1acfgr;
#[doc = "MPCWM1AR (rw) register accessor: GTZC1 TZSC memory 1 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm1ar`]
module"]
pub type MPCWM1AR = crate::Reg<mpcwm1ar::MPCWM1ARrs>;
#[doc = "GTZC1 TZSC memory 1 sub-region A watermark register"]
pub mod mpcwm1ar;
#[doc = "MPCWM1BCFGR (rw) register accessor: GTZC1 TZSC memory 1 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm1bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm1bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm1bcfgr`]
module"]
pub type MPCWM1BCFGR = crate::Reg<mpcwm1bcfgr::MPCWM1BCFGRrs>;
#[doc = "GTZC1 TZSC memory 1 sub-region B watermark configuration register"]
pub mod mpcwm1bcfgr;
#[doc = "MPCWM1BR (rw) register accessor: GTZC1 TZSC memory 1 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm1br`]
module"]
pub type MPCWM1BR = crate::Reg<mpcwm1br::MPCWM1BRrs>;
#[doc = "GTZC1 TZSC memory 1 sub-region B watermark register"]
pub mod mpcwm1br;
#[doc = "MPCWM2ACFGR (rw) register accessor: GTZC1 TZSC memory 2 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm2acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm2acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm2acfgr`]
module"]
pub type MPCWM2ACFGR = crate::Reg<mpcwm2acfgr::MPCWM2ACFGRrs>;
#[doc = "GTZC1 TZSC memory 2 sub-region A watermark configuration register"]
pub mod mpcwm2acfgr;
#[doc = "MPCWM2AR (rw) register accessor: GTZC1 TZSC memory 2 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm2ar`]
module"]
pub type MPCWM2AR = crate::Reg<mpcwm2ar::MPCWM2ARrs>;
#[doc = "GTZC1 TZSC memory 2 sub-region A watermark register"]
pub mod mpcwm2ar;
#[doc = "MPCWM2BCFGR (rw) register accessor: GTZC1 TZSC memory 2 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm2bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm2bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm2bcfgr`]
module"]
pub type MPCWM2BCFGR = crate::Reg<mpcwm2bcfgr::MPCWM2BCFGRrs>;
#[doc = "GTZC1 TZSC memory 2 sub-region B watermark configuration register"]
pub mod mpcwm2bcfgr;
#[doc = "MPCWM2BR (rw) register accessor: GTZC1 TZSC memory 2 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm2br`]
module"]
pub type MPCWM2BR = crate::Reg<mpcwm2br::MPCWM2BRrs>;
#[doc = "GTZC1 TZSC memory 2 sub-region B watermark register"]
pub mod mpcwm2br;
#[doc = "MPCWM3ACFGR (rw) register accessor: GTZC1 TZSC memory 3 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm3acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm3acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm3acfgr`]
module"]
pub type MPCWM3ACFGR = crate::Reg<mpcwm3acfgr::MPCWM3ACFGRrs>;
#[doc = "GTZC1 TZSC memory 3 sub-region A watermark configuration register"]
pub mod mpcwm3acfgr;
#[doc = "MPCWM3AR (rw) register accessor: GTZC1 TZSC memory 3 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm3ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm3ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm3ar`]
module"]
pub type MPCWM3AR = crate::Reg<mpcwm3ar::MPCWM3ARrs>;
#[doc = "GTZC1 TZSC memory 3 sub-region A watermark register"]
pub mod mpcwm3ar;
#[doc = "MPCWM3BCFGR (rw) register accessor: GTZC1 TZSC memory 3 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm3bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm3bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm3bcfgr`]
module"]
pub type MPCWM3BCFGR = crate::Reg<mpcwm3bcfgr::MPCWM3BCFGRrs>;
#[doc = "GTZC1 TZSC memory 3 sub-region B watermark configuration register"]
pub mod mpcwm3bcfgr;
#[doc = "MPCWM3BR (rw) register accessor: GTZC1 TZSC memory 3 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm3br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm3br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm3br`]
module"]
pub type MPCWM3BR = crate::Reg<mpcwm3br::MPCWM3BRrs>;
#[doc = "GTZC1 TZSC memory 3 sub-region B watermark register"]
pub mod mpcwm3br;
#[doc = "MPCWM4ACFGR (rw) register accessor: GTZC1 TZSC memory 4 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm4acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm4acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm4acfgr`]
module"]
pub type MPCWM4ACFGR = crate::Reg<mpcwm4acfgr::MPCWM4ACFGRrs>;
#[doc = "GTZC1 TZSC memory 4 sub-region A watermark configuration register"]
pub mod mpcwm4acfgr;
#[doc = "MPCWM4AR (rw) register accessor: GTZC1 TZSC memory 4 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm4ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm4ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm4ar`]
module"]
pub type MPCWM4AR = crate::Reg<mpcwm4ar::MPCWM4ARrs>;
#[doc = "GTZC1 TZSC memory 4 sub-region A watermark register"]
pub mod mpcwm4ar;
#[doc = "MPCWM4BCFGR (rw) register accessor: GTZC1 TZSC memory 4 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm4bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm4bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm4bcfgr`]
module"]
pub type MPCWM4BCFGR = crate::Reg<mpcwm4bcfgr::MPCWM4BCFGRrs>;
#[doc = "GTZC1 TZSC memory 4 sub-region B watermark configuration register"]
pub mod mpcwm4bcfgr;
#[doc = "MPCWM4BR (rw) register accessor: GTZC1 TZSC memory 4 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm4br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm4br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcwm4br`]
module"]
pub type MPCWM4BR = crate::Reg<mpcwm4br::MPCWM4BRrs>;
#[doc = "GTZC1 TZSC memory 4 sub-region B watermark register"]
pub mod mpcwm4br;
