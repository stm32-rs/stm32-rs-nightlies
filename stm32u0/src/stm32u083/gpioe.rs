#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpioe_moder: GPIOE_MODER,
    gpioe_otyper: GPIOE_OTYPER,
    gpioe_ospeedr: GPIOE_OSPEEDR,
    gpioe_pupdr: GPIOE_PUPDR,
    gpioe_idr: GPIOE_IDR,
    gpioe_odr: GPIOE_ODR,
    gpioe_bsrr: GPIOE_BSRR,
    gpioe_lckr: GPIOE_LCKR,
    gpioe_afrl: GPIOE_AFRL,
    gpioe_afrh: GPIOE_AFRH,
    gpioe_brr: GPIOE_BRR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpioe_moder(&self) -> &GPIOE_MODER {
        &self.gpioe_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpioe_otyper(&self) -> &GPIOE_OTYPER {
        &self.gpioe_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpioe_ospeedr(&self) -> &GPIOE_OSPEEDR {
        &self.gpioe_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpioe_pupdr(&self) -> &GPIOE_PUPDR {
        &self.gpioe_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpioe_idr(&self) -> &GPIOE_IDR {
        &self.gpioe_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpioe_odr(&self) -> &GPIOE_ODR {
        &self.gpioe_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpioe_bsrr(&self) -> &GPIOE_BSRR {
        &self.gpioe_bsrr
    }
    ///0x1c - GPIO port configuration lock register
    #[inline(always)]
    pub const fn gpioe_lckr(&self) -> &GPIOE_LCKR {
        &self.gpioe_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpioe_afrl(&self) -> &GPIOE_AFRL {
        &self.gpioe_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpioe_afrh(&self) -> &GPIOE_AFRH {
        &self.gpioe_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpioe_brr(&self) -> &GPIOE_BRR {
        &self.gpioe_brr
    }
}
/**GPIOE_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioe_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_MODER)

For information about available fields see [`mod@gpioe_moder`]
module*/
pub type GPIOE_MODER = crate::Reg<gpioe_moder::GPIOE_MODERrs>;
///GPIO port mode register
pub mod gpioe_moder;
/**GPIOE_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpioe_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_OTYPER)

For information about available fields see [`mod@gpioe_otyper`]
module*/
pub type GPIOE_OTYPER = crate::Reg<gpioe_otyper::GPIOE_OTYPERrs>;
///GPIO port output type register
pub mod gpioe_otyper;
/**GPIOE_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioe_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_OSPEEDR)

For information about available fields see [`mod@gpioe_ospeedr`]
module*/
pub type GPIOE_OSPEEDR = crate::Reg<gpioe_ospeedr::GPIOE_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpioe_ospeedr;
/**GPIOE_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioe_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_PUPDR)

For information about available fields see [`mod@gpioe_pupdr`]
module*/
pub type GPIOE_PUPDR = crate::Reg<gpioe_pupdr::GPIOE_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpioe_pupdr;
/**GPIOE_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioe_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_IDR)

For information about available fields see [`mod@gpioe_idr`]
module*/
pub type GPIOE_IDR = crate::Reg<gpioe_idr::GPIOE_IDRrs>;
///GPIO port input data register
pub mod gpioe_idr;
/**GPIOE_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpioe_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_ODR)

For information about available fields see [`mod@gpioe_odr`]
module*/
pub type GPIOE_ODR = crate::Reg<gpioe_odr::GPIOE_ODRrs>;
///GPIO port output data register
pub mod gpioe_odr;
/**GPIOE_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_BSRR)

For information about available fields see [`mod@gpioe_bsrr`]
module*/
pub type GPIOE_BSRR = crate::Reg<gpioe_bsrr::GPIOE_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpioe_bsrr;
/**GPIOE_LCKR (rw) register accessor: GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`gpioe_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_LCKR)

For information about available fields see [`mod@gpioe_lckr`]
module*/
pub type GPIOE_LCKR = crate::Reg<gpioe_lckr::GPIOE_LCKRrs>;
///GPIO port configuration lock register
pub mod gpioe_lckr;
/**GPIOE_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioe_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_AFRL)

For information about available fields see [`mod@gpioe_afrl`]
module*/
pub type GPIOE_AFRL = crate::Reg<gpioe_afrl::GPIOE_AFRLrs>;
///GPIO alternate function low register
pub mod gpioe_afrl;
/**GPIOE_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioe_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_AFRH)

For information about available fields see [`mod@gpioe_afrh`]
module*/
pub type GPIOE_AFRH = crate::Reg<gpioe_afrh::GPIOE_AFRHrs>;
///GPIO alternate function high register
pub mod gpioe_afrh;
/**GPIOE_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#GPIOE:GPIOE_BRR)

For information about available fields see [`mod@gpioe_brr`]
module*/
pub type GPIOE_BRR = crate::Reg<gpioe_brr::GPIOE_BRRrs>;
///GPIO port bit reset register
pub mod gpioe_brr;
