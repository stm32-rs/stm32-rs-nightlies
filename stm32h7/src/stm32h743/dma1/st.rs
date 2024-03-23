#[repr(C)]
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
pub struct ST {
    cr: CR,
    ndtr: NDTR,
    par: PAR,
    m0ar: M0AR,
    m1ar: M1AR,
    fcr: FCR,
}
impl ST {
    #[doc = "0x00 - stream x configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - stream x number of data register"]
    #[inline(always)]
    pub const fn ndtr(&self) -> &NDTR {
        &self.ndtr
    }
    #[doc = "0x08 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn par(&self) -> &PAR {
        &self.par
    }
    #[doc = "0x0c - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0ar(&self) -> &M0AR {
        &self.m0ar
    }
    #[doc = "0x10 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1ar(&self) -> &M1AR {
        &self.m1ar
    }
    #[doc = "0x14 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
}
#[doc = "CR (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "stream x configuration register"]
pub mod cr;
#[doc = "NDTR (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndtr`]
module"]
pub type NDTR = crate::Reg<ndtr::NDTRrs>;
#[doc = "stream x number of data register"]
pub mod ndtr;
#[doc = "PAR (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@par`]
module"]
pub type PAR = crate::Reg<par::PARrs>;
#[doc = "stream x peripheral address register"]
pub mod par;
#[doc = "M0AR (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0ar`]
module"]
pub type M0AR = crate::Reg<m0ar::M0ARrs>;
#[doc = "stream x memory 0 address register"]
pub mod m0ar;
#[doc = "M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1ar`]
module"]
pub type M1AR = crate::Reg<m1ar::M1ARrs>;
#[doc = "stream x memory 1 address register"]
pub mod m1ar;
#[doc = "FCR (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "stream x FIFO control register"]
pub mod fcr;
