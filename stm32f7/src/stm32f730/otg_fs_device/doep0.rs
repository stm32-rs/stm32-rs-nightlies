#[repr(C)]
#[derive(Debug)]
///Device OUT endpoint 0
pub struct DOEP0 {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    int: INT,
    _reserved2: [u8; 0x04],
    tsiz: TSIZ,
}
impl DOEP0 {
    ///0x00 - device endpoint-0 control register
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    ///0x08 - device endpoint-0 interrupt register
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    ///0x10 - device OUT endpoint-0 transfer size register
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
}
/**CTL (rw) register accessor: device endpoint-0 control register

You can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctl`] module*/
pub type CTL = crate::Reg<ctl::CTLrs>;
///device endpoint-0 control register
pub mod ctl;
/**INT (rw) register accessor: device endpoint-0 interrupt register

You can [`read`](crate::Reg::read) this register and get [`int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int`] module*/
pub type INT = crate::Reg<int::INTrs>;
///device endpoint-0 interrupt register
pub mod int;
/**TSIZ (rw) register accessor: device OUT endpoint-0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tsiz`] module*/
pub type TSIZ = crate::Reg<tsiz::TSIZrs>;
///device OUT endpoint-0 transfer size register
pub mod tsiz;
