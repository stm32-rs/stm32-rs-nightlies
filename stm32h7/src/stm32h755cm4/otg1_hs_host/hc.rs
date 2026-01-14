#[repr(C)]
#[derive(Debug)]
///Host channel
pub struct HC {
    char: CHAR,
    splt: SPLT,
    int: INT,
    intmsk: INTMSK,
    tsiz: TSIZ,
    dma: DMA,
    _reserved_end: [u8; 0x08],
}
impl HC {
    ///0x00 - OTG_HS host channel-0 characteristics register
    #[inline(always)]
    pub const fn char(&self) -> &CHAR {
        &self.char
    }
    ///0x04 - OTG_HS host channel-0 split control register
    #[inline(always)]
    pub const fn splt(&self) -> &SPLT {
        &self.splt
    }
    ///0x08 - OTG_HS host channel-11 interrupt register
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    ///0x0c - OTG_HS host channel-11 interrupt mask register
    #[inline(always)]
    pub const fn intmsk(&self) -> &INTMSK {
        &self.intmsk
    }
    ///0x10 - OTG_HS host channel-11 transfer size register
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
    ///0x14 - OTG_HS host channel-0 DMA address register
    #[inline(always)]
    pub const fn dma(&self) -> &DMA {
        &self.dma
    }
}
/**CHAR (rw) register accessor: OTG_HS host channel-0 characteristics register

You can [`read`](crate::Reg::read) this register and get [`char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@char`] module*/
pub type CHAR = crate::Reg<char::CHARrs>;
///OTG_HS host channel-0 characteristics register
pub mod char;
/**SPLT (rw) register accessor: OTG_HS host channel-0 split control register

You can [`read`](crate::Reg::read) this register and get [`splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@splt`] module*/
pub type SPLT = crate::Reg<splt::SPLTrs>;
///OTG_HS host channel-0 split control register
pub mod splt;
/**INT (rw) register accessor: OTG_HS host channel-11 interrupt register

You can [`read`](crate::Reg::read) this register and get [`int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int`] module*/
pub type INT = crate::Reg<int::INTrs>;
///OTG_HS host channel-11 interrupt register
pub mod int;
/**INTMSK (rw) register accessor: OTG_HS host channel-11 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intmsk`] module*/
pub type INTMSK = crate::Reg<intmsk::INTMSKrs>;
///OTG_HS host channel-11 interrupt mask register
pub mod intmsk;
/**TSIZ (rw) register accessor: OTG_HS host channel-11 transfer size register

You can [`read`](crate::Reg::read) this register and get [`tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tsiz`] module*/
pub type TSIZ = crate::Reg<tsiz::TSIZrs>;
///OTG_HS host channel-11 transfer size register
pub mod tsiz;
/**DMA (rw) register accessor: OTG_HS host channel-0 DMA address register

You can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma`] module*/
pub type DMA = crate::Reg<dma::DMArs>;
///OTG_HS host channel-0 DMA address register
pub mod dma;
