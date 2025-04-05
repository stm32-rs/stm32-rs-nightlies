#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    comp_sr: COMP_SR,
    comp_icfr: COMP_ICFR,
    _reserved2: [u8; 0x04],
    comp_cfgr1: COMP_CFGR1,
    comp_cfgr2: COMP_CFGR2,
}
impl RegisterBlock {
    ///0x00 - Comparator status register
    #[inline(always)]
    pub const fn comp_sr(&self) -> &COMP_SR {
        &self.comp_sr
    }
    ///0x04 - Comparator interrupt clear flag register
    #[inline(always)]
    pub const fn comp_icfr(&self) -> &COMP_ICFR {
        &self.comp_icfr
    }
    ///0x0c - Comparator configuration register 1
    #[inline(always)]
    pub const fn comp_cfgr1(&self) -> &COMP_CFGR1 {
        &self.comp_cfgr1
    }
    ///0x10 - Comparator configuration register 2
    #[inline(always)]
    pub const fn comp_cfgr2(&self) -> &COMP_CFGR2 {
        &self.comp_cfgr2
    }
}
/**COMP_SR (r) register accessor: Comparator status register

You can [`read`](crate::Reg::read) this register and get [`comp_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#COMP:COMP_SR)

For information about available fields see [`mod@comp_sr`] module*/
pub type COMP_SR = crate::Reg<comp_sr::COMP_SRrs>;
///Comparator status register
pub mod comp_sr;
/**COMP_ICFR (rw) register accessor: Comparator interrupt clear flag register

You can [`read`](crate::Reg::read) this register and get [`comp_icfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_icfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#COMP:COMP_ICFR)

For information about available fields see [`mod@comp_icfr`] module*/
pub type COMP_ICFR = crate::Reg<comp_icfr::COMP_ICFRrs>;
///Comparator interrupt clear flag register
pub mod comp_icfr;
/**COMP_CFGR1 (rw) register accessor: Comparator configuration register 1

You can [`read`](crate::Reg::read) this register and get [`comp_cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#COMP:COMP_CFGR1)

For information about available fields see [`mod@comp_cfgr1`] module*/
pub type COMP_CFGR1 = crate::Reg<comp_cfgr1::COMP_CFGR1rs>;
///Comparator configuration register 1
pub mod comp_cfgr1;
/**COMP_CFGR2 (rw) register accessor: Comparator configuration register 2

You can [`read`](crate::Reg::read) this register and get [`comp_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#COMP:COMP_CFGR2)

For information about available fields see [`mod@comp_cfgr2`] module*/
pub type COMP_CFGR2 = crate::Reg<comp_cfgr2::COMP_CFGR2rs>;
///Comparator configuration register 2
pub mod comp_cfgr2;
