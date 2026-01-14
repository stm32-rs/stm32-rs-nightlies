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
    ///0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\] is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\] must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset.A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence.Each lock bit freezes a specific configuration register (control and alternate function registers).
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
}
/**MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#GPIOC:MODER)

For information about available fields see [`mod@moder`] module*/
pub type MODER = crate::Reg<moder::MODERrs>;
///GPIO port mode register
pub mod moder;
pub use crate::stm32h747cm4::gpioa::otyper;
pub use crate::stm32h747cm4::gpioa::OTYPER;
/**OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#GPIOC:OSPEEDR)

For information about available fields see [`mod@ospeedr`] module*/
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDRrs>;
///GPIO port output speed register
pub mod ospeedr;
/**PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#GPIOC:PUPDR)

For information about available fields see [`mod@pupdr`] module*/
pub type PUPDR = crate::Reg<pupdr::PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod pupdr;
pub use crate::stm32h747cm4::gpioa::afrh;
pub use crate::stm32h747cm4::gpioa::afrl;
pub use crate::stm32h747cm4::gpioa::bsrr;
pub use crate::stm32h747cm4::gpioa::idr;
pub use crate::stm32h747cm4::gpioa::lckr;
pub use crate::stm32h747cm4::gpioa::odr;
pub use crate::stm32h747cm4::gpioa::AFRH;
pub use crate::stm32h747cm4::gpioa::AFRL;
pub use crate::stm32h747cm4::gpioa::BSRR;
pub use crate::stm32h747cm4::gpioa::IDR;
pub use crate::stm32h747cm4::gpioa::LCKR;
pub use crate::stm32h747cm4::gpioa::ODR;
