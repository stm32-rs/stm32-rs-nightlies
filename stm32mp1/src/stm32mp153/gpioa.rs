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
    _reserved11: [u8; 0x039c],
    gpioa_hwcfgr10: GPIOA_HWCFGR10,
    gpioa_hwcfgr9: GPIOA_HWCFGR9,
    gpioa_hwcfgr8: GPIOA_HWCFGR8,
    gpioa_hwcfgr7: GPIOA_HWCFGR7,
    gpioa_hwcfgr6: GPIOA_HWCFGR6,
    gpioa_hwcfgr5: GPIOA_HWCFGR5,
    gpioa_hwcfgr4: GPIOA_HWCFGR4,
    gpioa_hwcfgr3: GPIOA_HWCFGR3,
    gpioa_hwcfgr2: GPIOA_HWCFGR2,
    gpioa_hwcfgr1: GPIOA_HWCFGR1,
    gpioa_hwcfgr0: GPIOA_HWCFGR0,
    gpioa_verr: GPIOA_VERR,
    gpioa_ipidr: GPIOA_IPIDR,
    gpioa_sidr: GPIOA_SIDR,
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
    /**0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
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
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn gpioa_hwcfgr10(&self) -> &GPIOA_HWCFGR10 {
        &self.gpioa_hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioa_hwcfgr9(&self) -> &GPIOA_HWCFGR9 {
        &self.gpioa_hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioa_hwcfgr8(&self) -> &GPIOA_HWCFGR8 {
        &self.gpioa_hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn gpioa_hwcfgr7(&self) -> &GPIOA_HWCFGR7 {
        &self.gpioa_hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn gpioa_hwcfgr6(&self) -> &GPIOA_HWCFGR6 {
        &self.gpioa_hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn gpioa_hwcfgr5(&self) -> &GPIOA_HWCFGR5 {
        &self.gpioa_hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn gpioa_hwcfgr4(&self) -> &GPIOA_HWCFGR4 {
        &self.gpioa_hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn gpioa_hwcfgr3(&self) -> &GPIOA_HWCFGR3 {
        &self.gpioa_hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn gpioa_hwcfgr2(&self) -> &GPIOA_HWCFGR2 {
        &self.gpioa_hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn gpioa_hwcfgr1(&self) -> &GPIOA_HWCFGR1 {
        &self.gpioa_hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn gpioa_hwcfgr0(&self) -> &GPIOA_HWCFGR0 {
        &self.gpioa_hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn gpioa_verr(&self) -> &GPIOA_VERR {
        &self.gpioa_verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn gpioa_ipidr(&self) -> &GPIOA_IPIDR {
        &self.gpioa_ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn gpioa_sidr(&self) -> &GPIOA_SIDR {
        &self.gpioa_sidr
    }
}
/**GPIOA_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioa_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_MODER)

For information about available fields see [`mod@gpioa_moder`]
module*/
pub type GPIOA_MODER = crate::Reg<gpioa_moder::GPIOA_MODERrs>;
///GPIO port mode register
pub mod gpioa_moder;
/**GPIOA_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpioa_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_OTYPER)

For information about available fields see [`mod@gpioa_otyper`]
module*/
pub type GPIOA_OTYPER = crate::Reg<gpioa_otyper::GPIOA_OTYPERrs>;
///GPIO port output type register
pub mod gpioa_otyper;
/**GPIOA_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioa_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_OSPEEDR)

For information about available fields see [`mod@gpioa_ospeedr`]
module*/
pub type GPIOA_OSPEEDR = crate::Reg<gpioa_ospeedr::GPIOA_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpioa_ospeedr;
/**GPIOA_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioa_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_PUPDR)

For information about available fields see [`mod@gpioa_pupdr`]
module*/
pub type GPIOA_PUPDR = crate::Reg<gpioa_pupdr::GPIOA_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpioa_pupdr;
/**GPIOA_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioa_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_IDR)

For information about available fields see [`mod@gpioa_idr`]
module*/
pub type GPIOA_IDR = crate::Reg<gpioa_idr::GPIOA_IDRrs>;
///GPIO port input data register
pub mod gpioa_idr;
/**GPIOA_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpioa_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_ODR)

For information about available fields see [`mod@gpioa_odr`]
module*/
pub type GPIOA_ODR = crate::Reg<gpioa_odr::GPIOA_ODRrs>;
///GPIO port output data register
pub mod gpioa_odr;
/**GPIOA_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_BSRR)

For information about available fields see [`mod@gpioa_bsrr`]
module*/
pub type GPIOA_BSRR = crate::Reg<gpioa_bsrr::GPIOA_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpioa_bsrr;
/**GPIOA_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).

You can [`read`](crate::Reg::read) this register and get [`gpioa_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_LCKR)

For information about available fields see [`mod@gpioa_lckr`]
module*/
pub type GPIOA_LCKR = crate::Reg<gpioa_lckr::GPIOA_LCKRrs>;
/**This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
pub mod gpioa_lckr;
/**GPIOA_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioa_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_AFRL)

For information about available fields see [`mod@gpioa_afrl`]
module*/
pub type GPIOA_AFRL = crate::Reg<gpioa_afrl::GPIOA_AFRLrs>;
///GPIO alternate function low register
pub mod gpioa_afrl;
/**GPIOA_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioa_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_AFRH)

For information about available fields see [`mod@gpioa_afrh`]
module*/
pub type GPIOA_AFRH = crate::Reg<gpioa_afrh::GPIOA_AFRHrs>;
///GPIO alternate function high register
pub mod gpioa_afrh;
/**GPIOA_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_BRR)

For information about available fields see [`mod@gpioa_brr`]
module*/
pub type GPIOA_BRR = crate::Reg<gpioa_brr::GPIOA_BRRrs>;
///GPIO port bit reset register
pub mod gpioa_brr;
/**GPIOA_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR10)

For information about available fields see [`mod@gpioa_hwcfgr10`]
module*/
pub type GPIOA_HWCFGR10 = crate::Reg<gpioa_hwcfgr10::GPIOA_HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpioa_hwcfgr10;
/**GPIOA_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR9)

For information about available fields see [`mod@gpioa_hwcfgr9`]
module*/
pub type GPIOA_HWCFGR9 = crate::Reg<gpioa_hwcfgr9::GPIOA_HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioa_hwcfgr9;
/**GPIOA_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR8)

For information about available fields see [`mod@gpioa_hwcfgr8`]
module*/
pub type GPIOA_HWCFGR8 = crate::Reg<gpioa_hwcfgr8::GPIOA_HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioa_hwcfgr8;
/**GPIOA_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR7)

For information about available fields see [`mod@gpioa_hwcfgr7`]
module*/
pub type GPIOA_HWCFGR7 = crate::Reg<gpioa_hwcfgr7::GPIOA_HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod gpioa_hwcfgr7;
/**GPIOA_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR6)

For information about available fields see [`mod@gpioa_hwcfgr6`]
module*/
pub type GPIOA_HWCFGR6 = crate::Reg<gpioa_hwcfgr6::GPIOA_HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod gpioa_hwcfgr6;
/**GPIOA_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR5)

For information about available fields see [`mod@gpioa_hwcfgr5`]
module*/
pub type GPIOA_HWCFGR5 = crate::Reg<gpioa_hwcfgr5::GPIOA_HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod gpioa_hwcfgr5;
/**GPIOA_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR4)

For information about available fields see [`mod@gpioa_hwcfgr4`]
module*/
pub type GPIOA_HWCFGR4 = crate::Reg<gpioa_hwcfgr4::GPIOA_HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod gpioa_hwcfgr4;
/**GPIOA_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR3)

For information about available fields see [`mod@gpioa_hwcfgr3`]
module*/
pub type GPIOA_HWCFGR3 = crate::Reg<gpioa_hwcfgr3::GPIOA_HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod gpioa_hwcfgr3;
/**GPIOA_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR2)

For information about available fields see [`mod@gpioa_hwcfgr2`]
module*/
pub type GPIOA_HWCFGR2 = crate::Reg<gpioa_hwcfgr2::GPIOA_HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod gpioa_hwcfgr2;
/**GPIOA_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR1)

For information about available fields see [`mod@gpioa_hwcfgr1`]
module*/
pub type GPIOA_HWCFGR1 = crate::Reg<gpioa_hwcfgr1::GPIOA_HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod gpioa_hwcfgr1;
/**GPIOA_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`gpioa_hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_HWCFGR0)

For information about available fields see [`mod@gpioa_hwcfgr0`]
module*/
pub type GPIOA_HWCFGR0 = crate::Reg<gpioa_hwcfgr0::GPIOA_HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod gpioa_hwcfgr0;
/**GPIOA_VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`gpioa_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_VERR)

For information about available fields see [`mod@gpioa_verr`]
module*/
pub type GPIOA_VERR = crate::Reg<gpioa_verr::GPIOA_VERRrs>;
///GPIO version register
pub mod gpioa_verr;
/**GPIOA_IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpioa_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_IPIDR)

For information about available fields see [`mod@gpioa_ipidr`]
module*/
pub type GPIOA_IPIDR = crate::Reg<gpioa_ipidr::GPIOA_IPIDRrs>;
///GPIO identification register
pub mod gpioa_ipidr;
/**GPIOA_SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpioa_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOA:GPIOA_SIDR)

For information about available fields see [`mod@gpioa_sidr`]
module*/
pub type GPIOA_SIDR = crate::Reg<gpioa_sidr::GPIOA_SIDRrs>;
///GPIO size identification register
pub mod gpioa_sidr;
