#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    hprt: HPRT,
    _reserved7: [u8; 0xbc],
    hcchar0: HCCHAR0,
    _reserved8: [u8; 0x04],
    hcint0: HCINT0,
    hcintmsk0: HCINTMSK0,
    hctsiz0: HCTSIZ0,
    _reserved11: [u8; 0x0c],
    hcchar1: HCCHAR1,
    _reserved12: [u8; 0x04],
    hcint1: HCINT1,
    hcintmsk1: HCINTMSK1,
    hctsiz1: HCTSIZ1,
    _reserved15: [u8; 0x0c],
    hcchar2: HCCHAR2,
    _reserved16: [u8; 0x04],
    hcint2: HCINT2,
    hcintmsk2: HCINTMSK2,
    hctsiz2: HCTSIZ2,
    _reserved19: [u8; 0x0c],
    hcchar3: HCCHAR3,
    _reserved20: [u8; 0x04],
    hcint3: HCINT3,
    hcintmsk3: HCINTMSK3,
    hctsiz3: HCTSIZ3,
    _reserved23: [u8; 0x0c],
    hcchar4: HCCHAR4,
    _reserved24: [u8; 0x04],
    hcint4: HCINT4,
    hcintmsk4: HCINTMSK4,
    hctsiz4: HCTSIZ4,
    _reserved27: [u8; 0x0c],
    hcchar5: HCCHAR5,
    _reserved28: [u8; 0x04],
    hcint5: HCINT5,
    hcintmsk5: HCINTMSK5,
    hctsiz5: HCTSIZ5,
    _reserved31: [u8; 0x0c],
    hcchar6: HCCHAR6,
    _reserved32: [u8; 0x04],
    hcint6: HCINT6,
    hcintmsk6: HCINTMSK6,
    hctsiz6: HCTSIZ6,
    _reserved35: [u8; 0x0c],
    hcchar7: HCCHAR7,
    _reserved36: [u8; 0x04],
    hcint7: HCINT7,
    hcintmsk7: HCINTMSK7,
    hctsiz7: HCTSIZ7,
}
impl RegisterBlock {
    ///0x00 - OTG_FS host configuration register (OTG_FS_HCFG)
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    ///0x04 - OTG_FS Host frame interval register
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    ///0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    ///0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    ///0x14 - OTG_FS Host all channels interrupt register
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    ///0x18 - OTG_FS host all channels interrupt mask register
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    ///0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    ///0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    #[inline(always)]
    pub const fn hcchar0(&self) -> &HCCHAR0 {
        &self.hcchar0
    }
    ///0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    #[inline(always)]
    pub const fn hcint0(&self) -> &HCINT0 {
        &self.hcint0
    }
    ///0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    #[inline(always)]
    pub const fn hcintmsk0(&self) -> &HCINTMSK0 {
        &self.hcintmsk0
    }
    ///0x110 - OTG_FS host channel-0 transfer size register
    #[inline(always)]
    pub const fn hctsiz0(&self) -> &HCTSIZ0 {
        &self.hctsiz0
    }
    ///0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
    #[inline(always)]
    pub const fn hcchar1(&self) -> &HCCHAR1 {
        &self.hcchar1
    }
    ///0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
    #[inline(always)]
    pub const fn hcint1(&self) -> &HCINT1 {
        &self.hcint1
    }
    ///0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
    #[inline(always)]
    pub const fn hcintmsk1(&self) -> &HCINTMSK1 {
        &self.hcintmsk1
    }
    ///0x130 - OTG_FS host channel-1 transfer size register
    #[inline(always)]
    pub const fn hctsiz1(&self) -> &HCTSIZ1 {
        &self.hctsiz1
    }
    ///0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
    #[inline(always)]
    pub const fn hcchar2(&self) -> &HCCHAR2 {
        &self.hcchar2
    }
    ///0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
    #[inline(always)]
    pub const fn hcint2(&self) -> &HCINT2 {
        &self.hcint2
    }
    ///0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
    #[inline(always)]
    pub const fn hcintmsk2(&self) -> &HCINTMSK2 {
        &self.hcintmsk2
    }
    ///0x150 - OTG_FS host channel-2 transfer size register
    #[inline(always)]
    pub const fn hctsiz2(&self) -> &HCTSIZ2 {
        &self.hctsiz2
    }
    ///0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
    #[inline(always)]
    pub const fn hcchar3(&self) -> &HCCHAR3 {
        &self.hcchar3
    }
    ///0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
    #[inline(always)]
    pub const fn hcint3(&self) -> &HCINT3 {
        &self.hcint3
    }
    ///0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
    #[inline(always)]
    pub const fn hcintmsk3(&self) -> &HCINTMSK3 {
        &self.hcintmsk3
    }
    ///0x170 - OTG_FS host channel-3 transfer size register
    #[inline(always)]
    pub const fn hctsiz3(&self) -> &HCTSIZ3 {
        &self.hctsiz3
    }
    ///0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
    #[inline(always)]
    pub const fn hcchar4(&self) -> &HCCHAR4 {
        &self.hcchar4
    }
    ///0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
    #[inline(always)]
    pub const fn hcint4(&self) -> &HCINT4 {
        &self.hcint4
    }
    ///0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
    #[inline(always)]
    pub const fn hcintmsk4(&self) -> &HCINTMSK4 {
        &self.hcintmsk4
    }
    ///0x190 - OTG_FS host channel-x transfer size register
    #[inline(always)]
    pub const fn hctsiz4(&self) -> &HCTSIZ4 {
        &self.hctsiz4
    }
    ///0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
    #[inline(always)]
    pub const fn hcchar5(&self) -> &HCCHAR5 {
        &self.hcchar5
    }
    ///0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
    #[inline(always)]
    pub const fn hcint5(&self) -> &HCINT5 {
        &self.hcint5
    }
    ///0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
    #[inline(always)]
    pub const fn hcintmsk5(&self) -> &HCINTMSK5 {
        &self.hcintmsk5
    }
    ///0x1b0 - OTG_FS host channel-5 transfer size register
    #[inline(always)]
    pub const fn hctsiz5(&self) -> &HCTSIZ5 {
        &self.hctsiz5
    }
    ///0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
    #[inline(always)]
    pub const fn hcchar6(&self) -> &HCCHAR6 {
        &self.hcchar6
    }
    ///0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
    #[inline(always)]
    pub const fn hcint6(&self) -> &HCINT6 {
        &self.hcint6
    }
    ///0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
    #[inline(always)]
    pub const fn hcintmsk6(&self) -> &HCINTMSK6 {
        &self.hcintmsk6
    }
    ///0x1d0 - OTG_FS host channel-6 transfer size register
    #[inline(always)]
    pub const fn hctsiz6(&self) -> &HCTSIZ6 {
        &self.hctsiz6
    }
    ///0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
    #[inline(always)]
    pub const fn hcchar7(&self) -> &HCCHAR7 {
        &self.hcchar7
    }
    ///0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
    #[inline(always)]
    pub const fn hcint7(&self) -> &HCINT7 {
        &self.hcint7
    }
    ///0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
    #[inline(always)]
    pub const fn hcintmsk7(&self) -> &HCINTMSK7 {
        &self.hcintmsk7
    }
    ///0x1f0 - OTG_FS host channel-7 transfer size register
    #[inline(always)]
    pub const fn hctsiz7(&self) -> &HCTSIZ7 {
        &self.hctsiz7
    }
}
/**HCFG (rw) register accessor: OTG_FS host configuration register (OTG_FS_HCFG)

You can [`read`](crate::Reg::read) this register and get [`hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCFG)

For information about available fields see [`mod@hcfg`] module*/
pub type HCFG = crate::Reg<hcfg::HCFGrs>;
///OTG_FS host configuration register (OTG_FS_HCFG)
pub mod hcfg;
/**HFIR (rw) register accessor: OTG_FS Host frame interval register

You can [`read`](crate::Reg::read) this register and get [`hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HFIR)

For information about available fields see [`mod@hfir`] module*/
pub type HFIR = crate::Reg<hfir::HFIRrs>;
///OTG_FS Host frame interval register
pub mod hfir;
/**HFNUM (r) register accessor: OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)

You can [`read`](crate::Reg::read) this register and get [`hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HFNUM)

For information about available fields see [`mod@hfnum`] module*/
pub type HFNUM = crate::Reg<hfnum::HFNUMrs>;
///OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
pub mod hfnum;
/**HPTXSTS (rw) register accessor: OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)

You can [`read`](crate::Reg::read) this register and get [`hptxsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HPTXSTS)

For information about available fields see [`mod@hptxsts`] module*/
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTSrs>;
///OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
pub mod hptxsts;
/**HAINT (r) register accessor: OTG_FS Host all channels interrupt register

You can [`read`](crate::Reg::read) this register and get [`haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HAINT)

For information about available fields see [`mod@haint`] module*/
pub type HAINT = crate::Reg<haint::HAINTrs>;
///OTG_FS Host all channels interrupt register
pub mod haint;
/**HAINTMSK (rw) register accessor: OTG_FS host all channels interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HAINTMSK)

For information about available fields see [`mod@haintmsk`] module*/
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSKrs>;
///OTG_FS host all channels interrupt mask register
pub mod haintmsk;
/**HPRT (rw) register accessor: OTG_FS host port control and status register (OTG_FS_HPRT)

You can [`read`](crate::Reg::read) this register and get [`hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HPRT)

For information about available fields see [`mod@hprt`] module*/
pub type HPRT = crate::Reg<hprt::HPRTrs>;
///OTG_FS host port control and status register (OTG_FS_HPRT)
pub mod hprt;
/**HCCHAR0 (rw) register accessor: OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)

You can [`read`](crate::Reg::read) this register and get [`hcchar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCCHAR0)

For information about available fields see [`mod@hcchar0`] module*/
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0rs>;
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod hcchar0;
/**HCCHAR1 (rw) register accessor: OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)

You can [`read`](crate::Reg::read) this register and get [`hcchar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCCHAR1)

For information about available fields see [`mod@hcchar1`] module*/
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1rs>;
///OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
pub mod hcchar1;
/**HCCHAR2 (rw) register accessor: OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)

You can [`read`](crate::Reg::read) this register and get [`hcchar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCCHAR2)

For information about available fields see [`mod@hcchar2`] module*/
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2rs>;
///OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
pub mod hcchar2;
/**HCCHAR3 (rw) register accessor: OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)

You can [`read`](crate::Reg::read) this register and get [`hcchar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCCHAR3)

For information about available fields see [`mod@hcchar3`] module*/
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3rs>;
///OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
pub mod hcchar3;
/**HCCHAR4 (rw) register accessor: OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)

You can [`read`](crate::Reg::read) this register and get [`hcchar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCCHAR4)

For information about available fields see [`mod@hcchar4`] module*/
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4rs>;
///OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
pub mod hcchar4;
/**HCCHAR5 (rw) register accessor: OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)

You can [`read`](crate::Reg::read) this register and get [`hcchar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCCHAR5)

For information about available fields see [`mod@hcchar5`] module*/
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5rs>;
///OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
pub mod hcchar5;
/**HCCHAR6 (rw) register accessor: OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)

You can [`read`](crate::Reg::read) this register and get [`hcchar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCCHAR6)

For information about available fields see [`mod@hcchar6`] module*/
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6rs>;
///OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
pub mod hcchar6;
/**HCCHAR7 (rw) register accessor: OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)

You can [`read`](crate::Reg::read) this register and get [`hcchar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCCHAR7)

For information about available fields see [`mod@hcchar7`] module*/
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7rs>;
///OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
pub mod hcchar7;
/**HCINT0 (rw) register accessor: OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)

You can [`read`](crate::Reg::read) this register and get [`hcint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINT0)

For information about available fields see [`mod@hcint0`] module*/
pub type HCINT0 = crate::Reg<hcint0::HCINT0rs>;
///OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod hcint0;
/**HCINT1 (rw) register accessor: OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)

You can [`read`](crate::Reg::read) this register and get [`hcint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINT1)

For information about available fields see [`mod@hcint1`] module*/
pub type HCINT1 = crate::Reg<hcint1::HCINT1rs>;
///OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
pub mod hcint1;
/**HCINT2 (rw) register accessor: OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)

You can [`read`](crate::Reg::read) this register and get [`hcint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINT2)

For information about available fields see [`mod@hcint2`] module*/
pub type HCINT2 = crate::Reg<hcint2::HCINT2rs>;
///OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
pub mod hcint2;
/**HCINT3 (rw) register accessor: OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)

You can [`read`](crate::Reg::read) this register and get [`hcint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINT3)

For information about available fields see [`mod@hcint3`] module*/
pub type HCINT3 = crate::Reg<hcint3::HCINT3rs>;
///OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
pub mod hcint3;
/**HCINT4 (rw) register accessor: OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)

You can [`read`](crate::Reg::read) this register and get [`hcint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINT4)

For information about available fields see [`mod@hcint4`] module*/
pub type HCINT4 = crate::Reg<hcint4::HCINT4rs>;
///OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
pub mod hcint4;
/**HCINT5 (rw) register accessor: OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)

You can [`read`](crate::Reg::read) this register and get [`hcint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINT5)

For information about available fields see [`mod@hcint5`] module*/
pub type HCINT5 = crate::Reg<hcint5::HCINT5rs>;
///OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
pub mod hcint5;
/**HCINT6 (rw) register accessor: OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)

You can [`read`](crate::Reg::read) this register and get [`hcint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINT6)

For information about available fields see [`mod@hcint6`] module*/
pub type HCINT6 = crate::Reg<hcint6::HCINT6rs>;
///OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
pub mod hcint6;
/**HCINT7 (rw) register accessor: OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)

You can [`read`](crate::Reg::read) this register and get [`hcint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINT7)

For information about available fields see [`mod@hcint7`] module*/
pub type HCINT7 = crate::Reg<hcint7::HCINT7rs>;
///OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
pub mod hcint7;
/**HCINTMSK0 (rw) register accessor: OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)

You can [`read`](crate::Reg::read) this register and get [`hcintmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINTMSK0)

For information about available fields see [`mod@hcintmsk0`] module*/
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0rs>;
///OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod hcintmsk0;
/**HCINTMSK1 (rw) register accessor: OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)

You can [`read`](crate::Reg::read) this register and get [`hcintmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINTMSK1)

For information about available fields see [`mod@hcintmsk1`] module*/
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1rs>;
///OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
pub mod hcintmsk1;
/**HCINTMSK2 (rw) register accessor: OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)

You can [`read`](crate::Reg::read) this register and get [`hcintmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINTMSK2)

For information about available fields see [`mod@hcintmsk2`] module*/
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2rs>;
///OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
pub mod hcintmsk2;
/**HCINTMSK3 (rw) register accessor: OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)

You can [`read`](crate::Reg::read) this register and get [`hcintmsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINTMSK3)

For information about available fields see [`mod@hcintmsk3`] module*/
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3rs>;
///OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
pub mod hcintmsk3;
/**HCINTMSK4 (rw) register accessor: OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)

You can [`read`](crate::Reg::read) this register and get [`hcintmsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINTMSK4)

For information about available fields see [`mod@hcintmsk4`] module*/
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4rs>;
///OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
pub mod hcintmsk4;
/**HCINTMSK5 (rw) register accessor: OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)

You can [`read`](crate::Reg::read) this register and get [`hcintmsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINTMSK5)

For information about available fields see [`mod@hcintmsk5`] module*/
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5rs>;
///OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
pub mod hcintmsk5;
/**HCINTMSK6 (rw) register accessor: OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)

You can [`read`](crate::Reg::read) this register and get [`hcintmsk6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINTMSK6)

For information about available fields see [`mod@hcintmsk6`] module*/
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6rs>;
///OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
pub mod hcintmsk6;
/**HCINTMSK7 (rw) register accessor: OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)

You can [`read`](crate::Reg::read) this register and get [`hcintmsk7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCINTMSK7)

For information about available fields see [`mod@hcintmsk7`] module*/
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7rs>;
///OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
pub mod hcintmsk7;
/**HCTSIZ0 (rw) register accessor: OTG_FS host channel-0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCTSIZ0)

For information about available fields see [`mod@hctsiz0`] module*/
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0rs>;
///OTG_FS host channel-0 transfer size register
pub mod hctsiz0;
/**HCTSIZ1 (rw) register accessor: OTG_FS host channel-1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCTSIZ1)

For information about available fields see [`mod@hctsiz1`] module*/
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1rs>;
///OTG_FS host channel-1 transfer size register
pub mod hctsiz1;
/**HCTSIZ2 (rw) register accessor: OTG_FS host channel-2 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCTSIZ2)

For information about available fields see [`mod@hctsiz2`] module*/
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2rs>;
///OTG_FS host channel-2 transfer size register
pub mod hctsiz2;
/**HCTSIZ3 (rw) register accessor: OTG_FS host channel-3 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCTSIZ3)

For information about available fields see [`mod@hctsiz3`] module*/
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3rs>;
///OTG_FS host channel-3 transfer size register
pub mod hctsiz3;
/**HCTSIZ4 (rw) register accessor: OTG_FS host channel-x transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCTSIZ4)

For information about available fields see [`mod@hctsiz4`] module*/
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4rs>;
///OTG_FS host channel-x transfer size register
pub mod hctsiz4;
/**HCTSIZ5 (rw) register accessor: OTG_FS host channel-5 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCTSIZ5)

For information about available fields see [`mod@hctsiz5`] module*/
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5rs>;
///OTG_FS host channel-5 transfer size register
pub mod hctsiz5;
/**HCTSIZ6 (rw) register accessor: OTG_FS host channel-6 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCTSIZ6)

For information about available fields see [`mod@hctsiz6`] module*/
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6rs>;
///OTG_FS host channel-6 transfer size register
pub mod hctsiz6;
/**HCTSIZ7 (rw) register accessor: OTG_FS host channel-7 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_HOST:HCTSIZ7)

For information about available fields see [`mod@hctsiz7`] module*/
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7rs>;
///OTG_FS host channel-7 transfer size register
pub mod hctsiz7;
