#[repr(C)]
#[derive(Debug)]
///Device IN endpoint X
pub struct DIEP {
    ctl: CTL,
    _reserved1: [u8; 0x04],
    int: INT,
    _reserved2: [u8; 0x04],
    tsiz: TSIZ,
    _reserved3: [u8; 0x04],
    txfsts: TXFSTS,
    _reserved_end: [u8; 0x04],
}
impl DIEP {
    ///0x00 - OTG device endpoint-1 control register
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    ///0x08 - device endpoint-1 interrupt register
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    ///0x10 - device endpoint-1 transfer size register
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
    ///0x18 - OTG_FS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn txfsts(&self) -> &TXFSTS {
        &self.txfsts
    }
}
/**CTL (rw) register accessor: OTG device endpoint-1 control register

You can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctl`] module*/
pub type CTL = crate::Reg<ctl::CTLrs>;
///OTG device endpoint-1 control register
pub mod ctl;
pub use crate::stm32f215::otg_fs_device::diep0::int;
pub use crate::stm32f215::otg_fs_device::diep0::INT;
/**TSIZ (rw) register accessor: device endpoint-1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tsiz`] module*/
pub type TSIZ = crate::Reg<tsiz::TSIZrs>;
///device endpoint-1 transfer size register
pub mod tsiz;
pub use crate::stm32f215::otg_fs_device::diep0::txfsts;
pub use crate::stm32f215::otg_fs_device::diep0::TXFSTS;
