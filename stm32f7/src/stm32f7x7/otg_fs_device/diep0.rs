#[repr(C)]
#[doc = "Device IN endpoint 0"]
pub struct DIEP0 {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    int: INT,
    _reserved2: [u8; 0x04],
    tsiz: TSIZ,
    _reserved3: [u8; 0x04],
    txfsts: TXFSTS,
}
impl DIEP0 {
    #[doc = "0x00 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x08 - device endpoint-x interrupt register"]
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    #[doc = "0x10 - device endpoint-0 transfer size register"]
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
    #[doc = "0x18 - OTG_FS device IN endpoint transmit FIFO status register"]
    #[inline(always)]
    pub const fn txfsts(&self) -> &TXFSTS {
        &self.txfsts
    }
}
#[doc = "CTL (rw) register accessor: OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTLrs>;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod ctl;
#[doc = "INT (rw) register accessor: device endpoint-x interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
pub type INT = crate::Reg<int::INTrs>;
#[doc = "device endpoint-x interrupt register"]
pub mod int;
#[doc = "TSIZ (rw) register accessor: device endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsiz`]
module"]
pub type TSIZ = crate::Reg<tsiz::TSIZrs>;
#[doc = "device endpoint-0 transfer size register"]
pub mod tsiz;
#[doc = "TXFSTS (r) register accessor: OTG_FS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfsts`]
module"]
pub type TXFSTS = crate::Reg<txfsts::TXFSTSrs>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod txfsts;
