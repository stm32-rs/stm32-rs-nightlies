#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpioa_moder: GPIOA_MODER,
    gpioa_otyper: GPIOA_OTYPER,
    gpioa_ospeedr: GPIOA_OSPEEDR,
    gpioa_pupdr: GPIOA_PUPDR,
    gpioa_idr: GPIOA_IDR,
    gpioa_odr: GPIOA_ODR,
    gpioa_bsrr: GPIOA_BSRR,
    gpioa_lckr: GPIOA_LCKR,
    gpioa_afrl: GPIOA_AFRL,
    gpioa_afrh: GPIOA_AFRH,
    gpioa_brr: GPIOA_BRR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpioa_moder(&self) -> &GPIOA_MODER {
        &self.gpioa_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpioa_otyper(&self) -> &GPIOA_OTYPER {
        &self.gpioa_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpioa_ospeedr(&self) -> &GPIOA_OSPEEDR {
        &self.gpioa_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpioa_pupdr(&self) -> &GPIOA_PUPDR {
        &self.gpioa_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpioa_idr(&self) -> &GPIOA_IDR {
        &self.gpioa_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpioa_odr(&self) -> &GPIOA_ODR {
        &self.gpioa_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpioa_bsrr(&self) -> &GPIOA_BSRR {
        &self.gpioa_bsrr
    }
    ///0x1c - GPIO port configuration lock register
    #[inline(always)]
    pub const fn gpioa_lckr(&self) -> &GPIOA_LCKR {
        &self.gpioa_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpioa_afrl(&self) -> &GPIOA_AFRL {
        &self.gpioa_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpioa_afrh(&self) -> &GPIOA_AFRH {
        &self.gpioa_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpioa_brr(&self) -> &GPIOA_BRR {
        &self.gpioa_brr
    }
}
/**GPIOA_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioa_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_MODER)

For information about available fields see [`mod@gpioa_moder`] module*/
pub type GPIOA_MODER = crate::Reg<gpioa_moder::GPIOA_MODERrs>;
///GPIO port mode register
pub mod gpioa_moder;
/**GPIOA_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpioa_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_OTYPER)

For information about available fields see [`mod@gpioa_otyper`] module*/
pub type GPIOA_OTYPER = crate::Reg<gpioa_otyper::GPIOA_OTYPERrs>;
///GPIO port output type register
pub mod gpioa_otyper;
/**GPIOA_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioa_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_OSPEEDR)

For information about available fields see [`mod@gpioa_ospeedr`] module*/
pub type GPIOA_OSPEEDR = crate::Reg<gpioa_ospeedr::GPIOA_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpioa_ospeedr;
/**GPIOA_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioa_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_PUPDR)

For information about available fields see [`mod@gpioa_pupdr`] module*/
pub type GPIOA_PUPDR = crate::Reg<gpioa_pupdr::GPIOA_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpioa_pupdr;
/**GPIOA_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioa_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_IDR)

For information about available fields see [`mod@gpioa_idr`] module*/
pub type GPIOA_IDR = crate::Reg<gpioa_idr::GPIOA_IDRrs>;
///GPIO port input data register
pub mod gpioa_idr;
/**GPIOA_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpioa_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_ODR)

For information about available fields see [`mod@gpioa_odr`] module*/
pub type GPIOA_ODR = crate::Reg<gpioa_odr::GPIOA_ODRrs>;
///GPIO port output data register
pub mod gpioa_odr;
/**GPIOA_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_BSRR)

For information about available fields see [`mod@gpioa_bsrr`] module*/
pub type GPIOA_BSRR = crate::Reg<gpioa_bsrr::GPIOA_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpioa_bsrr;
/**GPIOA_LCKR (rw) register accessor: GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`gpioa_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_LCKR)

For information about available fields see [`mod@gpioa_lckr`] module*/
pub type GPIOA_LCKR = crate::Reg<gpioa_lckr::GPIOA_LCKRrs>;
///GPIO port configuration lock register
pub mod gpioa_lckr;
/**GPIOA_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioa_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_AFRL)

For information about available fields see [`mod@gpioa_afrl`] module*/
pub type GPIOA_AFRL = crate::Reg<gpioa_afrl::GPIOA_AFRLrs>;
///GPIO alternate function low register
pub mod gpioa_afrl;
/**GPIOA_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioa_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_AFRH)

For information about available fields see [`mod@gpioa_afrh`] module*/
pub type GPIOA_AFRH = crate::Reg<gpioa_afrh::GPIOA_AFRHrs>;
///GPIO alternate function high register
pub mod gpioa_afrh;
/**GPIOA_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOA:GPIOA_BRR)

For information about available fields see [`mod@gpioa_brr`] module*/
pub type GPIOA_BRR = crate::Reg<gpioa_brr::GPIOA_BRRrs>;
///GPIO port bit reset register
pub mod gpioa_brr;
