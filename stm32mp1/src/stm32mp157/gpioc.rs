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
    _reserved11: [u8; 0x039c],
    gpioc_hwcfgr10: GPIOC_HWCFGR10,
    gpioc_hwcfgr9: GPIOC_HWCFGR9,
    gpioc_hwcfgr8: GPIOC_HWCFGR8,
    gpioc_hwcfgr7: GPIOC_HWCFGR7,
    gpioc_hwcfgr6: GPIOC_HWCFGR6,
    gpioc_hwcfgr5: GPIOC_HWCFGR5,
    gpioc_hwcfgr4: GPIOC_HWCFGR4,
    gpioc_hwcfgr3: GPIOC_HWCFGR3,
    gpioc_hwcfgr2: GPIOC_HWCFGR2,
    gpioc_hwcfgr1: GPIOC_HWCFGR1,
    gpioc_hwcfgr0: GPIOC_HWCFGR0,
    gpioc_verr: GPIOC_VERR,
    gpioc_ipidr: GPIOC_IPIDR,
    gpioc_sidr: GPIOC_SIDR,
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
    /**0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
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
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn gpioc_hwcfgr10(&self) -> &GPIOC_HWCFGR10 {
        &self.gpioc_hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioc_hwcfgr9(&self) -> &GPIOC_HWCFGR9 {
        &self.gpioc_hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioc_hwcfgr8(&self) -> &GPIOC_HWCFGR8 {
        &self.gpioc_hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn gpioc_hwcfgr7(&self) -> &GPIOC_HWCFGR7 {
        &self.gpioc_hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn gpioc_hwcfgr6(&self) -> &GPIOC_HWCFGR6 {
        &self.gpioc_hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn gpioc_hwcfgr5(&self) -> &GPIOC_HWCFGR5 {
        &self.gpioc_hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn gpioc_hwcfgr4(&self) -> &GPIOC_HWCFGR4 {
        &self.gpioc_hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn gpioc_hwcfgr3(&self) -> &GPIOC_HWCFGR3 {
        &self.gpioc_hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn gpioc_hwcfgr2(&self) -> &GPIOC_HWCFGR2 {
        &self.gpioc_hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn gpioc_hwcfgr1(&self) -> &GPIOC_HWCFGR1 {
        &self.gpioc_hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn gpioc_hwcfgr0(&self) -> &GPIOC_HWCFGR0 {
        &self.gpioc_hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn gpioc_verr(&self) -> &GPIOC_VERR {
        &self.gpioc_verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn gpioc_ipidr(&self) -> &GPIOC_IPIDR {
        &self.gpioc_ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn gpioc_sidr(&self) -> &GPIOC_SIDR {
        &self.gpioc_sidr
    }
}
/**GPIOC_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioc_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_MODER)

For information about available fields see [`mod@gpioc_moder`]
module*/
pub type GPIOC_MODER = crate::Reg<gpioc_moder::GPIOC_MODERrs>;
///GPIO port mode register
pub mod gpioc_moder;
/**GPIOC_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpioc_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_OTYPER)

For information about available fields see [`mod@gpioc_otyper`]
module*/
pub type GPIOC_OTYPER = crate::Reg<gpioc_otyper::GPIOC_OTYPERrs>;
///GPIO port output type register
pub mod gpioc_otyper;
/**GPIOC_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioc_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_OSPEEDR)

For information about available fields see [`mod@gpioc_ospeedr`]
module*/
pub type GPIOC_OSPEEDR = crate::Reg<gpioc_ospeedr::GPIOC_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpioc_ospeedr;
/**GPIOC_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioc_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_PUPDR)

For information about available fields see [`mod@gpioc_pupdr`]
module*/
pub type GPIOC_PUPDR = crate::Reg<gpioc_pupdr::GPIOC_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpioc_pupdr;
/**GPIOC_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioc_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_IDR)

For information about available fields see [`mod@gpioc_idr`]
module*/
pub type GPIOC_IDR = crate::Reg<gpioc_idr::GPIOC_IDRrs>;
///GPIO port input data register
pub mod gpioc_idr;
/**GPIOC_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpioc_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_ODR)

For information about available fields see [`mod@gpioc_odr`]
module*/
pub type GPIOC_ODR = crate::Reg<gpioc_odr::GPIOC_ODRrs>;
///GPIO port output data register
pub mod gpioc_odr;
/**GPIOC_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_BSRR)

For information about available fields see [`mod@gpioc_bsrr`]
module*/
pub type GPIOC_BSRR = crate::Reg<gpioc_bsrr::GPIOC_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpioc_bsrr;
/**GPIOC_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).

You can [`read`](crate::Reg::read) this register and get [`gpioc_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_LCKR)

For information about available fields see [`mod@gpioc_lckr`]
module*/
pub type GPIOC_LCKR = crate::Reg<gpioc_lckr::GPIOC_LCKRrs>;
/**This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
pub mod gpioc_lckr;
/**GPIOC_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioc_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_AFRL)

For information about available fields see [`mod@gpioc_afrl`]
module*/
pub type GPIOC_AFRL = crate::Reg<gpioc_afrl::GPIOC_AFRLrs>;
///GPIO alternate function low register
pub mod gpioc_afrl;
/**GPIOC_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioc_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_AFRH)

For information about available fields see [`mod@gpioc_afrh`]
module*/
pub type GPIOC_AFRH = crate::Reg<gpioc_afrh::GPIOC_AFRHrs>;
///GPIO alternate function high register
pub mod gpioc_afrh;
/**GPIOC_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_BRR)

For information about available fields see [`mod@gpioc_brr`]
module*/
pub type GPIOC_BRR = crate::Reg<gpioc_brr::GPIOC_BRRrs>;
///GPIO port bit reset register
pub mod gpioc_brr;
/**GPIOC_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR10)

For information about available fields see [`mod@gpioc_hwcfgr10`]
module*/
pub type GPIOC_HWCFGR10 = crate::Reg<gpioc_hwcfgr10::GPIOC_HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpioc_hwcfgr10;
/**GPIOC_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR9)

For information about available fields see [`mod@gpioc_hwcfgr9`]
module*/
pub type GPIOC_HWCFGR9 = crate::Reg<gpioc_hwcfgr9::GPIOC_HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioc_hwcfgr9;
/**GPIOC_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR8)

For information about available fields see [`mod@gpioc_hwcfgr8`]
module*/
pub type GPIOC_HWCFGR8 = crate::Reg<gpioc_hwcfgr8::GPIOC_HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioc_hwcfgr8;
/**GPIOC_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR7)

For information about available fields see [`mod@gpioc_hwcfgr7`]
module*/
pub type GPIOC_HWCFGR7 = crate::Reg<gpioc_hwcfgr7::GPIOC_HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod gpioc_hwcfgr7;
/**GPIOC_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR6)

For information about available fields see [`mod@gpioc_hwcfgr6`]
module*/
pub type GPIOC_HWCFGR6 = crate::Reg<gpioc_hwcfgr6::GPIOC_HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod gpioc_hwcfgr6;
/**GPIOC_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR5)

For information about available fields see [`mod@gpioc_hwcfgr5`]
module*/
pub type GPIOC_HWCFGR5 = crate::Reg<gpioc_hwcfgr5::GPIOC_HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod gpioc_hwcfgr5;
/**GPIOC_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR4)

For information about available fields see [`mod@gpioc_hwcfgr4`]
module*/
pub type GPIOC_HWCFGR4 = crate::Reg<gpioc_hwcfgr4::GPIOC_HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod gpioc_hwcfgr4;
/**GPIOC_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR3)

For information about available fields see [`mod@gpioc_hwcfgr3`]
module*/
pub type GPIOC_HWCFGR3 = crate::Reg<gpioc_hwcfgr3::GPIOC_HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod gpioc_hwcfgr3;
/**GPIOC_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR2)

For information about available fields see [`mod@gpioc_hwcfgr2`]
module*/
pub type GPIOC_HWCFGR2 = crate::Reg<gpioc_hwcfgr2::GPIOC_HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod gpioc_hwcfgr2;
/**GPIOC_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR1)

For information about available fields see [`mod@gpioc_hwcfgr1`]
module*/
pub type GPIOC_HWCFGR1 = crate::Reg<gpioc_hwcfgr1::GPIOC_HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod gpioc_hwcfgr1;
/**GPIOC_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`gpioc_hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_HWCFGR0)

For information about available fields see [`mod@gpioc_hwcfgr0`]
module*/
pub type GPIOC_HWCFGR0 = crate::Reg<gpioc_hwcfgr0::GPIOC_HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod gpioc_hwcfgr0;
/**GPIOC_VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`gpioc_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_VERR)

For information about available fields see [`mod@gpioc_verr`]
module*/
pub type GPIOC_VERR = crate::Reg<gpioc_verr::GPIOC_VERRrs>;
///GPIO version register
pub mod gpioc_verr;
/**GPIOC_IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpioc_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_IPIDR)

For information about available fields see [`mod@gpioc_ipidr`]
module*/
pub type GPIOC_IPIDR = crate::Reg<gpioc_ipidr::GPIOC_IPIDRrs>;
///GPIO identification register
pub mod gpioc_ipidr;
/**GPIOC_SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpioc_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOC:GPIOC_SIDR)

For information about available fields see [`mod@gpioc_sidr`]
module*/
pub type GPIOC_SIDR = crate::Reg<gpioc_sidr::GPIOC_SIDRrs>;
///GPIO size identification register
pub mod gpioc_sidr;
