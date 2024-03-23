#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dlyb_cr: DLYB_CR,
    dlyb_cfgr: DLYB_CFGR,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn dlyb_cr(&self) -> &DLYB_CR {
        &self.dlyb_cr
    }
    #[doc = "0x04 - configuration register"]
    #[inline(always)]
    pub const fn dlyb_cfgr(&self) -> &DLYB_CFGR {
        &self.dlyb_cfgr
    }
}
#[doc = "DLYB_CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyb_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlyb_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlyb_cr`]
module"]
pub type DLYB_CR = crate::Reg<dlyb_cr::DLYB_CRrs>;
#[doc = "control register"]
pub mod dlyb_cr;
#[doc = "DLYB_CFGR (rw) register accessor: configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyb_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlyb_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlyb_cfgr`]
module"]
pub type DLYB_CFGR = crate::Reg<dlyb_cfgr::DLYB_CFGRrs>;
#[doc = "configuration register"]
pub mod dlyb_cfgr;
