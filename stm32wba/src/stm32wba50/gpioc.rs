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
    _reserved8: [u8; 0x04],
    afrh: AFRH,
    brr: BRR,
    _reserved10: [u8; 0x04],
    seccfgr: SECCFGR,
}
impl RegisterBlock {
    ///0x00 - GPIO port C mode register
    #[inline(always)]
    pub const fn moder(&self) -> &MODER {
        &self.moder
    }
    ///0x04 - GPIO port C output type register
    #[inline(always)]
    pub const fn otyper(&self) -> &OTYPER {
        &self.otyper
    }
    ///0x08 - GPIOC port output speed register
    #[inline(always)]
    pub const fn ospeedr(&self) -> &OSPEEDR {
        &self.ospeedr
    }
    ///0x0c - GPIO port C pull-up/pull-down register
    #[inline(always)]
    pub const fn pupdr(&self) -> &PUPDR {
        &self.pupdr
    }
    ///0x10 - GPIO port C input data register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x14 - GPIO port C output data register
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    ///0x18 - GPIO port C bit set/reset register
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    ///0x1c - GPIO port C configuration lock register
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
    ///0x24 - GPIO port C alternate function high register
    #[inline(always)]
    pub const fn afrh(&self) -> &AFRH {
        &self.afrh
    }
    ///0x28 - GPIO port C bit reset register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x30 - GPIO port C secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
}
/**MODER (rw) register accessor: GPIO port C mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:MODER)

For information about available fields see [`mod@moder`] module*/
pub type MODER = crate::Reg<moder::MODERrs>;
///GPIO port C mode register
pub mod moder;
/**OTYPER (rw) register accessor: GPIO port C output type register

You can [`read`](crate::Reg::read) this register and get [`otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:OTYPER)

For information about available fields see [`mod@otyper`] module*/
pub type OTYPER = crate::Reg<otyper::OTYPERrs>;
///GPIO port C output type register
pub mod otyper;
/**OSPEEDR (rw) register accessor: GPIOC port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:OSPEEDR)

For information about available fields see [`mod@ospeedr`] module*/
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDRrs>;
///GPIOC port output speed register
pub mod ospeedr;
/**PUPDR (rw) register accessor: GPIO port C pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:PUPDR)

For information about available fields see [`mod@pupdr`] module*/
pub type PUPDR = crate::Reg<pupdr::PUPDRrs>;
///GPIO port C pull-up/pull-down register
pub mod pupdr;
/**IDR (r) register accessor: GPIO port C input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///GPIO port C input data register
pub mod idr;
/**ODR (rw) register accessor: GPIO port C output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:ODR)

For information about available fields see [`mod@odr`] module*/
pub type ODR = crate::Reg<odr::ODRrs>;
///GPIO port C output data register
pub mod odr;
/**BSRR (w) register accessor: GPIO port C bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:BSRR)

For information about available fields see [`mod@bsrr`] module*/
pub type BSRR = crate::Reg<bsrr::BSRRrs>;
///GPIO port C bit set/reset register
pub mod bsrr;
/**LCKR (rw) register accessor: GPIO port C configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:LCKR)

For information about available fields see [`mod@lckr`] module*/
pub type LCKR = crate::Reg<lckr::LCKRrs>;
///GPIO port C configuration lock register
pub mod lckr;
/**AFRH (rw) register accessor: GPIO port C alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:AFRH)

For information about available fields see [`mod@afrh`] module*/
pub type AFRH = crate::Reg<afrh::AFRHrs>;
///GPIO port C alternate function high register
pub mod afrh;
/**BRR (w) register accessor: GPIO port C bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:BRR)

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///GPIO port C bit reset register
pub mod brr;
/**SECCFGR (w) register accessor: GPIO port C secure configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///GPIO port C secure configuration register
pub mod seccfgr;
