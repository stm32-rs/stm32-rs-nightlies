#[repr(C)]
#[doc = "Device IN endpoint X"]
pub struct DIEP {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    int: INT,
    _reserved2: [u8; 0x04],
    tsiz: TSIZ,
    dma: DMA,
    txfsts: TXFSTS,
    _reserved_end: [u8; 0x04],
}
impl DIEP {
    #[doc = "0x00 - OTG device endpoint-1 control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x08 - OTG device endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    #[doc = "0x10 - OTG_HS device endpoint transfer size register"]
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
    #[doc = "0x14 - OTG_HS device endpoint-2 DMA address register"]
    #[inline(always)]
    pub const fn dma(&self) -> &DMA {
        &self.dma
    }
    #[doc = "0x18 - OTG_HS device IN endpoint transmit FIFO status register"]
    #[inline(always)]
    pub const fn txfsts(&self) -> &TXFSTS {
        &self.txfsts
    }
}
#[doc = "CTL (rw) register accessor: OTG device endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTLrs>;
#[doc = "OTG device endpoint-1 control register"]
pub mod ctl;
#[doc = "INT (rw) register accessor: OTG device endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
pub type INT = crate::Reg<int::INTrs>;
#[doc = "OTG device endpoint-1 interrupt register"]
pub mod int;
#[doc = "TSIZ (rw) register accessor: OTG_HS device endpoint transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsiz`]
module"]
pub type TSIZ = crate::Reg<tsiz::TSIZrs>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod tsiz;
#[doc = "DMA (rw) register accessor: OTG_HS device endpoint-2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
pub type DMA = crate::Reg<dma::DMArs>;
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod dma;
#[doc = "TXFSTS (r) register accessor: OTG_HS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfsts`]
module"]
pub type TXFSTS = crate::Reg<txfsts::TXFSTSrs>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod txfsts;
