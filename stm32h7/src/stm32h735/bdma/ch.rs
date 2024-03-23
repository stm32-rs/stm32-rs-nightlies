#[repr(C)]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers"]
pub struct CH {
    cr: CR,
    ndtr: NDTR,
    par: PAR,
    m0ar: M0AR,
    m1ar: M1AR,
}
impl CH {
    #[doc = "0x00 - Channel x configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn ndtr(&self) -> &NDTR {
        &self.ndtr
    }
    #[doc = "0x08 - Channel x peripheral address register"]
    #[inline(always)]
    pub const fn par(&self) -> &PAR {
        &self.par
    }
    #[doc = "0x0c - Channel x memory 0 address register"]
    #[inline(always)]
    pub const fn m0ar(&self) -> &M0AR {
        &self.m0ar
    }
    #[doc = "0x10 - Channel x memory 1 address register"]
    #[inline(always)]
    pub const fn m1ar(&self) -> &M1AR {
        &self.m1ar
    }
}
#[doc = "CR (rw) register accessor: Channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Channel x configuration register"]
pub mod cr;
#[doc = "NDTR (rw) register accessor: Channel x number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndtr`]
module"]
pub type NDTR = crate::Reg<ndtr::NDTRrs>;
#[doc = "Channel x number of data to transfer register"]
pub mod ndtr;
#[doc = "PAR (rw) register accessor: Channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@par`]
module"]
pub type PAR = crate::Reg<par::PARrs>;
#[doc = "Channel x peripheral address register"]
pub mod par;
#[doc = "M0AR (rw) register accessor: Channel x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m0ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m0ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0ar`]
module"]
pub type M0AR = crate::Reg<m0ar::M0ARrs>;
#[doc = "Channel x memory 0 address register"]
pub mod m0ar;
#[doc = "M1AR (rw) register accessor: Channel x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1ar`]
module"]
pub type M1AR = crate::Reg<m1ar::M1ARrs>;
#[doc = "Channel x memory 1 address register"]
pub mod m1ar;
