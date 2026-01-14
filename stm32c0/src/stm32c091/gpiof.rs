#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpiof_moder: GPIOF_MODER,
    gpiof_otyper: GPIOF_OTYPER,
    gpiof_ospeedr: GPIOF_OSPEEDR,
    gpiof_pupdr: GPIOF_PUPDR,
    gpiof_idr: GPIOF_IDR,
    gpiof_odr: GPIOF_ODR,
    gpiof_bsrr: GPIOF_BSRR,
    gpiof_lckr: GPIOF_LCKR,
    gpiof_afrl: GPIOF_AFRL,
    gpiof_afrh: GPIOF_AFRH,
    gpiof_brr: GPIOF_BRR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpiof_moder(&self) -> &GPIOF_MODER {
        &self.gpiof_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpiof_otyper(&self) -> &GPIOF_OTYPER {
        &self.gpiof_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpiof_ospeedr(&self) -> &GPIOF_OSPEEDR {
        &self.gpiof_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpiof_pupdr(&self) -> &GPIOF_PUPDR {
        &self.gpiof_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpiof_idr(&self) -> &GPIOF_IDR {
        &self.gpiof_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpiof_odr(&self) -> &GPIOF_ODR {
        &self.gpiof_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpiof_bsrr(&self) -> &GPIOF_BSRR {
        &self.gpiof_bsrr
    }
    ///0x1c - GPIO port configuration lock register
    #[inline(always)]
    pub const fn gpiof_lckr(&self) -> &GPIOF_LCKR {
        &self.gpiof_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpiof_afrl(&self) -> &GPIOF_AFRL {
        &self.gpiof_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpiof_afrh(&self) -> &GPIOF_AFRH {
        &self.gpiof_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpiof_brr(&self) -> &GPIOF_BRR {
        &self.gpiof_brr
    }
}
/**GPIOF_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpiof_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_MODER)

For information about available fields see [`mod@gpiof_moder`] module*/
pub type GPIOF_MODER = crate::Reg<gpiof_moder::GPIOF_MODERrs>;
///GPIO port mode register
pub mod gpiof_moder;
/**GPIOF_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpiof_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_OTYPER)

For information about available fields see [`mod@gpiof_otyper`] module*/
pub type GPIOF_OTYPER = crate::Reg<gpiof_otyper::GPIOF_OTYPERrs>;
///GPIO port output type register
pub mod gpiof_otyper;
/**GPIOF_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpiof_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_OSPEEDR)

For information about available fields see [`mod@gpiof_ospeedr`] module*/
pub type GPIOF_OSPEEDR = crate::Reg<gpiof_ospeedr::GPIOF_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpiof_ospeedr;
/**GPIOF_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpiof_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_PUPDR)

For information about available fields see [`mod@gpiof_pupdr`] module*/
pub type GPIOF_PUPDR = crate::Reg<gpiof_pupdr::GPIOF_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpiof_pupdr;
/**GPIOF_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpiof_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_IDR)

For information about available fields see [`mod@gpiof_idr`] module*/
pub type GPIOF_IDR = crate::Reg<gpiof_idr::GPIOF_IDRrs>;
///GPIO port input data register
pub mod gpiof_idr;
/**GPIOF_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpiof_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_ODR)

For information about available fields see [`mod@gpiof_odr`] module*/
pub type GPIOF_ODR = crate::Reg<gpiof_odr::GPIOF_ODRrs>;
///GPIO port output data register
pub mod gpiof_odr;
/**GPIOF_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_BSRR)

For information about available fields see [`mod@gpiof_bsrr`] module*/
pub type GPIOF_BSRR = crate::Reg<gpiof_bsrr::GPIOF_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpiof_bsrr;
/**GPIOF_LCKR (rw) register accessor: GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`gpiof_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_LCKR)

For information about available fields see [`mod@gpiof_lckr`] module*/
pub type GPIOF_LCKR = crate::Reg<gpiof_lckr::GPIOF_LCKRrs>;
///GPIO port configuration lock register
pub mod gpiof_lckr;
/**GPIOF_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpiof_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_AFRL)

For information about available fields see [`mod@gpiof_afrl`] module*/
pub type GPIOF_AFRL = crate::Reg<gpiof_afrl::GPIOF_AFRLrs>;
///GPIO alternate function low register
pub mod gpiof_afrl;
/**GPIOF_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpiof_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_AFRH)

For information about available fields see [`mod@gpiof_afrh`] module*/
pub type GPIOF_AFRH = crate::Reg<gpiof_afrh::GPIOF_AFRHrs>;
///GPIO alternate function high register
pub mod gpiof_afrh;
/**GPIOF_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_BRR)

For information about available fields see [`mod@gpiof_brr`] module*/
pub type GPIOF_BRR = crate::Reg<gpiof_brr::GPIOF_BRRrs>;
///GPIO port bit reset register
pub mod gpiof_brr;
