#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    seccfgr1: SECCFGR1,
    seccfgr2: SECCFGR2,
    seccfgr3: SECCFGR3,
    _reserved4: [u8; 0x04],
    privcfgr1: PRIVCFGR1,
    privcfgr2: PRIVCFGR2,
    privcfgr3: PRIVCFGR3,
    _reserved7: [u8; 0x14],
    mpcwm1acfgr: MPCWM1ACFGR,
    mpcwm1ar: MPCWM1AR,
    mpcwm1bcfgr: MPCWM1BCFGR,
    mpcwm1br: MPCWM1BR,
    mpcwm2acfgr: MPCWM2ACFGR,
    mpcwm2ar: MPCWM2AR,
    mpcwm2bcfgr: MPCWM2BCFGR,
    mpcwm2br: MPCWM2BR,
    mpcwm3acfgr: MPCWM3ACFGR,
    mpcwm3ar: MPCWM3AR,
    mpcwm3bcfgr: MPCWM3BCFGR,
    mpcwm3br: MPCWM3BR,
    mpcwm4acfgr: MPCWM4ACFGR,
    mpcwm4ar: MPCWM4AR,
    mpcwm4bcfgr: MPCWM4BCFGR,
    mpcwm4br: MPCWM4BR,
}
impl RegisterBlock {
    ///0x00 - GTZC1 TZSC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - GTZC1 TZSC secure configuration register 1
    #[inline(always)]
    pub const fn seccfgr1(&self) -> &SECCFGR1 {
        &self.seccfgr1
    }
    ///0x14 - GTZC1 TZSC secure configuration register 2
    #[inline(always)]
    pub const fn seccfgr2(&self) -> &SECCFGR2 {
        &self.seccfgr2
    }
    ///0x18 - GTZC1 TZSC secure configuration register 3
    #[inline(always)]
    pub const fn seccfgr3(&self) -> &SECCFGR3 {
        &self.seccfgr3
    }
    ///0x20 - GTZC1 TZSC privilege configuration register 1
    #[inline(always)]
    pub const fn privcfgr1(&self) -> &PRIVCFGR1 {
        &self.privcfgr1
    }
    ///0x24 - GTZC1 TZSC privilege configuration register 2
    #[inline(always)]
    pub const fn privcfgr2(&self) -> &PRIVCFGR2 {
        &self.privcfgr2
    }
    ///0x28 - GTZC1 TZSC privilege configuration register 3
    #[inline(always)]
    pub const fn privcfgr3(&self) -> &PRIVCFGR3 {
        &self.privcfgr3
    }
    ///0x40 - GTZC1 TZSC memory 1 subregion A watermark configuration register
    #[inline(always)]
    pub const fn mpcwm1acfgr(&self) -> &MPCWM1ACFGR {
        &self.mpcwm1acfgr
    }
    ///0x44 - GTZC1 TZSC memory 1 subregion A watermark register
    #[inline(always)]
    pub const fn mpcwm1ar(&self) -> &MPCWM1AR {
        &self.mpcwm1ar
    }
    ///0x48 - GTZC1 TZSC memory 1 subregion B watermark configuration register
    #[inline(always)]
    pub const fn mpcwm1bcfgr(&self) -> &MPCWM1BCFGR {
        &self.mpcwm1bcfgr
    }
    ///0x4c - GTZC1 TZSC memory 1 subregion B watermark register
    #[inline(always)]
    pub const fn mpcwm1br(&self) -> &MPCWM1BR {
        &self.mpcwm1br
    }
    ///0x50 - GTZC1 TZSC memory 2 subregion A watermark configuration register
    #[inline(always)]
    pub const fn mpcwm2acfgr(&self) -> &MPCWM2ACFGR {
        &self.mpcwm2acfgr
    }
    ///0x54 - GTZC1 TZSC memory 2 subregion A watermark register
    #[inline(always)]
    pub const fn mpcwm2ar(&self) -> &MPCWM2AR {
        &self.mpcwm2ar
    }
    ///0x58 - GTZC1 TZSC memory 2 subregion B watermark configuration register
    #[inline(always)]
    pub const fn mpcwm2bcfgr(&self) -> &MPCWM2BCFGR {
        &self.mpcwm2bcfgr
    }
    ///0x5c - GTZC1 TZSC memory 2 subregion B watermark register
    #[inline(always)]
    pub const fn mpcwm2br(&self) -> &MPCWM2BR {
        &self.mpcwm2br
    }
    ///0x60 - GTZC1 TZSC memory 3 subregion A watermark configuration register
    #[inline(always)]
    pub const fn mpcwm3acfgr(&self) -> &MPCWM3ACFGR {
        &self.mpcwm3acfgr
    }
    ///0x64 - GTZC1 TZSC memory 3 subregion A watermark register
    #[inline(always)]
    pub const fn mpcwm3ar(&self) -> &MPCWM3AR {
        &self.mpcwm3ar
    }
    ///0x68 - GTZC1 TZSC memory 3 subregion B watermark configuration register
    #[inline(always)]
    pub const fn mpcwm3bcfgr(&self) -> &MPCWM3BCFGR {
        &self.mpcwm3bcfgr
    }
    ///0x6c - GTZC1 TZSC memory 3 subregion B watermark register
    #[inline(always)]
    pub const fn mpcwm3br(&self) -> &MPCWM3BR {
        &self.mpcwm3br
    }
    ///0x70 - GTZC1 TZSC memory 4 subregion A watermark configuration register
    #[inline(always)]
    pub const fn mpcwm4acfgr(&self) -> &MPCWM4ACFGR {
        &self.mpcwm4acfgr
    }
    ///0x74 - GTZC1 TZSC memory 4 subregion A watermark register
    #[inline(always)]
    pub const fn mpcwm4ar(&self) -> &MPCWM4AR {
        &self.mpcwm4ar
    }
    ///0x78 - GTZC1 TZSC memory 4 subregion B watermark configuration register
    #[inline(always)]
    pub const fn mpcwm4bcfgr(&self) -> &MPCWM4BCFGR {
        &self.mpcwm4bcfgr
    }
    ///0x7c - GTZC1 TZSC memory 4 subregion B watermark register
    #[inline(always)]
    pub const fn mpcwm4br(&self) -> &MPCWM4BR {
        &self.mpcwm4br
    }
}
/**CR (rw) register accessor: GTZC1 TZSC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///GTZC1 TZSC control register
pub mod cr;
/**SECCFGR1 (rw) register accessor: GTZC1 TZSC secure configuration register 1

You can [`read`](crate::Reg::read) this register and get [`seccfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:SECCFGR1)

For information about available fields see [`mod@seccfgr1`] module*/
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1rs>;
///GTZC1 TZSC secure configuration register 1
pub mod seccfgr1;
/**SECCFGR2 (rw) register accessor: GTZC1 TZSC secure configuration register 2

You can [`read`](crate::Reg::read) this register and get [`seccfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:SECCFGR2)

For information about available fields see [`mod@seccfgr2`] module*/
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2rs>;
///GTZC1 TZSC secure configuration register 2
pub mod seccfgr2;
/**SECCFGR3 (rw) register accessor: GTZC1 TZSC secure configuration register 3

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:SECCFGR3)

For information about available fields see [`mod@seccfgr3`] module*/
pub type SECCFGR3 = crate::Reg<seccfgr3::SECCFGR3rs>;
///GTZC1 TZSC secure configuration register 3
pub mod seccfgr3;
/**PRIVCFGR1 (rw) register accessor: GTZC1 TZSC privilege configuration register 1

You can [`read`](crate::Reg::read) this register and get [`privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:PRIVCFGR1)

For information about available fields see [`mod@privcfgr1`] module*/
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1rs>;
///GTZC1 TZSC privilege configuration register 1
pub mod privcfgr1;
/**PRIVCFGR2 (rw) register accessor: GTZC1 TZSC privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:PRIVCFGR2)

For information about available fields see [`mod@privcfgr2`] module*/
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2rs>;
///GTZC1 TZSC privilege configuration register 2
pub mod privcfgr2;
/**PRIVCFGR3 (rw) register accessor: GTZC1 TZSC privilege configuration register 3

You can [`read`](crate::Reg::read) this register and get [`privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:PRIVCFGR3)

For information about available fields see [`mod@privcfgr3`] module*/
pub type PRIVCFGR3 = crate::Reg<privcfgr3::PRIVCFGR3rs>;
///GTZC1 TZSC privilege configuration register 3
pub mod privcfgr3;
/**MPCWM1ACFGR (rw) register accessor: GTZC1 TZSC memory 1 subregion A watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm1acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM1ACFGR)

For information about available fields see [`mod@mpcwm1acfgr`] module*/
pub type MPCWM1ACFGR = crate::Reg<mpcwm1acfgr::MPCWM1ACFGRrs>;
///GTZC1 TZSC memory 1 subregion A watermark configuration register
pub mod mpcwm1acfgr;
/**MPCWM1AR (rw) register accessor: GTZC1 TZSC memory 1 subregion A watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM1AR)

For information about available fields see [`mod@mpcwm1ar`] module*/
pub type MPCWM1AR = crate::Reg<mpcwm1ar::MPCWM1ARrs>;
///GTZC1 TZSC memory 1 subregion A watermark register
pub mod mpcwm1ar;
/**MPCWM1BCFGR (rw) register accessor: GTZC1 TZSC memory 1 subregion B watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm1bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM1BCFGR)

For information about available fields see [`mod@mpcwm1bcfgr`] module*/
pub type MPCWM1BCFGR = crate::Reg<mpcwm1bcfgr::MPCWM1BCFGRrs>;
///GTZC1 TZSC memory 1 subregion B watermark configuration register
pub mod mpcwm1bcfgr;
/**MPCWM1BR (rw) register accessor: GTZC1 TZSC memory 1 subregion B watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm1br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM1BR)

For information about available fields see [`mod@mpcwm1br`] module*/
pub type MPCWM1BR = crate::Reg<mpcwm1br::MPCWM1BRrs>;
///GTZC1 TZSC memory 1 subregion B watermark register
pub mod mpcwm1br;
/**MPCWM2ACFGR (rw) register accessor: GTZC1 TZSC memory 2 subregion A watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm2acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm2acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM2ACFGR)

For information about available fields see [`mod@mpcwm2acfgr`] module*/
pub type MPCWM2ACFGR = crate::Reg<mpcwm2acfgr::MPCWM2ACFGRrs>;
///GTZC1 TZSC memory 2 subregion A watermark configuration register
pub mod mpcwm2acfgr;
/**MPCWM2AR (rw) register accessor: GTZC1 TZSC memory 2 subregion A watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm2ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm2ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM2AR)

For information about available fields see [`mod@mpcwm2ar`] module*/
pub type MPCWM2AR = crate::Reg<mpcwm2ar::MPCWM2ARrs>;
///GTZC1 TZSC memory 2 subregion A watermark register
pub mod mpcwm2ar;
/**MPCWM2BCFGR (rw) register accessor: GTZC1 TZSC memory 2 subregion B watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm2bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm2bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM2BCFGR)

For information about available fields see [`mod@mpcwm2bcfgr`] module*/
pub type MPCWM2BCFGR = crate::Reg<mpcwm2bcfgr::MPCWM2BCFGRrs>;
///GTZC1 TZSC memory 2 subregion B watermark configuration register
pub mod mpcwm2bcfgr;
/**MPCWM2BR (rw) register accessor: GTZC1 TZSC memory 2 subregion B watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm2br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm2br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM2BR)

For information about available fields see [`mod@mpcwm2br`] module*/
pub type MPCWM2BR = crate::Reg<mpcwm2br::MPCWM2BRrs>;
///GTZC1 TZSC memory 2 subregion B watermark register
pub mod mpcwm2br;
/**MPCWM3ACFGR (rw) register accessor: GTZC1 TZSC memory 3 subregion A watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm3acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm3acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM3ACFGR)

For information about available fields see [`mod@mpcwm3acfgr`] module*/
pub type MPCWM3ACFGR = crate::Reg<mpcwm3acfgr::MPCWM3ACFGRrs>;
///GTZC1 TZSC memory 3 subregion A watermark configuration register
pub mod mpcwm3acfgr;
/**MPCWM3AR (rw) register accessor: GTZC1 TZSC memory 3 subregion A watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm3ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm3ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM3AR)

For information about available fields see [`mod@mpcwm3ar`] module*/
pub type MPCWM3AR = crate::Reg<mpcwm3ar::MPCWM3ARrs>;
///GTZC1 TZSC memory 3 subregion A watermark register
pub mod mpcwm3ar;
/**MPCWM3BCFGR (rw) register accessor: GTZC1 TZSC memory 3 subregion B watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm3bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm3bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM3BCFGR)

For information about available fields see [`mod@mpcwm3bcfgr`] module*/
pub type MPCWM3BCFGR = crate::Reg<mpcwm3bcfgr::MPCWM3BCFGRrs>;
///GTZC1 TZSC memory 3 subregion B watermark configuration register
pub mod mpcwm3bcfgr;
/**MPCWM3BR (rw) register accessor: GTZC1 TZSC memory 3 subregion B watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm3br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm3br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM3BR)

For information about available fields see [`mod@mpcwm3br`] module*/
pub type MPCWM3BR = crate::Reg<mpcwm3br::MPCWM3BRrs>;
///GTZC1 TZSC memory 3 subregion B watermark register
pub mod mpcwm3br;
/**MPCWM4ACFGR (rw) register accessor: GTZC1 TZSC memory 4 subregion A watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm4acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm4acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM4ACFGR)

For information about available fields see [`mod@mpcwm4acfgr`] module*/
pub type MPCWM4ACFGR = crate::Reg<mpcwm4acfgr::MPCWM4ACFGRrs>;
///GTZC1 TZSC memory 4 subregion A watermark configuration register
pub mod mpcwm4acfgr;
/**MPCWM4AR (rw) register accessor: GTZC1 TZSC memory 4 subregion A watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm4ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm4ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM4AR)

For information about available fields see [`mod@mpcwm4ar`] module*/
pub type MPCWM4AR = crate::Reg<mpcwm4ar::MPCWM4ARrs>;
///GTZC1 TZSC memory 4 subregion A watermark register
pub mod mpcwm4ar;
/**MPCWM4BCFGR (rw) register accessor: GTZC1 TZSC memory 4 subregion B watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm4bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm4bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM4BCFGR)

For information about available fields see [`mod@mpcwm4bcfgr`] module*/
pub type MPCWM4BCFGR = crate::Reg<mpcwm4bcfgr::MPCWM4BCFGRrs>;
///GTZC1 TZSC memory 4 subregion B watermark configuration register
pub mod mpcwm4bcfgr;
/**MPCWM4BR (rw) register accessor: GTZC1 TZSC memory 4 subregion B watermark register

You can [`read`](crate::Reg::read) this register and get [`mpcwm4br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm4br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:MPCWM4BR)

For information about available fields see [`mod@mpcwm4br`] module*/
pub type MPCWM4BR = crate::Reg<mpcwm4br::MPCWM4BRrs>;
///GTZC1 TZSC memory 4 subregion B watermark register
pub mod mpcwm4br;
