#[repr(C)]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub struct CH {
    cr: CR,
    ndtr: NDTR,
    par: PAR,
    mar: MAR,
    _reserved_end: [u8; 0x04],
}
impl CH {
    #[doc = "0x00 - DMA channel x configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn ndtr(&self) -> &NDTR {
        &self.ndtr
    }
    #[doc = "0x08 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn par(&self) -> &PAR {
        &self.par
    }
    #[doc = "0x0c - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn mar(&self) -> &MAR {
        &self.mar
    }
}
#[doc = "CR (rw) register accessor: DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "DMA channel x configuration register"]
pub mod cr;
#[doc = "NDTR (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndtr`]
module"]
pub type NDTR = crate::Reg<ndtr::NDTRrs>;
#[doc = "DMA channel x number of data register"]
pub mod ndtr;
#[doc = "PAR (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`par::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`par::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@par`]
module"]
pub type PAR = crate::Reg<par::PARrs>;
#[doc = "DMA channel x peripheral address register"]
pub mod par;
#[doc = "MAR (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mar`]
module"]
pub type MAR = crate::Reg<mar::MARrs>;
#[doc = "DMA channel x memory address register"]
pub mod mar;
