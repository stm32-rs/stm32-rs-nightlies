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
    ///0x00 - GPIO port A mode register
    #[inline(always)]
    pub const fn moder(&self) -> &MODER {
        &self.moder
    }
    ///0x04 - GPIO port A output type register
    #[inline(always)]
    pub const fn otyper(&self) -> &OTYPER {
        &self.otyper
    }
    ///0x08 - GPIO port A output speed register
    #[inline(always)]
    pub const fn ospeedr(&self) -> &OSPEEDR {
        &self.ospeedr
    }
    ///0x0c - GPIO port A pull-up/pull-down register
    #[inline(always)]
    pub const fn pupdr(&self) -> &PUPDR {
        &self.pupdr
    }
    ///0x10 - GPIO port A input data register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x14 - GPIO port A output data register
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    ///0x18 - GPIO port A bit set/reset register
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    ///0x1c - GPIO port A configuration lock register
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
    ///0x20 - GPIO port A alternate function low register
    #[inline(always)]
    pub const fn afrl(&self) -> &AFRL {
        &self.afrl
    }
    ///0x24 - GPIO port A alternate function high register
    #[inline(always)]
    pub const fn afrh(&self) -> &AFRH {
        &self.afrh
    }
    ///0x28 - GPIO port A bit reset register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x30 - GPIO port A secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x34 - GPIO port A privileged configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x38 - GPIO port A resource configuration lock register
    #[inline(always)]
    pub const fn rcfglockr(&self) -> &RCFGLOCKR {
        &self.rcfglockr
    }
    ///0x40 - GPIO port A delay low register
    #[inline(always)]
    pub const fn delayrl(&self) -> &DELAYRL {
        &self.delayrl
    }
    ///0x44 - GPIO port A delay high register
    #[inline(always)]
    pub const fn delayrh(&self) -> &DELAYRH {
        &self.delayrh
    }
    ///0x48 - GPIO port A PIO control low register
    #[inline(always)]
    pub const fn piocfgrl(&self) -> &PIOCFGRL {
        &self.piocfgrl
    }
    ///0x4c - GPIO port A PIO control high register
    #[inline(always)]
    pub const fn piocfgrh(&self) -> &PIOCFGRH {
        &self.piocfgrh
    }
    ///0x3c8 - GPIO port A hardware configuration register 10
    #[inline(always)]
    pub const fn hwcfgr10(&self) -> &HWCFGR10 {
        &self.hwcfgr10
    }
    ///0x3cc - GPIO port A hardware configuration register 9
    #[inline(always)]
    pub const fn hwcfgr9(&self) -> &HWCFGR9 {
        &self.hwcfgr9
    }
    ///0x3d0 - GPIO port A hardware configuration register 8
    #[inline(always)]
    pub const fn hwcfgr8(&self) -> &HWCFGR8 {
        &self.hwcfgr8
    }
    ///0x3d4 - GPIO port A hardware configuration register 7
    #[inline(always)]
    pub const fn hwcfgr7(&self) -> &HWCFGR7 {
        &self.hwcfgr7
    }
    ///0x3d8 - GPIO port A hardware configuration register 6
    #[inline(always)]
    pub const fn hwcfgr6(&self) -> &HWCFGR6 {
        &self.hwcfgr6
    }
    ///0x3dc - GPIO port A hardware configuration register 5
    #[inline(always)]
    pub const fn hwcfgr5(&self) -> &HWCFGR5 {
        &self.hwcfgr5
    }
    ///0x3e0 - GPIO port A hardware configuration register 4
    #[inline(always)]
    pub const fn hwcfgr4(&self) -> &HWCFGR4 {
        &self.hwcfgr4
    }
    ///0x3e4 - GPIO port A hardware configuration register 3
    #[inline(always)]
    pub const fn hwcfgr3(&self) -> &HWCFGR3 {
        &self.hwcfgr3
    }
    ///0x3e8 - GPIO port A hardware configuration register 2
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3ec - GPIO port A hardware configuration register 1
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f0 - GPIO port A hardware configuration register 0
    #[inline(always)]
    pub const fn hwcfgr0(&self) -> &HWCFGR0 {
        &self.hwcfgr0
    }
    ///0x3f4 - GPIO port A version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - GPIO port A identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - GPIO port A size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**MODER (rw) register accessor: GPIO port A mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:MODER)

For information about available fields see [`mod@moder`] module*/
pub type MODER = crate::Reg<moder::MODERrs>;
///GPIO port A mode register
pub mod moder;
/**OTYPER (rw) register accessor: GPIO port A output type register

You can [`read`](crate::Reg::read) this register and get [`otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:OTYPER)

For information about available fields see [`mod@otyper`] module*/
pub type OTYPER = crate::Reg<otyper::OTYPERrs>;
///GPIO port A output type register
pub mod otyper;
/**OSPEEDR (rw) register accessor: GPIO port A output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:OSPEEDR)

For information about available fields see [`mod@ospeedr`] module*/
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDRrs>;
///GPIO port A output speed register
pub mod ospeedr;
/**PUPDR (rw) register accessor: GPIO port A pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:PUPDR)

For information about available fields see [`mod@pupdr`] module*/
pub type PUPDR = crate::Reg<pupdr::PUPDRrs>;
///GPIO port A pull-up/pull-down register
pub mod pupdr;
/**IDR (r) register accessor: GPIO port A input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///GPIO port A input data register
pub mod idr;
/**ODR (rw) register accessor: GPIO port A output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:ODR)

For information about available fields see [`mod@odr`] module*/
pub type ODR = crate::Reg<odr::ODRrs>;
///GPIO port A output data register
pub mod odr;
/**BSRR (w) register accessor: GPIO port A bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:BSRR)

For information about available fields see [`mod@bsrr`] module*/
pub type BSRR = crate::Reg<bsrr::BSRRrs>;
///GPIO port A bit set/reset register
pub mod bsrr;
/**LCKR (rw) register accessor: GPIO port A configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:LCKR)

For information about available fields see [`mod@lckr`] module*/
pub type LCKR = crate::Reg<lckr::LCKRrs>;
///GPIO port A configuration lock register
pub mod lckr;
/**AFRL (rw) register accessor: GPIO port A alternate function low register

You can [`read`](crate::Reg::read) this register and get [`afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:AFRL)

For information about available fields see [`mod@afrl`] module*/
pub type AFRL = crate::Reg<afrl::AFRLrs>;
///GPIO port A alternate function low register
pub mod afrl;
/**AFRH (rw) register accessor: GPIO port A alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:AFRH)

For information about available fields see [`mod@afrh`] module*/
pub type AFRH = crate::Reg<afrh::AFRHrs>;
///GPIO port A alternate function high register
pub mod afrh;
/**BRR (w) register accessor: GPIO port A bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:BRR)

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///GPIO port A bit reset register
pub mod brr;
/**SECCFGR (rw) register accessor: GPIO port A secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///GPIO port A secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: GPIO port A privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///GPIO port A privileged configuration register
pub mod privcfgr;
/**RCFGLOCKR (rw) register accessor: GPIO port A resource configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:RCFGLOCKR)

For information about available fields see [`mod@rcfglockr`] module*/
pub type RCFGLOCKR = crate::Reg<rcfglockr::RCFGLOCKRrs>;
///GPIO port A resource configuration lock register
pub mod rcfglockr;
/**DELAYRL (rw) register accessor: GPIO port A delay low register

You can [`read`](crate::Reg::read) this register and get [`delayrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delayrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:DELAYRL)

For information about available fields see [`mod@delayrl`] module*/
pub type DELAYRL = crate::Reg<delayrl::DELAYRLrs>;
///GPIO port A delay low register
pub mod delayrl;
/**DELAYRH (rw) register accessor: GPIO port A delay high register

You can [`read`](crate::Reg::read) this register and get [`delayrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delayrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:DELAYRH)

For information about available fields see [`mod@delayrh`] module*/
pub type DELAYRH = crate::Reg<delayrh::DELAYRHrs>;
///GPIO port A delay high register
pub mod delayrh;
/**PIOCFGRL (rw) register accessor: GPIO port A PIO control low register

You can [`read`](crate::Reg::read) this register and get [`piocfgrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`piocfgrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:PIOCFGRL)

For information about available fields see [`mod@piocfgrl`] module*/
pub type PIOCFGRL = crate::Reg<piocfgrl::PIOCFGRLrs>;
///GPIO port A PIO control low register
pub mod piocfgrl;
/**PIOCFGRH (rw) register accessor: GPIO port A PIO control high register

You can [`read`](crate::Reg::read) this register and get [`piocfgrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`piocfgrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:PIOCFGRH)

For information about available fields see [`mod@piocfgrh`] module*/
pub type PIOCFGRH = crate::Reg<piocfgrh::PIOCFGRHrs>;
///GPIO port A PIO control high register
pub mod piocfgrh;
/**HWCFGR10 (r) register accessor: GPIO port A hardware configuration register 10

You can [`read`](crate::Reg::read) this register and get [`hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR10)

For information about available fields see [`mod@hwcfgr10`] module*/
pub type HWCFGR10 = crate::Reg<hwcfgr10::HWCFGR10rs>;
///GPIO port A hardware configuration register 10
pub mod hwcfgr10;
/**HWCFGR9 (r) register accessor: GPIO port A hardware configuration register 9

You can [`read`](crate::Reg::read) this register and get [`hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR9)

For information about available fields see [`mod@hwcfgr9`] module*/
pub type HWCFGR9 = crate::Reg<hwcfgr9::HWCFGR9rs>;
///GPIO port A hardware configuration register 9
pub mod hwcfgr9;
/**HWCFGR8 (r) register accessor: GPIO port A hardware configuration register 8

You can [`read`](crate::Reg::read) this register and get [`hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR8)

For information about available fields see [`mod@hwcfgr8`] module*/
pub type HWCFGR8 = crate::Reg<hwcfgr8::HWCFGR8rs>;
///GPIO port A hardware configuration register 8
pub mod hwcfgr8;
/**HWCFGR7 (r) register accessor: GPIO port A hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR7)

For information about available fields see [`mod@hwcfgr7`] module*/
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7rs>;
///GPIO port A hardware configuration register 7
pub mod hwcfgr7;
/**HWCFGR6 (r) register accessor: GPIO port A hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR6)

For information about available fields see [`mod@hwcfgr6`] module*/
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
///GPIO port A hardware configuration register 6
pub mod hwcfgr6;
/**HWCFGR5 (r) register accessor: GPIO port A hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR5)

For information about available fields see [`mod@hwcfgr5`] module*/
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
///GPIO port A hardware configuration register 5
pub mod hwcfgr5;
/**HWCFGR4 (r) register accessor: GPIO port A hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR4)

For information about available fields see [`mod@hwcfgr4`] module*/
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
///GPIO port A hardware configuration register 4
pub mod hwcfgr4;
/**HWCFGR3 (r) register accessor: GPIO port A hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR3)

For information about available fields see [`mod@hwcfgr3`] module*/
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
///GPIO port A hardware configuration register 3
pub mod hwcfgr3;
/**HWCFGR2 (r) register accessor: GPIO port A hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///GPIO port A hardware configuration register 2
pub mod hwcfgr2;
/**HWCFGR1 (r) register accessor: GPIO port A hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///GPIO port A hardware configuration register 1
pub mod hwcfgr1;
/**HWCFGR0 (r) register accessor: GPIO port A hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:HWCFGR0)

For information about available fields see [`mod@hwcfgr0`] module*/
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0rs>;
///GPIO port A hardware configuration register 0
pub mod hwcfgr0;
/**VERR (r) register accessor: GPIO port A version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///GPIO port A version register
pub mod verr;
/**IPIDR (r) register accessor: GPIO port A identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///GPIO port A identification register
pub mod ipidr;
/**SIDR (r) register accessor: GPIO port A size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOA:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///GPIO port A size identification register
pub mod sidr;
