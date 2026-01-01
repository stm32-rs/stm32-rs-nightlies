#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpiod_moder: GPIOD_MODER,
    gpiod_otyper: GPIOD_OTYPER,
    gpiod_ospeedr: GPIOD_OSPEEDR,
    gpiod_pupdr: GPIOD_PUPDR,
    gpiod_idr: GPIOD_IDR,
    gpiod_odr: GPIOD_ODR,
    gpiod_bsrr: GPIOD_BSRR,
    gpiod_lckr: GPIOD_LCKR,
    gpiod_afrl: GPIOD_AFRL,
    gpiod_afrh: GPIOD_AFRH,
    gpiod_brr: GPIOD_BRR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpiod_moder(&self) -> &GPIOD_MODER {
        &self.gpiod_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpiod_otyper(&self) -> &GPIOD_OTYPER {
        &self.gpiod_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpiod_ospeedr(&self) -> &GPIOD_OSPEEDR {
        &self.gpiod_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpiod_pupdr(&self) -> &GPIOD_PUPDR {
        &self.gpiod_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpiod_idr(&self) -> &GPIOD_IDR {
        &self.gpiod_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpiod_odr(&self) -> &GPIOD_ODR {
        &self.gpiod_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpiod_bsrr(&self) -> &GPIOD_BSRR {
        &self.gpiod_bsrr
    }
    ///0x1c - GPIO port configuration lock register
    #[inline(always)]
    pub const fn gpiod_lckr(&self) -> &GPIOD_LCKR {
        &self.gpiod_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpiod_afrl(&self) -> &GPIOD_AFRL {
        &self.gpiod_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpiod_afrh(&self) -> &GPIOD_AFRH {
        &self.gpiod_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpiod_brr(&self) -> &GPIOD_BRR {
        &self.gpiod_brr
    }
}
/**GPIOD_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpiod_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_MODER)

For information about available fields see [`mod@gpiod_moder`] module*/
pub type GPIOD_MODER = crate::Reg<gpiod_moder::GPIOD_MODERrs>;
///GPIO port mode register
pub mod gpiod_moder;
/**GPIOD_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpiod_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_OTYPER)

For information about available fields see [`mod@gpiod_otyper`] module*/
pub type GPIOD_OTYPER = crate::Reg<gpiod_otyper::GPIOD_OTYPERrs>;
///GPIO port output type register
pub mod gpiod_otyper;
/**GPIOD_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpiod_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_OSPEEDR)

For information about available fields see [`mod@gpiod_ospeedr`] module*/
pub type GPIOD_OSPEEDR = crate::Reg<gpiod_ospeedr::GPIOD_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpiod_ospeedr;
/**GPIOD_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpiod_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_PUPDR)

For information about available fields see [`mod@gpiod_pupdr`] module*/
pub type GPIOD_PUPDR = crate::Reg<gpiod_pupdr::GPIOD_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpiod_pupdr;
/**GPIOD_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpiod_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_IDR)

For information about available fields see [`mod@gpiod_idr`] module*/
pub type GPIOD_IDR = crate::Reg<gpiod_idr::GPIOD_IDRrs>;
///GPIO port input data register
pub mod gpiod_idr;
/**GPIOD_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpiod_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_ODR)

For information about available fields see [`mod@gpiod_odr`] module*/
pub type GPIOD_ODR = crate::Reg<gpiod_odr::GPIOD_ODRrs>;
///GPIO port output data register
pub mod gpiod_odr;
/**GPIOD_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_BSRR)

For information about available fields see [`mod@gpiod_bsrr`] module*/
pub type GPIOD_BSRR = crate::Reg<gpiod_bsrr::GPIOD_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpiod_bsrr;
/**GPIOD_LCKR (rw) register accessor: GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`gpiod_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_LCKR)

For information about available fields see [`mod@gpiod_lckr`] module*/
pub type GPIOD_LCKR = crate::Reg<gpiod_lckr::GPIOD_LCKRrs>;
///GPIO port configuration lock register
pub mod gpiod_lckr;
/**GPIOD_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpiod_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_AFRL)

For information about available fields see [`mod@gpiod_afrl`] module*/
pub type GPIOD_AFRL = crate::Reg<gpiod_afrl::GPIOD_AFRLrs>;
///GPIO alternate function low register
pub mod gpiod_afrl;
/**GPIOD_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpiod_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_AFRH)

For information about available fields see [`mod@gpiod_afrh`] module*/
pub type GPIOD_AFRH = crate::Reg<gpiod_afrh::GPIOD_AFRHrs>;
///GPIO alternate function high register
pub mod gpiod_afrh;
/**GPIOD_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOD:GPIOD_BRR)

For information about available fields see [`mod@gpiod_brr`] module*/
pub type GPIOD_BRR = crate::Reg<gpiod_brr::GPIOD_BRRrs>;
///GPIO port bit reset register
pub mod gpiod_brr;
