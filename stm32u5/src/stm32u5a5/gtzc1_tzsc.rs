#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tzsc_cr: TZSC_CR,
    _reserved1: [u8; 0x0c],
    tzsc_seccfgr1: TZSC_SECCFGR1,
    tzsc_seccfgr2: TZSC_SECCFGR2,
    tzsc_seccfgr3: TZSC_SECCFGR3,
    _reserved4: [u8; 0x04],
    tzsc_privcfgr1: TZSC_PRIVCFGR1,
    tzsc_privcfgr2: TZSC_PRIVCFGR2,
    tzsc_privcfgr3: TZSC_PRIVCFGR3,
    _reserved7: [u8; 0x14],
    tzsc_mpcwm1acfgr: TZSC_MPCWM1ACFGR,
    tzsc_mpcwm1ar: TZSC_MPCWM1AR,
    tzsc_mpcwm1bcfgr: TZSC_MPCWM1BCFGR,
    tzsc_mpcwm1br: TZSC_MPCWM1BR,
    tzsc_mpcwm2acfgr: TZSC_MPCWM2ACFGR,
    tzsc_mpcwm2ar: TZSC_MPCWM2AR,
    tzsc_mpcwm2bcfgr: TZSC_MPCWM2BCFGR,
    tzsc_mpcwm2br: TZSC_MPCWM2BR,
    tzsc_mpcwm3acfgr: TZSC_MPCWM3ACFGR,
    tzsc_mpcwm3ar: TZSC_MPCWM3AR,
    _reserved17: [u8; 0x08],
    tzsc_mpcwm4acfgr: TZSC_MPCWM4ACFGR,
    tzsc_mpcwm4ar: TZSC_MPCWM4AR,
    _reserved19: [u8; 0x08],
    tzsc_mpcwm5acfgr: TZSC_MPCWM5ACFGR,
    tzsc_mpcwm5ar: TZSC_MPCWM5AR,
    tzsc_mpcwm5bcfgr: TZSC_MPCWM5BCFGR,
    tzsc_mpcwm5br: TZSC_MPCWM5BR,
    tzsc_mpcwm6acfgr: TZSC_MPCWM6ACFGR,
    tzsc_mpcwm6ar: TZSC_MPCWM6AR,
    tzsc_mpcwm6bcfgr: TZSC_MPCWM6BCFGR,
    tzsc_mpcwm6br: TZSC_MPCWM6BR,
}
impl RegisterBlock {
    #[doc = "0x00 - TZSC control register"]
    #[inline(always)]
    pub const fn tzsc_cr(&self) -> &TZSC_CR {
        &self.tzsc_cr
    }
    #[doc = "0x10 - TZSC secure configuration register 1"]
    #[inline(always)]
    pub const fn tzsc_seccfgr1(&self) -> &TZSC_SECCFGR1 {
        &self.tzsc_seccfgr1
    }
    #[doc = "0x14 - TZSC secure configuration register 2"]
    #[inline(always)]
    pub const fn tzsc_seccfgr2(&self) -> &TZSC_SECCFGR2 {
        &self.tzsc_seccfgr2
    }
    #[doc = "0x18 - TZSC secure configuration register 3"]
    #[inline(always)]
    pub const fn tzsc_seccfgr3(&self) -> &TZSC_SECCFGR3 {
        &self.tzsc_seccfgr3
    }
    #[doc = "0x20 - TZSC privilege configuration register 1"]
    #[inline(always)]
    pub const fn tzsc_privcfgr1(&self) -> &TZSC_PRIVCFGR1 {
        &self.tzsc_privcfgr1
    }
    #[doc = "0x24 - TZSC privilege configuration register 2"]
    #[inline(always)]
    pub const fn tzsc_privcfgr2(&self) -> &TZSC_PRIVCFGR2 {
        &self.tzsc_privcfgr2
    }
    #[doc = "0x28 - TZSC privilege configuration register 3"]
    #[inline(always)]
    pub const fn tzsc_privcfgr3(&self) -> &TZSC_PRIVCFGR3 {
        &self.tzsc_privcfgr3
    }
    #[doc = "0x40 - TZSC memory 1 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm1acfgr(&self) -> &TZSC_MPCWM1ACFGR {
        &self.tzsc_mpcwm1acfgr
    }
    #[doc = "0x44 - TZSC memory 1 sub-region A watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm1ar(&self) -> &TZSC_MPCWM1AR {
        &self.tzsc_mpcwm1ar
    }
    #[doc = "0x48 - TZSC memory 1 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm1bcfgr(&self) -> &TZSC_MPCWM1BCFGR {
        &self.tzsc_mpcwm1bcfgr
    }
    #[doc = "0x4c - TZSC memory 1 sub-region B watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm1br(&self) -> &TZSC_MPCWM1BR {
        &self.tzsc_mpcwm1br
    }
    #[doc = "0x50 - TZSC memory 2 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm2acfgr(&self) -> &TZSC_MPCWM2ACFGR {
        &self.tzsc_mpcwm2acfgr
    }
    #[doc = "0x54 - TZSC memory 2 sub-region A watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm2ar(&self) -> &TZSC_MPCWM2AR {
        &self.tzsc_mpcwm2ar
    }
    #[doc = "0x58 - TZSC memory 2 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm2bcfgr(&self) -> &TZSC_MPCWM2BCFGR {
        &self.tzsc_mpcwm2bcfgr
    }
    #[doc = "0x5c - TZSC memory 2 sub-region B watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm2br(&self) -> &TZSC_MPCWM2BR {
        &self.tzsc_mpcwm2br
    }
    #[doc = "0x60 - TZSC memory 3 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm3acfgr(&self) -> &TZSC_MPCWM3ACFGR {
        &self.tzsc_mpcwm3acfgr
    }
    #[doc = "0x64 - TZSC memory 3 sub-region A watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm3ar(&self) -> &TZSC_MPCWM3AR {
        &self.tzsc_mpcwm3ar
    }
    #[doc = "0x70 - TZSC memory 4 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm4acfgr(&self) -> &TZSC_MPCWM4ACFGR {
        &self.tzsc_mpcwm4acfgr
    }
    #[doc = "0x74 - TZSC memory 4 sub-region A watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm4ar(&self) -> &TZSC_MPCWM4AR {
        &self.tzsc_mpcwm4ar
    }
    #[doc = "0x80 - TZSC memory 5 sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm5acfgr(&self) -> &TZSC_MPCWM5ACFGR {
        &self.tzsc_mpcwm5acfgr
    }
    #[doc = "0x84 - TZSC memory 5 sub-region A watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm5ar(&self) -> &TZSC_MPCWM5AR {
        &self.tzsc_mpcwm5ar
    }
    #[doc = "0x88 - TZSC memory 5 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm5bcfgr(&self) -> &TZSC_MPCWM5BCFGR {
        &self.tzsc_mpcwm5bcfgr
    }
    #[doc = "0x8c - TZSC memory 5 sub-region B watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm5br(&self) -> &TZSC_MPCWM5BR {
        &self.tzsc_mpcwm5br
    }
    #[doc = "0x90 - TZSC memory 6 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm6acfgr(&self) -> &TZSC_MPCWM6ACFGR {
        &self.tzsc_mpcwm6acfgr
    }
    #[doc = "0x94 - TZSC memory 6 sub-region B watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm6ar(&self) -> &TZSC_MPCWM6AR {
        &self.tzsc_mpcwm6ar
    }
    #[doc = "0x98 - TZSC memory 6 sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm6bcfgr(&self) -> &TZSC_MPCWM6BCFGR {
        &self.tzsc_mpcwm6bcfgr
    }
    #[doc = "0x9c - TZSC memory 6 sub-region B watermark register"]
    #[inline(always)]
    pub const fn tzsc_mpcwm6br(&self) -> &TZSC_MPCWM6BR {
        &self.tzsc_mpcwm6br
    }
}
#[doc = "TZSC_CR (rw) register accessor: TZSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_cr`]
module"]
pub type TZSC_CR = crate::Reg<tzsc_cr::TZSC_CRrs>;
#[doc = "TZSC control register"]
pub mod tzsc_cr;
#[doc = "TZSC_SECCFGR1 (rw) register accessor: TZSC secure configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_seccfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_seccfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_seccfgr1`]
module"]
pub type TZSC_SECCFGR1 = crate::Reg<tzsc_seccfgr1::TZSC_SECCFGR1rs>;
#[doc = "TZSC secure configuration register 1"]
pub mod tzsc_seccfgr1;
#[doc = "TZSC_SECCFGR2 (rw) register accessor: TZSC secure configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_seccfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_seccfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_seccfgr2`]
module"]
pub type TZSC_SECCFGR2 = crate::Reg<tzsc_seccfgr2::TZSC_SECCFGR2rs>;
#[doc = "TZSC secure configuration register 2"]
pub mod tzsc_seccfgr2;
#[doc = "TZSC_SECCFGR3 (rw) register accessor: TZSC secure configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_seccfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_seccfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_seccfgr3`]
module"]
pub type TZSC_SECCFGR3 = crate::Reg<tzsc_seccfgr3::TZSC_SECCFGR3rs>;
#[doc = "TZSC secure configuration register 3"]
pub mod tzsc_seccfgr3;
#[doc = "TZSC_PRIVCFGR1 (rw) register accessor: TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr1`]
module"]
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1rs>;
#[doc = "TZSC privilege configuration register 1"]
pub mod tzsc_privcfgr1;
#[doc = "TZSC_PRIVCFGR2 (rw) register accessor: TZSC privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_privcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_privcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr2`]
module"]
pub type TZSC_PRIVCFGR2 = crate::Reg<tzsc_privcfgr2::TZSC_PRIVCFGR2rs>;
#[doc = "TZSC privilege configuration register 2"]
pub mod tzsc_privcfgr2;
#[doc = "TZSC_PRIVCFGR3 (rw) register accessor: TZSC privilege configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_privcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_privcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr3`]
module"]
pub type TZSC_PRIVCFGR3 = crate::Reg<tzsc_privcfgr3::TZSC_PRIVCFGR3rs>;
#[doc = "TZSC privilege configuration register 3"]
pub mod tzsc_privcfgr3;
#[doc = "TZSC_MPCWM1ACFGR (rw) register accessor: TZSC memory 1 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm1acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm1acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm1acfgr`]
module"]
pub type TZSC_MPCWM1ACFGR = crate::Reg<tzsc_mpcwm1acfgr::TZSC_MPCWM1ACFGRrs>;
#[doc = "TZSC memory 1 sub-region A watermark configuration register"]
pub mod tzsc_mpcwm1acfgr;
#[doc = "TZSC_MPCWM1AR (rw) register accessor: TZSC memory 1 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm1ar`]
module"]
pub type TZSC_MPCWM1AR = crate::Reg<tzsc_mpcwm1ar::TZSC_MPCWM1ARrs>;
#[doc = "TZSC memory 1 sub-region A watermark register"]
pub mod tzsc_mpcwm1ar;
#[doc = "TZSC_MPCWM1BCFGR (rw) register accessor: TZSC memory 1 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm1bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm1bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm1bcfgr`]
module"]
pub type TZSC_MPCWM1BCFGR = crate::Reg<tzsc_mpcwm1bcfgr::TZSC_MPCWM1BCFGRrs>;
#[doc = "TZSC memory 1 sub-region B watermark configuration register"]
pub mod tzsc_mpcwm1bcfgr;
#[doc = "TZSC_MPCWM1BR (rw) register accessor: TZSC memory 1 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm1br`]
module"]
pub type TZSC_MPCWM1BR = crate::Reg<tzsc_mpcwm1br::TZSC_MPCWM1BRrs>;
#[doc = "TZSC memory 1 sub-region B watermark register"]
pub mod tzsc_mpcwm1br;
#[doc = "TZSC_MPCWM2ACFGR (rw) register accessor: TZSC memory 2 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm2acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm2acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm2acfgr`]
module"]
pub type TZSC_MPCWM2ACFGR = crate::Reg<tzsc_mpcwm2acfgr::TZSC_MPCWM2ACFGRrs>;
#[doc = "TZSC memory 2 sub-region A watermark configuration register"]
pub mod tzsc_mpcwm2acfgr;
#[doc = "TZSC_MPCWM2AR (rw) register accessor: TZSC memory 2 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm2ar`]
module"]
pub type TZSC_MPCWM2AR = crate::Reg<tzsc_mpcwm2ar::TZSC_MPCWM2ARrs>;
#[doc = "TZSC memory 2 sub-region A watermark register"]
pub mod tzsc_mpcwm2ar;
#[doc = "TZSC_MPCWM2BCFGR (rw) register accessor: TZSC memory 2 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm2bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm2bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm2bcfgr`]
module"]
pub type TZSC_MPCWM2BCFGR = crate::Reg<tzsc_mpcwm2bcfgr::TZSC_MPCWM2BCFGRrs>;
#[doc = "TZSC memory 2 sub-region B watermark configuration register"]
pub mod tzsc_mpcwm2bcfgr;
#[doc = "TZSC_MPCWM2BR (rw) register accessor: TZSC memory 2 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm2br`]
module"]
pub type TZSC_MPCWM2BR = crate::Reg<tzsc_mpcwm2br::TZSC_MPCWM2BRrs>;
#[doc = "TZSC memory 2 sub-region B watermark register"]
pub mod tzsc_mpcwm2br;
#[doc = "TZSC_MPCWM3ACFGR (rw) register accessor: TZSC memory 3 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm3acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm3acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm3acfgr`]
module"]
pub type TZSC_MPCWM3ACFGR = crate::Reg<tzsc_mpcwm3acfgr::TZSC_MPCWM3ACFGRrs>;
#[doc = "TZSC memory 3 sub-region A watermark configuration register"]
pub mod tzsc_mpcwm3acfgr;
#[doc = "TZSC_MPCWM3AR (rw) register accessor: TZSC memory 3 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm3ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm3ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm3ar`]
module"]
pub type TZSC_MPCWM3AR = crate::Reg<tzsc_mpcwm3ar::TZSC_MPCWM3ARrs>;
#[doc = "TZSC memory 3 sub-region A watermark register"]
pub mod tzsc_mpcwm3ar;
#[doc = "TZSC_MPCWM4ACFGR (rw) register accessor: TZSC memory 4 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm4acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm4acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4acfgr`]
module"]
pub type TZSC_MPCWM4ACFGR = crate::Reg<tzsc_mpcwm4acfgr::TZSC_MPCWM4ACFGRrs>;
#[doc = "TZSC memory 4 sub-region A watermark configuration register"]
pub mod tzsc_mpcwm4acfgr;
#[doc = "TZSC_MPCWM4AR (rw) register accessor: TZSC memory 4 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm4ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm4ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4ar`]
module"]
pub type TZSC_MPCWM4AR = crate::Reg<tzsc_mpcwm4ar::TZSC_MPCWM4ARrs>;
#[doc = "TZSC memory 4 sub-region A watermark register"]
pub mod tzsc_mpcwm4ar;
#[doc = "TZSC_MPCWM5ACFGR (rw) register accessor: TZSC memory 5 sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm5acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm5acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm5acfgr`]
module"]
pub type TZSC_MPCWM5ACFGR = crate::Reg<tzsc_mpcwm5acfgr::TZSC_MPCWM5ACFGRrs>;
#[doc = "TZSC memory 5 sub-region A watermark configuration register"]
pub mod tzsc_mpcwm5acfgr;
#[doc = "TZSC_MPCWM5AR (rw) register accessor: TZSC memory 5 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm5ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm5ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm5ar`]
module"]
pub type TZSC_MPCWM5AR = crate::Reg<tzsc_mpcwm5ar::TZSC_MPCWM5ARrs>;
#[doc = "TZSC memory 5 sub-region A watermark register"]
pub mod tzsc_mpcwm5ar;
#[doc = "TZSC_MPCWM5BCFGR (rw) register accessor: TZSC memory 5 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm5bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm5bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm5bcfgr`]
module"]
pub type TZSC_MPCWM5BCFGR = crate::Reg<tzsc_mpcwm5bcfgr::TZSC_MPCWM5BCFGRrs>;
#[doc = "TZSC memory 5 sub-region B watermark configuration register"]
pub mod tzsc_mpcwm5bcfgr;
#[doc = "TZSC_MPCWM5BR (rw) register accessor: TZSC memory 5 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm5br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm5br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm5br`]
module"]
pub type TZSC_MPCWM5BR = crate::Reg<tzsc_mpcwm5br::TZSC_MPCWM5BRrs>;
#[doc = "TZSC memory 5 sub-region B watermark register"]
pub mod tzsc_mpcwm5br;
#[doc = "TZSC_MPCWM6ACFGR (rw) register accessor: TZSC memory 6 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm6acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm6acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm6acfgr`]
module"]
pub type TZSC_MPCWM6ACFGR = crate::Reg<tzsc_mpcwm6acfgr::TZSC_MPCWM6ACFGRrs>;
#[doc = "TZSC memory 6 sub-region B watermark configuration register"]
pub mod tzsc_mpcwm6acfgr;
#[doc = "TZSC_MPCWM6AR (rw) register accessor: TZSC memory 6 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm6ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm6ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm6ar`]
module"]
pub type TZSC_MPCWM6AR = crate::Reg<tzsc_mpcwm6ar::TZSC_MPCWM6ARrs>;
#[doc = "TZSC memory 6 sub-region B watermark register"]
pub mod tzsc_mpcwm6ar;
#[doc = "TZSC_MPCWM6BCFGR (rw) register accessor: TZSC memory 6 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm6bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm6bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm6bcfgr`]
module"]
pub type TZSC_MPCWM6BCFGR = crate::Reg<tzsc_mpcwm6bcfgr::TZSC_MPCWM6BCFGRrs>;
#[doc = "TZSC memory 6 sub-region B watermark configuration register"]
pub mod tzsc_mpcwm6bcfgr;
#[doc = "TZSC_MPCWM6BR (rw) register accessor: TZSC memory 6 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm6br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm6br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm6br`]
module"]
pub type TZSC_MPCWM6BR = crate::Reg<tzsc_mpcwm6br::TZSC_MPCWM6BRrs>;
#[doc = "TZSC memory 6 sub-region B watermark register"]
pub mod tzsc_mpcwm6br;
