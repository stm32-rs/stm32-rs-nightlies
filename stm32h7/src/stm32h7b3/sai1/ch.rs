#[repr(C)]
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub struct CH {
    cr1: CR1,
    cr2: CR2,
    frcr: FRCR,
    slotr: SLOTR,
    im: IM,
    sr: SR,
    clrfr: CLRFR,
    dr: DR,
}
impl CH {
    #[doc = "0x00 - Configuration register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - Configuration register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - This register has no meaning in AC97 and SPDIF audio protocol"]
    #[inline(always)]
    pub const fn frcr(&self) -> &FRCR {
        &self.frcr
    }
    #[doc = "0x0c - This register has no meaning in AC97 and SPDIF audio protocol"]
    #[inline(always)]
    pub const fn slotr(&self) -> &SLOTR {
        &self.slotr
    }
    #[doc = "0x10 - Interrupt mask register 2"]
    #[inline(always)]
    pub const fn im(&self) -> &IM {
        &self.im
    }
    #[doc = "0x14 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x18 - Clear flag register"]
    #[inline(always)]
    pub const fn clrfr(&self) -> &CLRFR {
        &self.clrfr
    }
    #[doc = "0x1c - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
}
#[doc = "CR1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "Configuration register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "Configuration register 2"]
pub mod cr2;
#[doc = "FRCR (rw) register accessor: This register has no meaning in AC97 and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frcr`]
module"]
pub type FRCR = crate::Reg<frcr::FRCRrs>;
#[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
pub mod frcr;
#[doc = "SLOTR (rw) register accessor: This register has no meaning in AC97 and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slotr`]
module"]
pub type SLOTR = crate::Reg<slotr::SLOTRrs>;
#[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
pub mod slotr;
#[doc = "IM (rw) register accessor: Interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`]
module"]
pub type IM = crate::Reg<im::IMrs>;
#[doc = "Interrupt mask register 2"]
pub mod im;
#[doc = "SR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CLRFR (w) register accessor: Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrfr`]
module"]
pub type CLRFR = crate::Reg<clrfr::CLRFRrs>;
#[doc = "Clear flag register"]
pub mod clrfr;
#[doc = "DR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "Data register"]
pub mod dr;
