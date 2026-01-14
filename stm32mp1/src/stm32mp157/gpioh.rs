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
    _reserved11: [u8; 0x039c],
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
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn moder(&self) -> &MODER {
        &self.moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn otyper(&self) -> &OTYPER {
        &self.otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn ospeedr(&self) -> &OSPEEDR {
        &self.ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn pupdr(&self) -> &PUPDR {
        &self.pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    ///0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\] is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\] must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn afrl(&self) -> &AFRL {
        &self.afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn afrh(&self) -> &AFRH {
        &self.afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn hwcfgr10(&self) -> &HWCFGR10 {
        &self.hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn hwcfgr9(&self) -> &HWCFGR9 {
        &self.hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn hwcfgr8(&self) -> &HWCFGR8 {
        &self.hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn hwcfgr7(&self) -> &HWCFGR7 {
        &self.hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn hwcfgr6(&self) -> &HWCFGR6 {
        &self.hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn hwcfgr5(&self) -> &HWCFGR5 {
        &self.hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn hwcfgr4(&self) -> &HWCFGR4 {
        &self.hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn hwcfgr3(&self) -> &HWCFGR3 {
        &self.hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn hwcfgr0(&self) -> &HWCFGR0 {
        &self.hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:MODER)

For information about available fields see [`mod@moder`] module*/
pub type MODER = crate::Reg<moder::MODERrs>;
///GPIO port mode register
pub mod moder;
pub use crate::stm32mp157::gpioa::otyper;
pub use crate::stm32mp157::gpioa::OTYPER;
/**OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:OSPEEDR)

For information about available fields see [`mod@ospeedr`] module*/
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDRrs>;
///GPIO port output speed register
pub mod ospeedr;
/**PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:PUPDR)

For information about available fields see [`mod@pupdr`] module*/
pub type PUPDR = crate::Reg<pupdr::PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod pupdr;
pub use crate::stm32mp157::gpioa::afrh;
pub use crate::stm32mp157::gpioa::afrl;
pub use crate::stm32mp157::gpioa::brr;
pub use crate::stm32mp157::gpioa::bsrr;
pub use crate::stm32mp157::gpioa::idr;
pub use crate::stm32mp157::gpioa::lckr;
pub use crate::stm32mp157::gpioa::odr;
pub use crate::stm32mp157::gpioa::AFRH;
pub use crate::stm32mp157::gpioa::AFRL;
pub use crate::stm32mp157::gpioa::BRR;
pub use crate::stm32mp157::gpioa::BSRR;
pub use crate::stm32mp157::gpioa::IDR;
pub use crate::stm32mp157::gpioa::LCKR;
pub use crate::stm32mp157::gpioa::ODR;
/**HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR10)

For information about available fields see [`mod@hwcfgr10`] module*/
pub type HWCFGR10 = crate::Reg<hwcfgr10::HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod hwcfgr10;
/**HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR9)

For information about available fields see [`mod@hwcfgr9`] module*/
pub type HWCFGR9 = crate::Reg<hwcfgr9::HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod hwcfgr9;
/**HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR8)

For information about available fields see [`mod@hwcfgr8`] module*/
pub type HWCFGR8 = crate::Reg<hwcfgr8::HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod hwcfgr8;
/**HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR7)

For information about available fields see [`mod@hwcfgr7`] module*/
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod hwcfgr7;
/**HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR6)

For information about available fields see [`mod@hwcfgr6`] module*/
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod hwcfgr6;
/**HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR5)

For information about available fields see [`mod@hwcfgr5`] module*/
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod hwcfgr5;
/**HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR4)

For information about available fields see [`mod@hwcfgr4`] module*/
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod hwcfgr4;
/**HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR3)

For information about available fields see [`mod@hwcfgr3`] module*/
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod hwcfgr3;
/**HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod hwcfgr2;
/**HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod hwcfgr1;
/**HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:HWCFGR0)

For information about available fields see [`mod@hwcfgr0`] module*/
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod hwcfgr0;
/**VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///GPIO version register
pub mod verr;
/**IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///GPIO identification register
pub mod ipidr;
/**SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///GPIO size identification register
pub mod sidr;
