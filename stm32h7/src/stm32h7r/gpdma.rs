#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    privcfgr: PRIVCFGR,
    rcfglockr: RCFGLOCKR,
    misr: MISR,
    _reserved3: [u8; 0x40],
    ch: [CH; 12],
    ech: [ECH; 4],
}
impl RegisterBlock {
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
    ///0x0c - GPDMA masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x50..0x650 - Channel cluster
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x50..0x650 - Channel cluster
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0x650..0x850 - Extended channel cluster
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `ECH12` cluster.</div>
    #[inline(always)]
    pub const fn ech(&self, n: usize) -> &ECH {
        &self.ech[n]
    }
    ///Iterator for array of:
    ///0x650..0x850 - Extended channel cluster
    #[inline(always)]
    pub fn ech_iter(&self) -> impl Iterator<Item = &ECH> {
        self.ech.iter()
    }
    ///0x650..0x6d0 - Extended channel cluster
    #[inline(always)]
    pub const fn ech12(&self) -> &ECH {
        self.ech(0)
    }
    ///0x6d0..0x750 - Extended channel cluster
    #[inline(always)]
    pub const fn ech13(&self) -> &ECH {
        self.ech(1)
    }
    ///0x750..0x7d0 - Extended channel cluster
    #[inline(always)]
    pub const fn ech14(&self) -> &ECH {
        self.ech(2)
    }
    ///0x7d0..0x850 - Extended channel cluster
    #[inline(always)]
    pub const fn ech15(&self) -> &ECH {
        self.ech(3)
    }
}
/**PRIVCFGR (rw) register accessor: GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#GPDMA:PRIVCFGR)

For information about available fields see [`mod@privcfgr`]
module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///GPDMA privileged configuration register
pub mod privcfgr;
/**RCFGLOCKR (rw) register accessor: GPDMA configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#GPDMA:RCFGLOCKR)

For information about available fields see [`mod@rcfglockr`]
module*/
pub type RCFGLOCKR = crate::Reg<rcfglockr::RCFGLOCKRrs>;
///GPDMA configuration lock register
pub mod rcfglockr;
/**MISR (r) register accessor: GPDMA masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#GPDMA:MISR)

For information about available fields see [`mod@misr`]
module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///GPDMA masked interrupt status register
pub mod misr;
///Channel cluster
pub use self::ch::CH;
///Cluster
///Channel cluster
pub mod ch;
///Extended channel cluster
pub use self::ech::ECH;
///Cluster
///Extended channel cluster
pub mod ech;
