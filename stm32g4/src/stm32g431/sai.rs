#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ch: [CH; 2],
    pdmcr: PDMCR,
    pdmdly: PDMDLY,
}
impl RegisterBlock {
    ///0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CHA` cluster.</div>
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0x04..0x24 - Cluster CHA, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    #[inline(always)]
    pub const fn cha(&self) -> &CH {
        self.ch(0)
    }
    ///0x24..0x44 - Cluster CHB, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    #[inline(always)]
    pub const fn chb(&self) -> &CH {
        self.ch(1)
    }
    ///0x44 - PDM control register
    #[inline(always)]
    pub const fn pdmcr(&self) -> &PDMCR {
        &self.pdmcr
    }
    ///0x48 - PDM delay register
    #[inline(always)]
    pub const fn pdmdly(&self) -> &PDMDLY {
        &self.pdmdly
    }
}
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub mod ch;
/**PDMCR (rw) register accessor: PDM control register

You can [`read`](crate::Reg::read) this register and get [`pdmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#SAI:PDMCR)

For information about available fields see [`mod@pdmcr`] module*/
pub type PDMCR = crate::Reg<pdmcr::PDMCRrs>;
///PDM control register
pub mod pdmcr;
/**PDMDLY (rw) register accessor: PDM delay register

You can [`read`](crate::Reg::read) this register and get [`pdmdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#SAI:PDMDLY)

For information about available fields see [`mod@pdmdly`] module*/
pub type PDMDLY = crate::Reg<pdmdly::PDMDLYrs>;
///PDM delay register
pub mod pdmdly;
