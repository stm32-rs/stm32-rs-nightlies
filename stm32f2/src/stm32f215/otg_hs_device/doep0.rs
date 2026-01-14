#[repr(C)]
#[derive(Debug)]
///Device OUT endpoint 0
pub struct DOEP0 {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    int: INT,
    _reserved2: [u8; 0x04],
    tsiz: TSIZ,
    dma: DMA,
}
impl DOEP0 {
    ///0x00 - OTG_HS device control OUT endpoint 0 control register
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    ///0x08 - OTG_HS device endpoint-0 interrupt register
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    ///0x10 - OTG_HS device endpoint-1 transfer size register
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
    ///0x14 - OTG_HS device endpoint-0 DMA address register
    #[inline(always)]
    pub const fn dma(&self) -> &DMA {
        &self.dma
    }
}
/**CTL (rw) register accessor: OTG_HS device control OUT endpoint 0 control register

You can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctl`] module*/
pub type CTL = crate::Reg<ctl::CTLrs>;
///OTG_HS device control OUT endpoint 0 control register
pub mod ctl;
/**INT (rw) register accessor: OTG_HS device endpoint-0 interrupt register

You can [`read`](crate::Reg::read) this register and get [`int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int`] module*/
pub type INT = crate::Reg<int::INTrs>;
///OTG_HS device endpoint-0 interrupt register
pub mod int;
/**TSIZ (rw) register accessor: OTG_HS device endpoint-1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tsiz`] module*/
pub type TSIZ = crate::Reg<tsiz::TSIZrs>;
///OTG_HS device endpoint-1 transfer size register
pub mod tsiz;
/**DMA (rw) register accessor: OTG_HS device endpoint-0 DMA address register

You can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma`] module*/
pub type DMA = crate::Reg<dma::DMArs>;
///OTG_HS device endpoint-0 DMA address register
pub mod dma;
