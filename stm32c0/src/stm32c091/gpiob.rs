#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpiob_moder: GPIOB_MODER,
    gpiob_otyper: GPIOB_OTYPER,
    gpiob_ospeedr: GPIOB_OSPEEDR,
    gpiob_pupdr: GPIOB_PUPDR,
    gpiob_idr: GPIOB_IDR,
    gpiob_odr: GPIOB_ODR,
    gpiob_bsrr: GPIOB_BSRR,
    gpiob_lckr: GPIOB_LCKR,
    gpiob_afrl: GPIOB_AFRL,
    gpiob_afrh: GPIOB_AFRH,
    gpiob_brr: GPIOB_BRR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpiob_moder(&self) -> &GPIOB_MODER {
        &self.gpiob_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpiob_otyper(&self) -> &GPIOB_OTYPER {
        &self.gpiob_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpiob_ospeedr(&self) -> &GPIOB_OSPEEDR {
        &self.gpiob_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpiob_pupdr(&self) -> &GPIOB_PUPDR {
        &self.gpiob_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpiob_idr(&self) -> &GPIOB_IDR {
        &self.gpiob_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpiob_odr(&self) -> &GPIOB_ODR {
        &self.gpiob_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpiob_bsrr(&self) -> &GPIOB_BSRR {
        &self.gpiob_bsrr
    }
    ///0x1c - GPIO port configuration lock register
    #[inline(always)]
    pub const fn gpiob_lckr(&self) -> &GPIOB_LCKR {
        &self.gpiob_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpiob_afrl(&self) -> &GPIOB_AFRL {
        &self.gpiob_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpiob_afrh(&self) -> &GPIOB_AFRH {
        &self.gpiob_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpiob_brr(&self) -> &GPIOB_BRR {
        &self.gpiob_brr
    }
}
/**GPIOB_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpiob_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_MODER)

For information about available fields see [`mod@gpiob_moder`] module*/
pub type GPIOB_MODER = crate::Reg<gpiob_moder::GPIOB_MODERrs>;
///GPIO port mode register
pub mod gpiob_moder;
/**GPIOB_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpiob_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_OTYPER)

For information about available fields see [`mod@gpiob_otyper`] module*/
pub type GPIOB_OTYPER = crate::Reg<gpiob_otyper::GPIOB_OTYPERrs>;
///GPIO port output type register
pub mod gpiob_otyper;
/**GPIOB_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpiob_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_OSPEEDR)

For information about available fields see [`mod@gpiob_ospeedr`] module*/
pub type GPIOB_OSPEEDR = crate::Reg<gpiob_ospeedr::GPIOB_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpiob_ospeedr;
/**GPIOB_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpiob_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_PUPDR)

For information about available fields see [`mod@gpiob_pupdr`] module*/
pub type GPIOB_PUPDR = crate::Reg<gpiob_pupdr::GPIOB_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpiob_pupdr;
/**GPIOB_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpiob_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_IDR)

For information about available fields see [`mod@gpiob_idr`] module*/
pub type GPIOB_IDR = crate::Reg<gpiob_idr::GPIOB_IDRrs>;
///GPIO port input data register
pub mod gpiob_idr;
/**GPIOB_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpiob_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_ODR)

For information about available fields see [`mod@gpiob_odr`] module*/
pub type GPIOB_ODR = crate::Reg<gpiob_odr::GPIOB_ODRrs>;
///GPIO port output data register
pub mod gpiob_odr;
/**GPIOB_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_BSRR)

For information about available fields see [`mod@gpiob_bsrr`] module*/
pub type GPIOB_BSRR = crate::Reg<gpiob_bsrr::GPIOB_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpiob_bsrr;
/**GPIOB_LCKR (rw) register accessor: GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`gpiob_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_LCKR)

For information about available fields see [`mod@gpiob_lckr`] module*/
pub type GPIOB_LCKR = crate::Reg<gpiob_lckr::GPIOB_LCKRrs>;
///GPIO port configuration lock register
pub mod gpiob_lckr;
/**GPIOB_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpiob_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_AFRL)

For information about available fields see [`mod@gpiob_afrl`] module*/
pub type GPIOB_AFRL = crate::Reg<gpiob_afrl::GPIOB_AFRLrs>;
///GPIO alternate function low register
pub mod gpiob_afrl;
/**GPIOB_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpiob_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_AFRH)

For information about available fields see [`mod@gpiob_afrh`] module*/
pub type GPIOB_AFRH = crate::Reg<gpiob_afrh::GPIOB_AFRHrs>;
///GPIO alternate function high register
pub mod gpiob_afrh;
/**GPIOB_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_BRR)

For information about available fields see [`mod@gpiob_brr`] module*/
pub type GPIOB_BRR = crate::Reg<gpiob_brr::GPIOB_BRRrs>;
///GPIO port bit reset register
pub mod gpiob_brr;
