#[repr(C)]
#[doc = "Device OUT endpoint 0"]
pub struct DOEP0 {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    int: INT,
    _reserved2: [u8; 0x04],
    tsiz: TSIZ,
}
impl DOEP0 {
    #[doc = "0x00 - device endpoint-0 control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x08 - device endpoint-0 interrupt register"]
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    #[doc = "0x10 - device OUT endpoint-0 transfer size register"]
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
}
#[doc = "CTL (rw) register accessor: device endpoint-0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTLrs>;
#[doc = "device endpoint-0 control register"]
pub mod ctl;
#[doc = "INT (rw) register accessor: device endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
pub type INT = crate::Reg<int::INTrs>;
#[doc = "device endpoint-0 interrupt register"]
pub mod int;
#[doc = "TSIZ (rw) register accessor: device OUT endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsiz`]
module"]
pub type TSIZ = crate::Reg<tsiz::TSIZrs>;
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod tsiz;
