#[repr(C)]
#[derive(Debug)]
///Host channel
pub struct HC {
    char: CHAR,
    _reserved1: [u8; 0x04],
    int: INT,
    intmsk: INTMSK,
    tsiz: TSIZ,
    _reserved_end: [u8; 0x0c],
}
impl HC {
    ///0x00 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    #[inline(always)]
    pub const fn char(&self) -> &CHAR {
        &self.char
    }
    ///0x08 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    ///0x0c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    #[inline(always)]
    pub const fn intmsk(&self) -> &INTMSK {
        &self.intmsk
    }
    ///0x10 - OTG_FS host channel-0 transfer size register
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
}
/**CHAR (rw) register accessor: OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)

You can [`read`](crate::Reg::read) this register and get [`char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@char`] module*/
pub type CHAR = crate::Reg<char::CHARrs>;
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod char;
/**INT (rw) register accessor: OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)

You can [`read`](crate::Reg::read) this register and get [`int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int`] module*/
pub type INT = crate::Reg<int::INTrs>;
///OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod int;
/**INTMSK (rw) register accessor: OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)

You can [`read`](crate::Reg::read) this register and get [`intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intmsk`] module*/
pub type INTMSK = crate::Reg<intmsk::INTMSKrs>;
///OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod intmsk;
/**TSIZ (rw) register accessor: OTG_FS host channel-0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tsiz`] module*/
pub type TSIZ = crate::Reg<tsiz::TSIZrs>;
///OTG_FS host channel-0 transfer size register
pub mod tsiz;
