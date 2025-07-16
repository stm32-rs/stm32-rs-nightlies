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
}
impl RegisterBlock {
    ///0x00 - MODER register
    #[inline(always)]
    pub const fn moder(&self) -> &MODER {
        &self.moder
    }
    ///0x04 - OTYPER register
    #[inline(always)]
    pub const fn otyper(&self) -> &OTYPER {
        &self.otyper
    }
    ///0x08 - OSPEEDR register
    #[inline(always)]
    pub const fn ospeedr(&self) -> &OSPEEDR {
        &self.ospeedr
    }
    ///0x0c - PUPDR register
    #[inline(always)]
    pub const fn pupdr(&self) -> &PUPDR {
        &self.pupdr
    }
    ///0x10 - IDR register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x14 - ODR register
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    ///0x18 - BSRR register
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    ///0x1c - LCKR register
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
    ///0x20 - AFRL register
    #[inline(always)]
    pub const fn afrl(&self) -> &AFRL {
        &self.afrl
    }
    ///0x24 - AFRH register
    #[inline(always)]
    pub const fn afrh(&self) -> &AFRH {
        &self.afrh
    }
    ///0x28 - BRR register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
}
/**MODER (rw) register accessor: MODER register

You can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#GPIOB:MODER)

For information about available fields see [`mod@moder`] module*/
pub type MODER = crate::Reg<moder::MODERrs>;
///MODER register
pub mod moder;
pub use crate::stm32wb07::gpioa::otyper;
pub use crate::stm32wb07::gpioa::OTYPER;
/**OSPEEDR (rw) register accessor: OSPEEDR register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#GPIOB:OSPEEDR)

For information about available fields see [`mod@ospeedr`] module*/
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDRrs>;
///OSPEEDR register
pub mod ospeedr;
/**PUPDR (rw) register accessor: PUPDR register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#GPIOB:PUPDR)

For information about available fields see [`mod@pupdr`] module*/
pub type PUPDR = crate::Reg<pupdr::PUPDRrs>;
///PUPDR register
pub mod pupdr;
pub use crate::stm32wb07::gpioa::afrh;
pub use crate::stm32wb07::gpioa::afrl;
pub use crate::stm32wb07::gpioa::brr;
pub use crate::stm32wb07::gpioa::bsrr;
pub use crate::stm32wb07::gpioa::idr;
pub use crate::stm32wb07::gpioa::lckr;
pub use crate::stm32wb07::gpioa::odr;
pub use crate::stm32wb07::gpioa::AFRH;
pub use crate::stm32wb07::gpioa::AFRL;
pub use crate::stm32wb07::gpioa::BRR;
pub use crate::stm32wb07::gpioa::BSRR;
pub use crate::stm32wb07::gpioa::IDR;
pub use crate::stm32wb07::gpioa::LCKR;
pub use crate::stm32wb07::gpioa::ODR;
