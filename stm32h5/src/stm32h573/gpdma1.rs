#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
    rcfglockr: RCFGLOCKR,
    misr: MISR,
    smisr: SMISR,
    _reserved5: [u8; 0x3c],
    ch: [CH; 6],
    ch2d: [CH2D; 2],
}
impl RegisterBlock {
    ///0x00 - GPDMA secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x04 - GPDMA privileged configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x08 - GPDMA configuration lock register
    #[inline(always)]
    pub const fn rcfglockr(&self) -> &RCFGLOCKR {
        &self.rcfglockr
    }
    ///0x0c - GPDMA non-secure masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x10 - GPDMA secure masked interrupt status register
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    ///0x50..0x350 - Channel cluster
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x50..0x350 - Channel cluster
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0x350..0x450 - 2D-addressing channel cluster
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CH2D6` cluster.</div>
    #[inline(always)]
    pub const fn ch2d(&self, n: usize) -> &CH2D {
        &self.ch2d[n]
    }
    ///Iterator for array of:
    ///0x350..0x450 - 2D-addressing channel cluster
    #[inline(always)]
    pub fn ch2d_iter(&self) -> impl Iterator<Item = &CH2D> {
        self.ch2d.iter()
    }
    ///0x350..0x3d0 - 2D-addressing channel cluster
    #[inline(always)]
    pub const fn ch2d6(&self) -> &CH2D {
        self.ch2d(0)
    }
    ///0x3d0..0x450 - 2D-addressing channel cluster
    #[inline(always)]
    pub const fn ch2d7(&self) -> &CH2D {
        self.ch2d(1)
    }
}
/**SECCFGR (rw) register accessor: GPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GPDMA1:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///GPDMA secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GPDMA1:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///GPDMA privileged configuration register
pub mod privcfgr;
/**RCFGLOCKR (rw) register accessor: GPDMA configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GPDMA1:RCFGLOCKR)

For information about available fields see [`mod@rcfglockr`] module*/
pub type RCFGLOCKR = crate::Reg<rcfglockr::RCFGLOCKRrs>;
///GPDMA configuration lock register
pub mod rcfglockr;
/**MISR (r) register accessor: GPDMA non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GPDMA1:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///GPDMA non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: GPDMA secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GPDMA1:SMISR)

For information about available fields see [`mod@smisr`] module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///GPDMA secure masked interrupt status register
pub mod smisr;
///Channel cluster
pub use self::ch::CH;
///Cluster
///Channel cluster
pub mod ch;
///2D-addressing channel cluster
pub use self::ch2d::CH2D;
///Cluster
///2D-addressing channel cluster
pub mod ch2d;
