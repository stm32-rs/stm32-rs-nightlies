#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    seccfgr1: SECCFGR1,
    seccfgr2: SECCFGR2,
    _reserved3: [u8; 0x08],
    privcfgr1: PRIVCFGR1,
    privcfgr2: PRIVCFGR2,
    _reserved5: [u8; 0x08],
    mpcwm1_nswmr1: MPCWM1_NSWMR1,
    mpcwm1_nswmr2: MPCWM1_NSWMR2,
    mpcwm2_nswmr1: MPCWM2_NSWMR1,
    mpcwm2_nswmr2: MPCWM2_NSWMR2,
    mpcwm3_nswmr1: MPCWM3_NSWMR1,
}
impl RegisterBlock {
    ///0x00 - TZSC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - TZSC secure configuration register 1
    #[inline(always)]
    pub const fn seccfgr1(&self) -> &SECCFGR1 {
        &self.seccfgr1
    }
    ///0x14 - TZSC secure configuration register 2
    #[inline(always)]
    pub const fn seccfgr2(&self) -> &SECCFGR2 {
        &self.seccfgr2
    }
    ///0x20 - TZSC privilege configuration register 1
    #[inline(always)]
    pub const fn privcfgr1(&self) -> &PRIVCFGR1 {
        &self.privcfgr1
    }
    ///0x24 - TZSC privilege configuration register 2
    #[inline(always)]
    pub const fn privcfgr2(&self) -> &PRIVCFGR2 {
        &self.privcfgr2
    }
    ///0x30 - TZSC external memory non-secure watermark register 1
    #[inline(always)]
    pub const fn mpcwm1_nswmr1(&self) -> &MPCWM1_NSWMR1 {
        &self.mpcwm1_nswmr1
    }
    ///0x34 - TZSC external memory non-secure watermark register 1
    #[inline(always)]
    pub const fn mpcwm1_nswmr2(&self) -> &MPCWM1_NSWMR2 {
        &self.mpcwm1_nswmr2
    }
    ///0x38 - TZSC external memory non-secure watermark register 1
    #[inline(always)]
    pub const fn mpcwm2_nswmr1(&self) -> &MPCWM2_NSWMR1 {
        &self.mpcwm2_nswmr1
    }
    ///0x3c - TZSC external memory non-secure watermark register 2
    #[inline(always)]
    pub const fn mpcwm2_nswmr2(&self) -> &MPCWM2_NSWMR2 {
        &self.mpcwm2_nswmr2
    }
    ///0x40 - TZSC external memory non-secure watermark register 2
    #[inline(always)]
    pub const fn mpcwm3_nswmr1(&self) -> &MPCWM3_NSWMR1 {
        &self.mpcwm3_nswmr1
    }
}
/**CR (rw) register accessor: TZSC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///TZSC control register
pub mod cr;
/**SECCFGR1 (rw) register accessor: TZSC secure configuration register 1

You can [`read`](crate::Reg::read) this register and get [`seccfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:SECCFGR1)

For information about available fields see [`mod@seccfgr1`] module*/
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1rs>;
///TZSC secure configuration register 1
pub mod seccfgr1;
/**SECCFGR2 (rw) register accessor: TZSC secure configuration register 2

You can [`read`](crate::Reg::read) this register and get [`seccfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:SECCFGR2)

For information about available fields see [`mod@seccfgr2`] module*/
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2rs>;
///TZSC secure configuration register 2
pub mod seccfgr2;
/**PRIVCFGR1 (rw) register accessor: TZSC privilege configuration register 1

You can [`read`](crate::Reg::read) this register and get [`privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:PRIVCFGR1)

For information about available fields see [`mod@privcfgr1`] module*/
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1rs>;
///TZSC privilege configuration register 1
pub mod privcfgr1;
/**PRIVCFGR2 (rw) register accessor: TZSC privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:PRIVCFGR2)

For information about available fields see [`mod@privcfgr2`] module*/
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2rs>;
///TZSC privilege configuration register 2
pub mod privcfgr2;
/**MPCWM1_NSWMR1 (rw) register accessor: TZSC external memory non-secure watermark register 1

You can [`read`](crate::Reg::read) this register and get [`mpcwm1_nswmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1_nswmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:MPCWM1_NSWMR1)

For information about available fields see [`mod@mpcwm1_nswmr1`] module*/
pub type MPCWM1_NSWMR1 = crate::Reg<mpcwm1_nswmr1::MPCWM1_NSWMR1rs>;
///TZSC external memory non-secure watermark register 1
pub mod mpcwm1_nswmr1;
/**MPCWM1_NSWMR2 (rw) register accessor: TZSC external memory non-secure watermark register 1

You can [`read`](crate::Reg::read) this register and get [`mpcwm1_nswmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1_nswmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:MPCWM1_NSWMR2)

For information about available fields see [`mod@mpcwm1_nswmr2`] module*/
pub type MPCWM1_NSWMR2 = crate::Reg<mpcwm1_nswmr2::MPCWM1_NSWMR2rs>;
///TZSC external memory non-secure watermark register 1
pub mod mpcwm1_nswmr2;
/**MPCWM2_NSWMR1 (rw) register accessor: TZSC external memory non-secure watermark register 1

You can [`read`](crate::Reg::read) this register and get [`mpcwm2_nswmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm2_nswmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:MPCWM2_NSWMR1)

For information about available fields see [`mod@mpcwm2_nswmr1`] module*/
pub type MPCWM2_NSWMR1 = crate::Reg<mpcwm2_nswmr1::MPCWM2_NSWMR1rs>;
///TZSC external memory non-secure watermark register 1
pub mod mpcwm2_nswmr1;
/**MPCWM3_NSWMR1 (rw) register accessor: TZSC external memory non-secure watermark register 2

You can [`read`](crate::Reg::read) this register and get [`mpcwm3_nswmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm3_nswmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:MPCWM3_NSWMR1)

For information about available fields see [`mod@mpcwm3_nswmr1`] module*/
pub type MPCWM3_NSWMR1 = crate::Reg<mpcwm3_nswmr1::MPCWM3_NSWMR1rs>;
///TZSC external memory non-secure watermark register 2
pub mod mpcwm3_nswmr1;
/**MPCWM2_NSWMR2 (rw) register accessor: TZSC external memory non-secure watermark register 2

You can [`read`](crate::Reg::read) this register and get [`mpcwm2_nswmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm2_nswmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZSC:MPCWM2_NSWMR2)

For information about available fields see [`mod@mpcwm2_nswmr2`] module*/
pub type MPCWM2_NSWMR2 = crate::Reg<mpcwm2_nswmr2::MPCWM2_NSWMR2rs>;
///TZSC external memory non-secure watermark register 2
pub mod mpcwm2_nswmr2;
