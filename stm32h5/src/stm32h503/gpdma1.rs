#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    privcfgr: PRIVCFGR,
    _reserved1: [u8; 0x04],
    misr: MISR,
    _reserved2: [u8; 0x40],
    ch: [CH; 6],
    ch2d: [CH2D; 2],
}
impl RegisterBlock {
    ///0x04 - GPDMA privileged configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x0c - GPDMA masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
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
/**PRIVCFGR (rw) register accessor: GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GPDMA1:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///GPDMA privileged configuration register
pub mod privcfgr;
/**MISR (r) register accessor: GPDMA masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GPDMA1:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///GPDMA masked interrupt status register
pub mod misr;
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
