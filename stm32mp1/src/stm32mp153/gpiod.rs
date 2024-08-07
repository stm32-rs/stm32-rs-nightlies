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
    _reserved11: [u8; 0x039c],
    gpiod_hwcfgr10: GPIOD_HWCFGR10,
    gpiod_hwcfgr9: GPIOD_HWCFGR9,
    gpiod_hwcfgr8: GPIOD_HWCFGR8,
    gpiod_hwcfgr7: GPIOD_HWCFGR7,
    gpiod_hwcfgr6: GPIOD_HWCFGR6,
    gpiod_hwcfgr5: GPIOD_HWCFGR5,
    gpiod_hwcfgr4: GPIOD_HWCFGR4,
    gpiod_hwcfgr3: GPIOD_HWCFGR3,
    gpiod_hwcfgr2: GPIOD_HWCFGR2,
    gpiod_hwcfgr1: GPIOD_HWCFGR1,
    gpiod_hwcfgr0: GPIOD_HWCFGR0,
    gpiod_verr: GPIOD_VERR,
    gpiod_ipidr: GPIOD_IPIDR,
    gpiod_sidr: GPIOD_SIDR,
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
    /**0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
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
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn gpiod_hwcfgr10(&self) -> &GPIOD_HWCFGR10 {
        &self.gpiod_hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpiod_hwcfgr9(&self) -> &GPIOD_HWCFGR9 {
        &self.gpiod_hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpiod_hwcfgr8(&self) -> &GPIOD_HWCFGR8 {
        &self.gpiod_hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn gpiod_hwcfgr7(&self) -> &GPIOD_HWCFGR7 {
        &self.gpiod_hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn gpiod_hwcfgr6(&self) -> &GPIOD_HWCFGR6 {
        &self.gpiod_hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn gpiod_hwcfgr5(&self) -> &GPIOD_HWCFGR5 {
        &self.gpiod_hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn gpiod_hwcfgr4(&self) -> &GPIOD_HWCFGR4 {
        &self.gpiod_hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn gpiod_hwcfgr3(&self) -> &GPIOD_HWCFGR3 {
        &self.gpiod_hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn gpiod_hwcfgr2(&self) -> &GPIOD_HWCFGR2 {
        &self.gpiod_hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn gpiod_hwcfgr1(&self) -> &GPIOD_HWCFGR1 {
        &self.gpiod_hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn gpiod_hwcfgr0(&self) -> &GPIOD_HWCFGR0 {
        &self.gpiod_hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn gpiod_verr(&self) -> &GPIOD_VERR {
        &self.gpiod_verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn gpiod_ipidr(&self) -> &GPIOD_IPIDR {
        &self.gpiod_ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn gpiod_sidr(&self) -> &GPIOD_SIDR {
        &self.gpiod_sidr
    }
}
/**GPIOD_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpiod_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_MODER)

For information about available fields see [`mod@gpiod_moder`]
module*/
pub type GPIOD_MODER = crate::Reg<gpiod_moder::GPIOD_MODERrs>;
///GPIO port mode register
pub mod gpiod_moder;
/**GPIOD_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpiod_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_OTYPER)

For information about available fields see [`mod@gpiod_otyper`]
module*/
pub type GPIOD_OTYPER = crate::Reg<gpiod_otyper::GPIOD_OTYPERrs>;
///GPIO port output type register
pub mod gpiod_otyper;
/**GPIOD_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpiod_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_OSPEEDR)

For information about available fields see [`mod@gpiod_ospeedr`]
module*/
pub type GPIOD_OSPEEDR = crate::Reg<gpiod_ospeedr::GPIOD_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpiod_ospeedr;
/**GPIOD_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpiod_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_PUPDR)

For information about available fields see [`mod@gpiod_pupdr`]
module*/
pub type GPIOD_PUPDR = crate::Reg<gpiod_pupdr::GPIOD_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpiod_pupdr;
/**GPIOD_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpiod_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_IDR)

For information about available fields see [`mod@gpiod_idr`]
module*/
pub type GPIOD_IDR = crate::Reg<gpiod_idr::GPIOD_IDRrs>;
///GPIO port input data register
pub mod gpiod_idr;
/**GPIOD_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpiod_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_ODR)

For information about available fields see [`mod@gpiod_odr`]
module*/
pub type GPIOD_ODR = crate::Reg<gpiod_odr::GPIOD_ODRrs>;
///GPIO port output data register
pub mod gpiod_odr;
/**GPIOD_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_BSRR)

For information about available fields see [`mod@gpiod_bsrr`]
module*/
pub type GPIOD_BSRR = crate::Reg<gpiod_bsrr::GPIOD_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpiod_bsrr;
/**GPIOD_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).

You can [`read`](crate::Reg::read) this register and get [`gpiod_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_LCKR)

For information about available fields see [`mod@gpiod_lckr`]
module*/
pub type GPIOD_LCKR = crate::Reg<gpiod_lckr::GPIOD_LCKRrs>;
/**This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
pub mod gpiod_lckr;
/**GPIOD_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpiod_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_AFRL)

For information about available fields see [`mod@gpiod_afrl`]
module*/
pub type GPIOD_AFRL = crate::Reg<gpiod_afrl::GPIOD_AFRLrs>;
///GPIO alternate function low register
pub mod gpiod_afrl;
/**GPIOD_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpiod_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_AFRH)

For information about available fields see [`mod@gpiod_afrh`]
module*/
pub type GPIOD_AFRH = crate::Reg<gpiod_afrh::GPIOD_AFRHrs>;
///GPIO alternate function high register
pub mod gpiod_afrh;
/**GPIOD_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_BRR)

For information about available fields see [`mod@gpiod_brr`]
module*/
pub type GPIOD_BRR = crate::Reg<gpiod_brr::GPIOD_BRRrs>;
///GPIO port bit reset register
pub mod gpiod_brr;
/**GPIOD_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR10)

For information about available fields see [`mod@gpiod_hwcfgr10`]
module*/
pub type GPIOD_HWCFGR10 = crate::Reg<gpiod_hwcfgr10::GPIOD_HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpiod_hwcfgr10;
/**GPIOD_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR9)

For information about available fields see [`mod@gpiod_hwcfgr9`]
module*/
pub type GPIOD_HWCFGR9 = crate::Reg<gpiod_hwcfgr9::GPIOD_HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiod_hwcfgr9;
/**GPIOD_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR8)

For information about available fields see [`mod@gpiod_hwcfgr8`]
module*/
pub type GPIOD_HWCFGR8 = crate::Reg<gpiod_hwcfgr8::GPIOD_HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiod_hwcfgr8;
/**GPIOD_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR7)

For information about available fields see [`mod@gpiod_hwcfgr7`]
module*/
pub type GPIOD_HWCFGR7 = crate::Reg<gpiod_hwcfgr7::GPIOD_HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod gpiod_hwcfgr7;
/**GPIOD_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR6)

For information about available fields see [`mod@gpiod_hwcfgr6`]
module*/
pub type GPIOD_HWCFGR6 = crate::Reg<gpiod_hwcfgr6::GPIOD_HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod gpiod_hwcfgr6;
/**GPIOD_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR5)

For information about available fields see [`mod@gpiod_hwcfgr5`]
module*/
pub type GPIOD_HWCFGR5 = crate::Reg<gpiod_hwcfgr5::GPIOD_HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod gpiod_hwcfgr5;
/**GPIOD_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR4)

For information about available fields see [`mod@gpiod_hwcfgr4`]
module*/
pub type GPIOD_HWCFGR4 = crate::Reg<gpiod_hwcfgr4::GPIOD_HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod gpiod_hwcfgr4;
/**GPIOD_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR3)

For information about available fields see [`mod@gpiod_hwcfgr3`]
module*/
pub type GPIOD_HWCFGR3 = crate::Reg<gpiod_hwcfgr3::GPIOD_HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod gpiod_hwcfgr3;
/**GPIOD_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR2)

For information about available fields see [`mod@gpiod_hwcfgr2`]
module*/
pub type GPIOD_HWCFGR2 = crate::Reg<gpiod_hwcfgr2::GPIOD_HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod gpiod_hwcfgr2;
/**GPIOD_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR1)

For information about available fields see [`mod@gpiod_hwcfgr1`]
module*/
pub type GPIOD_HWCFGR1 = crate::Reg<gpiod_hwcfgr1::GPIOD_HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod gpiod_hwcfgr1;
/**GPIOD_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`gpiod_hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_HWCFGR0)

For information about available fields see [`mod@gpiod_hwcfgr0`]
module*/
pub type GPIOD_HWCFGR0 = crate::Reg<gpiod_hwcfgr0::GPIOD_HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod gpiod_hwcfgr0;
/**GPIOD_VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`gpiod_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_VERR)

For information about available fields see [`mod@gpiod_verr`]
module*/
pub type GPIOD_VERR = crate::Reg<gpiod_verr::GPIOD_VERRrs>;
///GPIO version register
pub mod gpiod_verr;
/**GPIOD_IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpiod_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_IPIDR)

For information about available fields see [`mod@gpiod_ipidr`]
module*/
pub type GPIOD_IPIDR = crate::Reg<gpiod_ipidr::GPIOD_IPIDRrs>;
///GPIO identification register
pub mod gpiod_ipidr;
/**GPIOD_SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpiod_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_SIDR)

For information about available fields see [`mod@gpiod_sidr`]
module*/
pub type GPIOD_SIDR = crate::Reg<gpiod_sidr::GPIOD_SIDRrs>;
///GPIO size identification register
pub mod gpiod_sidr;
