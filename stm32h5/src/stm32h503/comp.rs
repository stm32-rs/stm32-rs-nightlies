#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    comp_sr: COMP_SR,
    comp_icfr: COMP_ICFR,
    _reserved2: [u8; 0x04],
    comp_cfgr1: COMP_CFGR1,
    comp_cfgr2: COMP_CFGR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator status register"]
    #[inline(always)]
    pub const fn comp_sr(&self) -> &COMP_SR {
        &self.comp_sr
    }
    #[doc = "0x04 - Comparator interrupt clear flag register"]
    #[inline(always)]
    pub const fn comp_icfr(&self) -> &COMP_ICFR {
        &self.comp_icfr
    }
    #[doc = "0x0c - Comparator configuration register 1"]
    #[inline(always)]
    pub const fn comp_cfgr1(&self) -> &COMP_CFGR1 {
        &self.comp_cfgr1
    }
    #[doc = "0x10 - Comparator configuration register 2"]
    #[inline(always)]
    pub const fn comp_cfgr2(&self) -> &COMP_CFGR2 {
        &self.comp_cfgr2
    }
}
#[doc = "COMP_SR (r) register accessor: Comparator status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_sr`]
module"]
pub type COMP_SR = crate::Reg<comp_sr::COMP_SRrs>;
#[doc = "Comparator status register"]
pub mod comp_sr;
#[doc = "COMP_ICFR (rw) register accessor: Comparator interrupt clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_icfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_icfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_icfr`]
module"]
pub type COMP_ICFR = crate::Reg<comp_icfr::COMP_ICFRrs>;
#[doc = "Comparator interrupt clear flag register"]
pub mod comp_icfr;
#[doc = "COMP_CFGR1 (rw) register accessor: Comparator configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_cfgr1`]
module"]
pub type COMP_CFGR1 = crate::Reg<comp_cfgr1::COMP_CFGR1rs>;
#[doc = "Comparator configuration register 1"]
pub mod comp_cfgr1;
#[doc = "COMP_CFGR2 (rw) register accessor: Comparator configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_cfgr2`]
module"]
pub type COMP_CFGR2 = crate::Reg<comp_cfgr2::COMP_CFGR2rs>;
#[doc = "Comparator configuration register 2"]
pub mod comp_cfgr2;
