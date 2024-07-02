#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpiog_moder: GPIOG_MODER,
    gpiog_otyper: GPIOG_OTYPER,
    gpiog_ospeedr: GPIOG_OSPEEDR,
    gpiog_pupdr: GPIOG_PUPDR,
    gpiog_idr: GPIOG_IDR,
    gpiog_odr: GPIOG_ODR,
    gpiog_bsrr: GPIOG_BSRR,
    gpiog_lckr: GPIOG_LCKR,
    gpiog_afrl: GPIOG_AFRL,
    gpiog_afrh: GPIOG_AFRH,
    gpiog_brr: GPIOG_BRR,
    _reserved11: [u8; 0x039c],
    gpiog_hwcfgr10: GPIOG_HWCFGR10,
    gpiog_hwcfgr9: GPIOG_HWCFGR9,
    gpiog_hwcfgr8: GPIOG_HWCFGR8,
    gpiog_hwcfgr7: GPIOG_HWCFGR7,
    gpiog_hwcfgr6: GPIOG_HWCFGR6,
    gpiog_hwcfgr5: GPIOG_HWCFGR5,
    gpiog_hwcfgr4: GPIOG_HWCFGR4,
    gpiog_hwcfgr3: GPIOG_HWCFGR3,
    gpiog_hwcfgr2: GPIOG_HWCFGR2,
    gpiog_hwcfgr1: GPIOG_HWCFGR1,
    gpiog_hwcfgr0: GPIOG_HWCFGR0,
    gpiog_verr: GPIOG_VERR,
    gpiog_ipidr: GPIOG_IPIDR,
    gpiog_sidr: GPIOG_SIDR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpiog_moder(&self) -> &GPIOG_MODER {
        &self.gpiog_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpiog_otyper(&self) -> &GPIOG_OTYPER {
        &self.gpiog_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpiog_ospeedr(&self) -> &GPIOG_OSPEEDR {
        &self.gpiog_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpiog_pupdr(&self) -> &GPIOG_PUPDR {
        &self.gpiog_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpiog_idr(&self) -> &GPIOG_IDR {
        &self.gpiog_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpiog_odr(&self) -> &GPIOG_ODR {
        &self.gpiog_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpiog_bsrr(&self) -> &GPIOG_BSRR {
        &self.gpiog_bsrr
    }
    /**0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
    #[inline(always)]
    pub const fn gpiog_lckr(&self) -> &GPIOG_LCKR {
        &self.gpiog_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpiog_afrl(&self) -> &GPIOG_AFRL {
        &self.gpiog_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpiog_afrh(&self) -> &GPIOG_AFRH {
        &self.gpiog_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpiog_brr(&self) -> &GPIOG_BRR {
        &self.gpiog_brr
    }
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn gpiog_hwcfgr10(&self) -> &GPIOG_HWCFGR10 {
        &self.gpiog_hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpiog_hwcfgr9(&self) -> &GPIOG_HWCFGR9 {
        &self.gpiog_hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpiog_hwcfgr8(&self) -> &GPIOG_HWCFGR8 {
        &self.gpiog_hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn gpiog_hwcfgr7(&self) -> &GPIOG_HWCFGR7 {
        &self.gpiog_hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn gpiog_hwcfgr6(&self) -> &GPIOG_HWCFGR6 {
        &self.gpiog_hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn gpiog_hwcfgr5(&self) -> &GPIOG_HWCFGR5 {
        &self.gpiog_hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn gpiog_hwcfgr4(&self) -> &GPIOG_HWCFGR4 {
        &self.gpiog_hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn gpiog_hwcfgr3(&self) -> &GPIOG_HWCFGR3 {
        &self.gpiog_hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn gpiog_hwcfgr2(&self) -> &GPIOG_HWCFGR2 {
        &self.gpiog_hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn gpiog_hwcfgr1(&self) -> &GPIOG_HWCFGR1 {
        &self.gpiog_hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn gpiog_hwcfgr0(&self) -> &GPIOG_HWCFGR0 {
        &self.gpiog_hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn gpiog_verr(&self) -> &GPIOG_VERR {
        &self.gpiog_verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn gpiog_ipidr(&self) -> &GPIOG_IPIDR {
        &self.gpiog_ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn gpiog_sidr(&self) -> &GPIOG_SIDR {
        &self.gpiog_sidr
    }
}
/**GPIOG_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpiog_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_MODER)

For information about available fields see [`mod@gpiog_moder`]
module*/
pub type GPIOG_MODER = crate::Reg<gpiog_moder::GPIOG_MODERrs>;
///GPIO port mode register
pub mod gpiog_moder;
/**GPIOG_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpiog_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_OTYPER)

For information about available fields see [`mod@gpiog_otyper`]
module*/
pub type GPIOG_OTYPER = crate::Reg<gpiog_otyper::GPIOG_OTYPERrs>;
///GPIO port output type register
pub mod gpiog_otyper;
/**GPIOG_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpiog_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_OSPEEDR)

For information about available fields see [`mod@gpiog_ospeedr`]
module*/
pub type GPIOG_OSPEEDR = crate::Reg<gpiog_ospeedr::GPIOG_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpiog_ospeedr;
/**GPIOG_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpiog_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_PUPDR)

For information about available fields see [`mod@gpiog_pupdr`]
module*/
pub type GPIOG_PUPDR = crate::Reg<gpiog_pupdr::GPIOG_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpiog_pupdr;
/**GPIOG_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpiog_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_IDR)

For information about available fields see [`mod@gpiog_idr`]
module*/
pub type GPIOG_IDR = crate::Reg<gpiog_idr::GPIOG_IDRrs>;
///GPIO port input data register
pub mod gpiog_idr;
/**GPIOG_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpiog_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_ODR)

For information about available fields see [`mod@gpiog_odr`]
module*/
pub type GPIOG_ODR = crate::Reg<gpiog_odr::GPIOG_ODRrs>;
///GPIO port output data register
pub mod gpiog_odr;
/**GPIOG_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_BSRR)

For information about available fields see [`mod@gpiog_bsrr`]
module*/
pub type GPIOG_BSRR = crate::Reg<gpiog_bsrr::GPIOG_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpiog_bsrr;
/**GPIOG_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).

You can [`read`](crate::Reg::read) this register and get [`gpiog_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_LCKR)

For information about available fields see [`mod@gpiog_lckr`]
module*/
pub type GPIOG_LCKR = crate::Reg<gpiog_lckr::GPIOG_LCKRrs>;
/**This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
pub mod gpiog_lckr;
/**GPIOG_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpiog_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_AFRL)

For information about available fields see [`mod@gpiog_afrl`]
module*/
pub type GPIOG_AFRL = crate::Reg<gpiog_afrl::GPIOG_AFRLrs>;
///GPIO alternate function low register
pub mod gpiog_afrl;
/**GPIOG_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpiog_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_AFRH)

For information about available fields see [`mod@gpiog_afrh`]
module*/
pub type GPIOG_AFRH = crate::Reg<gpiog_afrh::GPIOG_AFRHrs>;
///GPIO alternate function high register
pub mod gpiog_afrh;
/**GPIOG_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiog_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_BRR)

For information about available fields see [`mod@gpiog_brr`]
module*/
pub type GPIOG_BRR = crate::Reg<gpiog_brr::GPIOG_BRRrs>;
///GPIO port bit reset register
pub mod gpiog_brr;
/**GPIOG_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR10)

For information about available fields see [`mod@gpiog_hwcfgr10`]
module*/
pub type GPIOG_HWCFGR10 = crate::Reg<gpiog_hwcfgr10::GPIOG_HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpiog_hwcfgr10;
/**GPIOG_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR9)

For information about available fields see [`mod@gpiog_hwcfgr9`]
module*/
pub type GPIOG_HWCFGR9 = crate::Reg<gpiog_hwcfgr9::GPIOG_HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiog_hwcfgr9;
/**GPIOG_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR8)

For information about available fields see [`mod@gpiog_hwcfgr8`]
module*/
pub type GPIOG_HWCFGR8 = crate::Reg<gpiog_hwcfgr8::GPIOG_HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiog_hwcfgr8;
/**GPIOG_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR7)

For information about available fields see [`mod@gpiog_hwcfgr7`]
module*/
pub type GPIOG_HWCFGR7 = crate::Reg<gpiog_hwcfgr7::GPIOG_HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod gpiog_hwcfgr7;
/**GPIOG_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR6)

For information about available fields see [`mod@gpiog_hwcfgr6`]
module*/
pub type GPIOG_HWCFGR6 = crate::Reg<gpiog_hwcfgr6::GPIOG_HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod gpiog_hwcfgr6;
/**GPIOG_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR5)

For information about available fields see [`mod@gpiog_hwcfgr5`]
module*/
pub type GPIOG_HWCFGR5 = crate::Reg<gpiog_hwcfgr5::GPIOG_HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod gpiog_hwcfgr5;
/**GPIOG_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR4)

For information about available fields see [`mod@gpiog_hwcfgr4`]
module*/
pub type GPIOG_HWCFGR4 = crate::Reg<gpiog_hwcfgr4::GPIOG_HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod gpiog_hwcfgr4;
/**GPIOG_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR3)

For information about available fields see [`mod@gpiog_hwcfgr3`]
module*/
pub type GPIOG_HWCFGR3 = crate::Reg<gpiog_hwcfgr3::GPIOG_HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod gpiog_hwcfgr3;
/**GPIOG_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR2)

For information about available fields see [`mod@gpiog_hwcfgr2`]
module*/
pub type GPIOG_HWCFGR2 = crate::Reg<gpiog_hwcfgr2::GPIOG_HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod gpiog_hwcfgr2;
/**GPIOG_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR1)

For information about available fields see [`mod@gpiog_hwcfgr1`]
module*/
pub type GPIOG_HWCFGR1 = crate::Reg<gpiog_hwcfgr1::GPIOG_HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod gpiog_hwcfgr1;
/**GPIOG_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR0)

For information about available fields see [`mod@gpiog_hwcfgr0`]
module*/
pub type GPIOG_HWCFGR0 = crate::Reg<gpiog_hwcfgr0::GPIOG_HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod gpiog_hwcfgr0;
/**GPIOG_VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`gpiog_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_VERR)

For information about available fields see [`mod@gpiog_verr`]
module*/
pub type GPIOG_VERR = crate::Reg<gpiog_verr::GPIOG_VERRrs>;
///GPIO version register
pub mod gpiog_verr;
/**GPIOG_IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpiog_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_IPIDR)

For information about available fields see [`mod@gpiog_ipidr`]
module*/
pub type GPIOG_IPIDR = crate::Reg<gpiog_ipidr::GPIOG_IPIDRrs>;
///GPIO identification register
pub mod gpiog_ipidr;
/**GPIOG_SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpiog_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_SIDR)

For information about available fields see [`mod@gpiog_sidr`]
module*/
pub type GPIOG_SIDR = crate::Reg<gpiog_sidr::GPIOG_SIDRrs>;
///GPIO size identification register
pub mod gpiog_sidr;
