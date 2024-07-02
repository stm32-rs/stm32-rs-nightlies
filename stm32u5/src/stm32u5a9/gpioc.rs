#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpio_moder: GPIO_MODER,
    gpio_otyper: GPIO_OTYPER,
    gpio_ospeedr: GPIO_OSPEEDR,
    gpio_pupdr: GPIO_PUPDR,
    gpio_idr: GPIO_IDR,
    gpio_odr: GPIO_ODR,
    gpio_bsrr: GPIO_BSRR,
    gpio_lckr: GPIO_LCKR,
    gpio_afrl: GPIO_AFRL,
    gpio_afrh: GPIO_AFRH,
    gpio_brr: GPIO_BRR,
    gpio_hslvr: GPIO_HSLVR,
    gpio_seccfgr: GPIO_SECCFGR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpio_moder(&self) -> &GPIO_MODER {
        &self.gpio_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpio_otyper(&self) -> &GPIO_OTYPER {
        &self.gpio_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpio_ospeedr(&self) -> &GPIO_OSPEEDR {
        &self.gpio_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpio_pupdr(&self) -> &GPIO_PUPDR {
        &self.gpio_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpio_idr(&self) -> &GPIO_IDR {
        &self.gpio_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpio_odr(&self) -> &GPIO_ODR {
        &self.gpio_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpio_bsrr(&self) -> &GPIO_BSRR {
        &self.gpio_bsrr
    }
    ///0x1c - GPIO port configuration lock register
    #[inline(always)]
    pub const fn gpio_lckr(&self) -> &GPIO_LCKR {
        &self.gpio_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpio_afrl(&self) -> &GPIO_AFRL {
        &self.gpio_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpio_afrh(&self) -> &GPIO_AFRH {
        &self.gpio_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpio_brr(&self) -> &GPIO_BRR {
        &self.gpio_brr
    }
    ///0x2c - GPIO high-speed low-voltage register
    #[inline(always)]
    pub const fn gpio_hslvr(&self) -> &GPIO_HSLVR {
        &self.gpio_hslvr
    }
    ///0x30 - GPIO secure configuration register
    #[inline(always)]
    pub const fn gpio_seccfgr(&self) -> &GPIO_SECCFGR {
        &self.gpio_seccfgr
    }
}
/**GPIO_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpio_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_MODER)

For information about available fields see [`mod@gpio_moder`]
module*/
pub type GPIO_MODER = crate::Reg<gpio_moder::GPIO_MODERrs>;
///GPIO port mode register
pub mod gpio_moder;
/**GPIO_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpio_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_OTYPER)

For information about available fields see [`mod@gpio_otyper`]
module*/
pub type GPIO_OTYPER = crate::Reg<gpio_otyper::GPIO_OTYPERrs>;
///GPIO port output type register
pub mod gpio_otyper;
/**GPIO_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpio_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_OSPEEDR)

For information about available fields see [`mod@gpio_ospeedr`]
module*/
pub type GPIO_OSPEEDR = crate::Reg<gpio_ospeedr::GPIO_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpio_ospeedr;
/**GPIO_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpio_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_PUPDR)

For information about available fields see [`mod@gpio_pupdr`]
module*/
pub type GPIO_PUPDR = crate::Reg<gpio_pupdr::GPIO_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpio_pupdr;
/**GPIO_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpio_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_IDR)

For information about available fields see [`mod@gpio_idr`]
module*/
pub type GPIO_IDR = crate::Reg<gpio_idr::GPIO_IDRrs>;
///GPIO port input data register
pub mod gpio_idr;
/**GPIO_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpio_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_ODR)

For information about available fields see [`mod@gpio_odr`]
module*/
pub type GPIO_ODR = crate::Reg<gpio_odr::GPIO_ODRrs>;
///GPIO port output data register
pub mod gpio_odr;
/**GPIO_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_BSRR)

For information about available fields see [`mod@gpio_bsrr`]
module*/
pub type GPIO_BSRR = crate::Reg<gpio_bsrr::GPIO_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpio_bsrr;
/**GPIO_LCKR (rw) register accessor: GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`gpio_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_LCKR)

For information about available fields see [`mod@gpio_lckr`]
module*/
pub type GPIO_LCKR = crate::Reg<gpio_lckr::GPIO_LCKRrs>;
///GPIO port configuration lock register
pub mod gpio_lckr;
/**GPIO_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpio_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_AFRL)

For information about available fields see [`mod@gpio_afrl`]
module*/
pub type GPIO_AFRL = crate::Reg<gpio_afrl::GPIO_AFRLrs>;
///GPIO alternate function low register
pub mod gpio_afrl;
/**GPIO_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpio_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_AFRH)

For information about available fields see [`mod@gpio_afrh`]
module*/
pub type GPIO_AFRH = crate::Reg<gpio_afrh::GPIO_AFRHrs>;
///GPIO alternate function high register
pub mod gpio_afrh;
/**GPIO_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_BRR)

For information about available fields see [`mod@gpio_brr`]
module*/
pub type GPIO_BRR = crate::Reg<gpio_brr::GPIO_BRRrs>;
///GPIO port bit reset register
pub mod gpio_brr;
/**GPIO_HSLVR (rw) register accessor: GPIO high-speed low-voltage register

You can [`read`](crate::Reg::read) this register and get [`gpio_hslvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hslvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_HSLVR)

For information about available fields see [`mod@gpio_hslvr`]
module*/
pub type GPIO_HSLVR = crate::Reg<gpio_hslvr::GPIO_HSLVRrs>;
///GPIO high-speed low-voltage register
pub mod gpio_hslvr;
/**GPIO_SECCFGR (rw) register accessor: GPIO secure configuration register

You can [`read`](crate::Reg::read) this register and get [`gpio_seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPIOC:GPIO_SECCFGR)

For information about available fields see [`mod@gpio_seccfgr`]
module*/
pub type GPIO_SECCFGR = crate::Reg<gpio_seccfgr::GPIO_SECCFGRrs>;
///GPIO secure configuration register
pub mod gpio_seccfgr;
