#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    confr0: CONFR0,
    confr1: CONFR1,
    confr2: CONFR2,
    confr3: CONFR3,
    confrn1: CONFRN1,
    confrn2: CONFRN2,
    confrn3: CONFRN3,
    confrn4: CONFRN4,
    _reserved8: [u8; 0x10],
    cr: CR,
    sr: SR,
    cfr: CFR,
    _reserved11: [u8; 0x04],
    dir: DIR,
    dor: DOR,
}
impl RegisterBlock {
    #[doc = "0x00 - JPEG codec control register"]
    #[inline(always)]
    pub const fn confr0(&self) -> &CONFR0 {
        &self.confr0
    }
    #[doc = "0x04 - JPEG codec configuration register 1"]
    #[inline(always)]
    pub const fn confr1(&self) -> &CONFR1 {
        &self.confr1
    }
    #[doc = "0x08 - JPEG codec configuration register 2"]
    #[inline(always)]
    pub const fn confr2(&self) -> &CONFR2 {
        &self.confr2
    }
    #[doc = "0x0c - JPEG codec configuration register 3"]
    #[inline(always)]
    pub const fn confr3(&self) -> &CONFR3 {
        &self.confr3
    }
    #[doc = "0x10 - JPEG codec configuration register 4-7"]
    #[inline(always)]
    pub const fn confrn1(&self) -> &CONFRN1 {
        &self.confrn1
    }
    #[doc = "0x14 - JPEG codec configuration register 4-7"]
    #[inline(always)]
    pub const fn confrn2(&self) -> &CONFRN2 {
        &self.confrn2
    }
    #[doc = "0x18 - JPEG codec configuration register 4-7"]
    #[inline(always)]
    pub const fn confrn3(&self) -> &CONFRN3 {
        &self.confrn3
    }
    #[doc = "0x1c - JPEG codec configuration register 4-7"]
    #[inline(always)]
    pub const fn confrn4(&self) -> &CONFRN4 {
        &self.confrn4
    }
    #[doc = "0x30 - JPEG control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x34 - JPEG status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x38 - JPEG clear flag register"]
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    #[doc = "0x40 - JPEG data input register"]
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
    #[doc = "0x44 - JPEG data output register"]
    #[inline(always)]
    pub const fn dor(&self) -> &DOR {
        &self.dor
    }
}
#[doc = "CONFR0 (w) register accessor: JPEG codec control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confr0`]
module"]
pub type CONFR0 = crate::Reg<confr0::CONFR0rs>;
#[doc = "JPEG codec control register"]
pub mod confr0;
#[doc = "CONFR1 (rw) register accessor: JPEG codec configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confr1`]
module"]
pub type CONFR1 = crate::Reg<confr1::CONFR1rs>;
#[doc = "JPEG codec configuration register 1"]
pub mod confr1;
#[doc = "CONFR2 (rw) register accessor: JPEG codec configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confr2`]
module"]
pub type CONFR2 = crate::Reg<confr2::CONFR2rs>;
#[doc = "JPEG codec configuration register 2"]
pub mod confr2;
#[doc = "CONFR3 (rw) register accessor: JPEG codec configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confr3`]
module"]
pub type CONFR3 = crate::Reg<confr3::CONFR3rs>;
#[doc = "JPEG codec configuration register 3"]
pub mod confr3;
#[doc = "CONFRN1 (rw) register accessor: JPEG codec configuration register 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confrn1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confrn1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confrn1`]
module"]
pub type CONFRN1 = crate::Reg<confrn1::CONFRN1rs>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn1;
#[doc = "CONFRN2 (rw) register accessor: JPEG codec configuration register 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confrn2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confrn2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confrn2`]
module"]
pub type CONFRN2 = crate::Reg<confrn2::CONFRN2rs>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn2;
#[doc = "CONFRN3 (rw) register accessor: JPEG codec configuration register 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confrn3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confrn3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confrn3`]
module"]
pub type CONFRN3 = crate::Reg<confrn3::CONFRN3rs>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn3;
#[doc = "CONFRN4 (rw) register accessor: JPEG codec configuration register 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confrn4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confrn4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confrn4`]
module"]
pub type CONFRN4 = crate::Reg<confrn4::CONFRN4rs>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn4;
#[doc = "CR (rw) register accessor: JPEG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "JPEG control register"]
pub mod cr;
#[doc = "SR (r) register accessor: JPEG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "JPEG status register"]
pub mod sr;
#[doc = "CFR (rw) register accessor: JPEG clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`]
module"]
pub type CFR = crate::Reg<cfr::CFRrs>;
#[doc = "JPEG clear flag register"]
pub mod cfr;
#[doc = "DIR (w) register accessor: JPEG data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
pub type DIR = crate::Reg<dir::DIRrs>;
#[doc = "JPEG data input register"]
pub mod dir;
#[doc = "DOR (r) register accessor: JPEG data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor`]
module"]
pub type DOR = crate::Reg<dor::DORrs>;
#[doc = "JPEG data output register"]
pub mod dor;
