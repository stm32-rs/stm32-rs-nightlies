#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    moder: MODER,
    otyper: OTYPER,
    ospeedr: OSPEEDR,
    pupdr: PUPDR,
    idr: IDR,
    odr: ODR,
    bsrr: BSRR,
    lckr: LCKR,
    afrl: AFRL,
    afrh: AFRH,
    brr: BRR,
    _reserved11: [u8; 0x04],
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
    rcfglockr: RCFGLOCKR,
    _reserved14: [u8; 0x04],
    delayrl: DELAYRL,
    delayrh: DELAYRH,
    piocfgrl: PIOCFGRL,
    piocfgrh: PIOCFGRH,
    _reserved18: [u8; 0x0378],
    hwcfgr10: HWCFGR10,
    hwcfgr9: HWCFGR9,
    hwcfgr8: HWCFGR8,
    hwcfgr7: HWCFGR7,
    hwcfgr6: HWCFGR6,
    hwcfgr5: HWCFGR5,
    hwcfgr4: HWCFGR4,
    hwcfgr3: HWCFGR3,
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    hwcfgr0: HWCFGR0,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - GPIO port H mode register
    #[inline(always)]
    pub const fn moder(&self) -> &MODER {
        &self.moder
    }
    ///0x04 - GPIO port H output type register
    #[inline(always)]
    pub const fn otyper(&self) -> &OTYPER {
        &self.otyper
    }
    ///0x08 - GPIO port H output speed register
    #[inline(always)]
    pub const fn ospeedr(&self) -> &OSPEEDR {
        &self.ospeedr
    }
    ///0x0c - GPIO port H pull-up/pull-down register
    #[inline(always)]
    pub const fn pupdr(&self) -> &PUPDR {
        &self.pupdr
    }
    ///0x10 - GPIO port H input data register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x14 - GPIO port H output data register
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    ///0x18 - GPIO port H bit set/reset register
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    ///0x1c - GPIO port H configuration lock register
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
    ///0x20 - GPIO port H alternate function low register
    #[inline(always)]
    pub const fn afrl(&self) -> &AFRL {
        &self.afrl
    }
    ///0x24 - GPIO port H alternate function high register
    #[inline(always)]
    pub const fn afrh(&self) -> &AFRH {
        &self.afrh
    }
    ///0x28 - GPIO port H bit reset register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x30 - GPIO port H secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x34 - GPIO port H privileged configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x38 - GPIO port H resource configuration lock register
    #[inline(always)]
    pub const fn rcfglockr(&self) -> &RCFGLOCKR {
        &self.rcfglockr
    }
    ///0x40 - GPIO port H delay low register
    #[inline(always)]
    pub const fn delayrl(&self) -> &DELAYRL {
        &self.delayrl
    }
    ///0x44 - GPIO port H delay high register
    #[inline(always)]
    pub const fn delayrh(&self) -> &DELAYRH {
        &self.delayrh
    }
    ///0x48 - GPIO port H PIO control low register
    #[inline(always)]
    pub const fn piocfgrl(&self) -> &PIOCFGRL {
        &self.piocfgrl
    }
    ///0x4c - GPIO port H PIO control high register
    #[inline(always)]
    pub const fn piocfgrh(&self) -> &PIOCFGRH {
        &self.piocfgrh
    }
    ///0x3c8 - GPIO port H hardware configuration register 10
    #[inline(always)]
    pub const fn hwcfgr10(&self) -> &HWCFGR10 {
        &self.hwcfgr10
    }
    ///0x3cc - GPIO port H hardware configuration register 9
    #[inline(always)]
    pub const fn hwcfgr9(&self) -> &HWCFGR9 {
        &self.hwcfgr9
    }
    ///0x3d0 - GPIO port H hardware configuration register 8
    #[inline(always)]
    pub const fn hwcfgr8(&self) -> &HWCFGR8 {
        &self.hwcfgr8
    }
    ///0x3d4 - GPIO port H hardware configuration register 7
    #[inline(always)]
    pub const fn hwcfgr7(&self) -> &HWCFGR7 {
        &self.hwcfgr7
    }
    ///0x3d8 - GPIO port H hardware configuration register 6
    #[inline(always)]
    pub const fn hwcfgr6(&self) -> &HWCFGR6 {
        &self.hwcfgr6
    }
    ///0x3dc - GPIO port H hardware configuration register 5
    #[inline(always)]
    pub const fn hwcfgr5(&self) -> &HWCFGR5 {
        &self.hwcfgr5
    }
    ///0x3e0 - GPIO port H hardware configuration register 4
    #[inline(always)]
    pub const fn hwcfgr4(&self) -> &HWCFGR4 {
        &self.hwcfgr4
    }
    ///0x3e4 - GPIO port H hardware configuration register 3
    #[inline(always)]
    pub const fn hwcfgr3(&self) -> &HWCFGR3 {
        &self.hwcfgr3
    }
    ///0x3e8 - GPIO port H hardware configuration register 2
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3ec - GPIO port H hardware configuration register 1
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f0 - GPIO port H hardware configuration register 0
    #[inline(always)]
    pub const fn hwcfgr0(&self) -> &HWCFGR0 {
        &self.hwcfgr0
    }
    ///0x3f4 - GPIO port H version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - GPIO port H identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - GPIO port H size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**MODER (rw) register accessor: GPIO port H mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:MODER)

For information about available fields see [`mod@moder`] module*/
pub type MODER = crate::Reg<moder::MODERrs>;
///GPIO port H mode register
pub mod moder;
pub use crate::stm32n645::gpioa::otyper;
pub use crate::stm32n645::gpioa::OTYPER;
/**OSPEEDR (rw) register accessor: GPIO port H output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:OSPEEDR)

For information about available fields see [`mod@ospeedr`] module*/
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDRrs>;
///GPIO port H output speed register
pub mod ospeedr;
/**PUPDR (rw) register accessor: GPIO port H pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:PUPDR)

For information about available fields see [`mod@pupdr`] module*/
pub type PUPDR = crate::Reg<pupdr::PUPDRrs>;
///GPIO port H pull-up/pull-down register
pub mod pupdr;
pub use crate::stm32n645::gpioa::afrh;
pub use crate::stm32n645::gpioa::afrl;
pub use crate::stm32n645::gpioa::brr;
pub use crate::stm32n645::gpioa::bsrr;
pub use crate::stm32n645::gpioa::idr;
pub use crate::stm32n645::gpioa::lckr;
pub use crate::stm32n645::gpioa::odr;
pub use crate::stm32n645::gpioa::AFRH;
pub use crate::stm32n645::gpioa::AFRL;
pub use crate::stm32n645::gpioa::BRR;
pub use crate::stm32n645::gpioa::BSRR;
pub use crate::stm32n645::gpioa::IDR;
pub use crate::stm32n645::gpioa::LCKR;
pub use crate::stm32n645::gpioa::ODR;
/**SECCFGR (rw) register accessor: GPIO port H secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///GPIO port H secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: GPIO port H privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///GPIO port H privileged configuration register
pub mod privcfgr;
/**RCFGLOCKR (rw) register accessor: GPIO port H resource configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:RCFGLOCKR)

For information about available fields see [`mod@rcfglockr`] module*/
pub type RCFGLOCKR = crate::Reg<rcfglockr::RCFGLOCKRrs>;
///GPIO port H resource configuration lock register
pub mod rcfglockr;
/**DELAYRL (rw) register accessor: GPIO port H delay low register

You can [`read`](crate::Reg::read) this register and get [`delayrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delayrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:DELAYRL)

For information about available fields see [`mod@delayrl`] module*/
pub type DELAYRL = crate::Reg<delayrl::DELAYRLrs>;
///GPIO port H delay low register
pub mod delayrl;
/**DELAYRH (rw) register accessor: GPIO port H delay high register

You can [`read`](crate::Reg::read) this register and get [`delayrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delayrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:DELAYRH)

For information about available fields see [`mod@delayrh`] module*/
pub type DELAYRH = crate::Reg<delayrh::DELAYRHrs>;
///GPIO port H delay high register
pub mod delayrh;
/**PIOCFGRL (rw) register accessor: GPIO port H PIO control low register

You can [`read`](crate::Reg::read) this register and get [`piocfgrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`piocfgrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:PIOCFGRL)

For information about available fields see [`mod@piocfgrl`] module*/
pub type PIOCFGRL = crate::Reg<piocfgrl::PIOCFGRLrs>;
///GPIO port H PIO control low register
pub mod piocfgrl;
/**PIOCFGRH (rw) register accessor: GPIO port H PIO control high register

You can [`read`](crate::Reg::read) this register and get [`piocfgrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`piocfgrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:PIOCFGRH)

For information about available fields see [`mod@piocfgrh`] module*/
pub type PIOCFGRH = crate::Reg<piocfgrh::PIOCFGRHrs>;
///GPIO port H PIO control high register
pub mod piocfgrh;
/**HWCFGR10 (r) register accessor: GPIO port H hardware configuration register 10

You can [`read`](crate::Reg::read) this register and get [`hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR10)

For information about available fields see [`mod@hwcfgr10`] module*/
pub type HWCFGR10 = crate::Reg<hwcfgr10::HWCFGR10rs>;
///GPIO port H hardware configuration register 10
pub mod hwcfgr10;
/**HWCFGR9 (r) register accessor: GPIO port H hardware configuration register 9

You can [`read`](crate::Reg::read) this register and get [`hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR9)

For information about available fields see [`mod@hwcfgr9`] module*/
pub type HWCFGR9 = crate::Reg<hwcfgr9::HWCFGR9rs>;
///GPIO port H hardware configuration register 9
pub mod hwcfgr9;
/**HWCFGR8 (r) register accessor: GPIO port H hardware configuration register 8

You can [`read`](crate::Reg::read) this register and get [`hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR8)

For information about available fields see [`mod@hwcfgr8`] module*/
pub type HWCFGR8 = crate::Reg<hwcfgr8::HWCFGR8rs>;
///GPIO port H hardware configuration register 8
pub mod hwcfgr8;
/**HWCFGR7 (r) register accessor: GPIO port H hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR7)

For information about available fields see [`mod@hwcfgr7`] module*/
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7rs>;
///GPIO port H hardware configuration register 7
pub mod hwcfgr7;
/**HWCFGR6 (r) register accessor: GPIO port H hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR6)

For information about available fields see [`mod@hwcfgr6`] module*/
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
///GPIO port H hardware configuration register 6
pub mod hwcfgr6;
/**HWCFGR5 (r) register accessor: GPIO port H hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR5)

For information about available fields see [`mod@hwcfgr5`] module*/
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
///GPIO port H hardware configuration register 5
pub mod hwcfgr5;
/**HWCFGR4 (r) register accessor: GPIO port H hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR4)

For information about available fields see [`mod@hwcfgr4`] module*/
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
///GPIO port H hardware configuration register 4
pub mod hwcfgr4;
/**HWCFGR3 (r) register accessor: GPIO port H hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR3)

For information about available fields see [`mod@hwcfgr3`] module*/
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
///GPIO port H hardware configuration register 3
pub mod hwcfgr3;
/**HWCFGR2 (r) register accessor: GPIO port H hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///GPIO port H hardware configuration register 2
pub mod hwcfgr2;
/**HWCFGR1 (r) register accessor: GPIO port H hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///GPIO port H hardware configuration register 1
pub mod hwcfgr1;
/**HWCFGR0 (r) register accessor: GPIO port H hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:HWCFGR0)

For information about available fields see [`mod@hwcfgr0`] module*/
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0rs>;
///GPIO port H hardware configuration register 0
pub mod hwcfgr0;
/**VERR (r) register accessor: GPIO port H version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///GPIO port H version register
pub mod verr;
/**IPIDR (r) register accessor: GPIO port H identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///GPIO port H identification register
pub mod ipidr;
/**SIDR (r) register accessor: GPIO port H size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOH:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///GPIO port H size identification register
pub mod sidr;
