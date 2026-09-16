#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    tzsc_privcfgr1: TZSC_PRIVCFGR1,
    tzsc_privcfgr2: TZSC_PRIVCFGR2,
    tzsc_privcfgr3: TZSC_PRIVCFGR3,
    _reserved3: [u8; 0x44],
    tzsc_mpcwm4acfgr: TZSC_MPCWM4ACFGR,
    tzsc_mpcwm4ar: TZSC_MPCWM4AR,
    tzsc_mpcwm4bcfgr: TZSC_MPCWM4BCFGR,
    tzsc_mpcwm4br: TZSC_MPCWM4BR,
    _reserved7: [u8; 0x0180],
    mpcbb1_privcfgr: [MPCBB1_PRIVCFGR; 32],
    _reserved8: [u8; 0x0380],
    mpcbb2_privcfgr: [MPCBB2_PRIVCFGR; 32],
}
impl RegisterBlock {
    ///0x20 - GTZC1 TZSC privilege configuration register 1
    #[inline(always)]
    pub const fn tzsc_privcfgr1(&self) -> &TZSC_PRIVCFGR1 {
        &self.tzsc_privcfgr1
    }
    ///0x24 - GTZC1 TZSC privilege configuration register 2
    #[inline(always)]
    pub const fn tzsc_privcfgr2(&self) -> &TZSC_PRIVCFGR2 {
        &self.tzsc_privcfgr2
    }
    ///0x28 - GTZC1 TZSC privilege configuration register 3
    #[inline(always)]
    pub const fn tzsc_privcfgr3(&self) -> &TZSC_PRIVCFGR3 {
        &self.tzsc_privcfgr3
    }
    ///0x70 - GTZC1 TZSC BKPSRAM sub-region A watermark configuration register
    #[inline(always)]
    pub const fn tzsc_mpcwm4acfgr(&self) -> &TZSC_MPCWM4ACFGR {
        &self.tzsc_mpcwm4acfgr
    }
    ///0x74 - GTZC1 TZSC BKPSRAM sub-region A watermark register
    #[inline(always)]
    pub const fn tzsc_mpcwm4ar(&self) -> &TZSC_MPCWM4AR {
        &self.tzsc_mpcwm4ar
    }
    ///0x78 - GTZC1 TZSC BKPSRAM sub-region B watermark configuration register
    #[inline(always)]
    pub const fn tzsc_mpcwm4bcfgr(&self) -> &TZSC_MPCWM4BCFGR {
        &self.tzsc_mpcwm4bcfgr
    }
    ///0x7c - GTZC1 TZSC BKPSRAM sub-region B watermark register
    #[inline(always)]
    pub const fn tzsc_mpcwm4br(&self) -> &TZSC_MPCWM4BR {
        &self.tzsc_mpcwm4br
    }
    ///0x200..0x280 - SRAM1 MPCBB privileged configuration for super-block %s register
    #[inline(always)]
    pub const fn mpcbb1_privcfgr(&self, n: usize) -> &MPCBB1_PRIVCFGR {
        &self.mpcbb1_privcfgr[n]
    }
    ///Iterator for array of:
    ///0x200..0x280 - SRAM1 MPCBB privileged configuration for super-block %s register
    #[inline(always)]
    pub fn mpcbb1_privcfgr_iter(&self) -> impl Iterator<Item = &MPCBB1_PRIVCFGR> {
        self.mpcbb1_privcfgr.iter()
    }
    ///0x600..0x680 - SRAM2 MPCBB privileged configuration for super-block %s register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr(&self, n: usize) -> &MPCBB2_PRIVCFGR {
        &self.mpcbb2_privcfgr[n]
    }
    ///Iterator for array of:
    ///0x600..0x680 - SRAM2 MPCBB privileged configuration for super-block %s register
    #[inline(always)]
    pub fn mpcbb2_privcfgr_iter(&self) -> impl Iterator<Item = &MPCBB2_PRIVCFGR> {
        self.mpcbb2_privcfgr.iter()
    }
}
/**TZSC_PRIVCFGR1 (rw) register accessor: GTZC1 TZSC privilege configuration register 1

You can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_PRIVCFGR1)

For information about available fields see [`mod@tzsc_privcfgr1`] module*/
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1rs>;
///GTZC1 TZSC privilege configuration register 1
pub mod tzsc_privcfgr1;
/**TZSC_PRIVCFGR2 (rw) register accessor: GTZC1 TZSC privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_PRIVCFGR2)

For information about available fields see [`mod@tzsc_privcfgr2`] module*/
pub type TZSC_PRIVCFGR2 = crate::Reg<tzsc_privcfgr2::TZSC_PRIVCFGR2rs>;
///GTZC1 TZSC privilege configuration register 2
pub mod tzsc_privcfgr2;
/**TZSC_PRIVCFGR3 (rw) register accessor: GTZC1 TZSC privilege configuration register 3

You can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_PRIVCFGR3)

For information about available fields see [`mod@tzsc_privcfgr3`] module*/
pub type TZSC_PRIVCFGR3 = crate::Reg<tzsc_privcfgr3::TZSC_PRIVCFGR3rs>;
///GTZC1 TZSC privilege configuration register 3
pub mod tzsc_privcfgr3;
/**TZSC_MPCWM4ACFGR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region A watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_MPCWM4ACFGR)

For information about available fields see [`mod@tzsc_mpcwm4acfgr`] module*/
pub type TZSC_MPCWM4ACFGR = crate::Reg<tzsc_mpcwm4acfgr::TZSC_MPCWM4ACFGRrs>;
///GTZC1 TZSC BKPSRAM sub-region A watermark configuration register
pub mod tzsc_mpcwm4acfgr;
/**TZSC_MPCWM4AR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region A watermark register

You can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_MPCWM4AR)

For information about available fields see [`mod@tzsc_mpcwm4ar`] module*/
pub type TZSC_MPCWM4AR = crate::Reg<tzsc_mpcwm4ar::TZSC_MPCWM4ARrs>;
///GTZC1 TZSC BKPSRAM sub-region A watermark register
pub mod tzsc_mpcwm4ar;
/**TZSC_MPCWM4BCFGR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region B watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_MPCWM4BCFGR)

For information about available fields see [`mod@tzsc_mpcwm4bcfgr`] module*/
pub type TZSC_MPCWM4BCFGR = crate::Reg<tzsc_mpcwm4bcfgr::TZSC_MPCWM4BCFGRrs>;
///GTZC1 TZSC BKPSRAM sub-region B watermark configuration register
pub mod tzsc_mpcwm4bcfgr;
/**TZSC_MPCWM4BR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region B watermark register

You can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_MPCWM4BR)

For information about available fields see [`mod@tzsc_mpcwm4br`] module*/
pub type TZSC_MPCWM4BR = crate::Reg<tzsc_mpcwm4br::TZSC_MPCWM4BRrs>;
///GTZC1 TZSC BKPSRAM sub-region B watermark register
pub mod tzsc_mpcwm4br;
/**MPCBB1_PRIVCFGR (rw) register accessor: SRAM1 MPCBB privileged configuration for super-block %s register

You can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:MPCBB1_PRIVCFGR[0])

For information about available fields see [`mod@mpcbb1_privcfgr`] module*/
pub type MPCBB1_PRIVCFGR = crate::Reg<mpcbb1_privcfgr::MPCBB1_PRIVCFGRrs>;
///SRAM1 MPCBB privileged configuration for super-block %s register
pub mod mpcbb1_privcfgr;
/**MPCBB2_PRIVCFGR (rw) register accessor: SRAM2 MPCBB privileged configuration for super-block %s register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:MPCBB2_PRIVCFGR[0])

For information about available fields see [`mod@mpcbb2_privcfgr`] module*/
pub type MPCBB2_PRIVCFGR = crate::Reg<mpcbb2_privcfgr::MPCBB2_PRIVCFGRrs>;
///SRAM2 MPCBB privileged configuration for super-block %s register
pub mod mpcbb2_privcfgr;
