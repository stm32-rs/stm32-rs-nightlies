#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpioc_moder: GPIOC_MODER,
    gpioc_otyper: GPIOC_OTYPER,
    gpioc_ospeedr: GPIOC_OSPEEDR,
    gpioc_pupdr: GPIOC_PUPDR,
    gpioc_idr: GPIOC_IDR,
    gpioc_odr: GPIOC_ODR,
    gpioc_bsrr: GPIOC_BSRR,
    gpioc_lckr: GPIOC_LCKR,
    gpioc_afrl: GPIOC_AFRL,
    gpioc_afrh: GPIOC_AFRH,
    gpioc_brr: GPIOC_BRR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpioc_moder(&self) -> &GPIOC_MODER {
        &self.gpioc_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpioc_otyper(&self) -> &GPIOC_OTYPER {
        &self.gpioc_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpioc_ospeedr(&self) -> &GPIOC_OSPEEDR {
        &self.gpioc_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpioc_pupdr(&self) -> &GPIOC_PUPDR {
        &self.gpioc_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpioc_idr(&self) -> &GPIOC_IDR {
        &self.gpioc_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpioc_odr(&self) -> &GPIOC_ODR {
        &self.gpioc_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpioc_bsrr(&self) -> &GPIOC_BSRR {
        &self.gpioc_bsrr
    }
    ///0x1c - GPIO port configuration lock register
    #[inline(always)]
    pub const fn gpioc_lckr(&self) -> &GPIOC_LCKR {
        &self.gpioc_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpioc_afrl(&self) -> &GPIOC_AFRL {
        &self.gpioc_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpioc_afrh(&self) -> &GPIOC_AFRH {
        &self.gpioc_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpioc_brr(&self) -> &GPIOC_BRR {
        &self.gpioc_brr
    }
}
/**GPIOC_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioc_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_MODER)

For information about available fields see [`mod@gpioc_moder`] module*/
pub type GPIOC_MODER = crate::Reg<gpioc_moder::GPIOC_MODERrs>;
///GPIO port mode register
pub mod gpioc_moder;
/**GPIOC_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpioc_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_OTYPER)

For information about available fields see [`mod@gpioc_otyper`] module*/
pub type GPIOC_OTYPER = crate::Reg<gpioc_otyper::GPIOC_OTYPERrs>;
///GPIO port output type register
pub mod gpioc_otyper;
/**GPIOC_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioc_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_OSPEEDR)

For information about available fields see [`mod@gpioc_ospeedr`] module*/
pub type GPIOC_OSPEEDR = crate::Reg<gpioc_ospeedr::GPIOC_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpioc_ospeedr;
/**GPIOC_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioc_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_PUPDR)

For information about available fields see [`mod@gpioc_pupdr`] module*/
pub type GPIOC_PUPDR = crate::Reg<gpioc_pupdr::GPIOC_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpioc_pupdr;
/**GPIOC_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioc_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_IDR)

For information about available fields see [`mod@gpioc_idr`] module*/
pub type GPIOC_IDR = crate::Reg<gpioc_idr::GPIOC_IDRrs>;
///GPIO port input data register
pub mod gpioc_idr;
/**GPIOC_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpioc_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_ODR)

For information about available fields see [`mod@gpioc_odr`] module*/
pub type GPIOC_ODR = crate::Reg<gpioc_odr::GPIOC_ODRrs>;
///GPIO port output data register
pub mod gpioc_odr;
/**GPIOC_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_BSRR)

For information about available fields see [`mod@gpioc_bsrr`] module*/
pub type GPIOC_BSRR = crate::Reg<gpioc_bsrr::GPIOC_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpioc_bsrr;
/**GPIOC_LCKR (rw) register accessor: GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`gpioc_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_LCKR)

For information about available fields see [`mod@gpioc_lckr`] module*/
pub type GPIOC_LCKR = crate::Reg<gpioc_lckr::GPIOC_LCKRrs>;
///GPIO port configuration lock register
pub mod gpioc_lckr;
/**GPIOC_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioc_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_AFRL)

For information about available fields see [`mod@gpioc_afrl`] module*/
pub type GPIOC_AFRL = crate::Reg<gpioc_afrl::GPIOC_AFRLrs>;
///GPIO alternate function low register
pub mod gpioc_afrl;
/**GPIOC_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioc_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_AFRH)

For information about available fields see [`mod@gpioc_afrh`] module*/
pub type GPIOC_AFRH = crate::Reg<gpioc_afrh::GPIOC_AFRHrs>;
///GPIO alternate function high register
pub mod gpioc_afrh;
/**GPIOC_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_BRR)

For information about available fields see [`mod@gpioc_brr`] module*/
pub type GPIOC_BRR = crate::Reg<gpioc_brr::GPIOC_BRRrs>;
///GPIO port bit reset register
pub mod gpioc_brr;
