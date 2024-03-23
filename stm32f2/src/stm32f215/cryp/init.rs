#[repr(C)]
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR"]
pub struct INIT {
    ivlr: IVLR,
    ivrr: IVRR,
}
impl INIT {
    #[doc = "0x00 - initialization vector registers"]
    #[inline(always)]
    pub const fn ivlr(&self) -> &IVLR {
        &self.ivlr
    }
    #[doc = "0x04 - initialization vector registers"]
    #[inline(always)]
    pub const fn ivrr(&self) -> &IVRR {
        &self.ivrr
    }
}
#[doc = "IVLR (rw) register accessor: initialization vector registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivlr`]
module"]
pub type IVLR = crate::Reg<ivlr::IVLRrs>;
#[doc = "initialization vector registers"]
pub mod ivlr;
#[doc = "IVRR (rw) register accessor: initialization vector registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivrr`]
module"]
pub type IVRR = crate::Reg<ivrr::IVRRrs>;
#[doc = "initialization vector registers"]
pub mod ivrr;
