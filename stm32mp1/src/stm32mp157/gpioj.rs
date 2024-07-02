#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpioj_moder: GPIOJ_MODER,
    gpioj_otyper: GPIOJ_OTYPER,
    gpioj_ospeedr: GPIOJ_OSPEEDR,
    gpioj_pupdr: GPIOJ_PUPDR,
    gpioj_idr: GPIOJ_IDR,
    gpioj_odr: GPIOJ_ODR,
    gpioj_bsrr: GPIOJ_BSRR,
    gpioj_lckr: GPIOJ_LCKR,
    gpioj_afrl: GPIOJ_AFRL,
    gpioj_afrh: GPIOJ_AFRH,
    gpioj_brr: GPIOJ_BRR,
    _reserved11: [u8; 0x039c],
    gpioj_hwcfgr10: GPIOJ_HWCFGR10,
    gpioj_hwcfgr9: GPIOJ_HWCFGR9,
    gpioj_hwcfgr8: GPIOJ_HWCFGR8,
    gpioj_hwcfgr7: GPIOJ_HWCFGR7,
    gpioj_hwcfgr6: GPIOJ_HWCFGR6,
    gpioj_hwcfgr5: GPIOJ_HWCFGR5,
    gpioj_hwcfgr4: GPIOJ_HWCFGR4,
    gpioj_hwcfgr3: GPIOJ_HWCFGR3,
    gpioj_hwcfgr2: GPIOJ_HWCFGR2,
    gpioj_hwcfgr1: GPIOJ_HWCFGR1,
    gpioj_hwcfgr0: GPIOJ_HWCFGR0,
    gpioj_verr: GPIOJ_VERR,
    gpioj_ipidr: GPIOJ_IPIDR,
    gpioj_sidr: GPIOJ_SIDR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpioj_moder(&self) -> &GPIOJ_MODER {
        &self.gpioj_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpioj_otyper(&self) -> &GPIOJ_OTYPER {
        &self.gpioj_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpioj_ospeedr(&self) -> &GPIOJ_OSPEEDR {
        &self.gpioj_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpioj_pupdr(&self) -> &GPIOJ_PUPDR {
        &self.gpioj_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpioj_idr(&self) -> &GPIOJ_IDR {
        &self.gpioj_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpioj_odr(&self) -> &GPIOJ_ODR {
        &self.gpioj_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpioj_bsrr(&self) -> &GPIOJ_BSRR {
        &self.gpioj_bsrr
    }
    /**0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
    #[inline(always)]
    pub const fn gpioj_lckr(&self) -> &GPIOJ_LCKR {
        &self.gpioj_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpioj_afrl(&self) -> &GPIOJ_AFRL {
        &self.gpioj_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpioj_afrh(&self) -> &GPIOJ_AFRH {
        &self.gpioj_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpioj_brr(&self) -> &GPIOJ_BRR {
        &self.gpioj_brr
    }
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn gpioj_hwcfgr10(&self) -> &GPIOJ_HWCFGR10 {
        &self.gpioj_hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioj_hwcfgr9(&self) -> &GPIOJ_HWCFGR9 {
        &self.gpioj_hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioj_hwcfgr8(&self) -> &GPIOJ_HWCFGR8 {
        &self.gpioj_hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn gpioj_hwcfgr7(&self) -> &GPIOJ_HWCFGR7 {
        &self.gpioj_hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn gpioj_hwcfgr6(&self) -> &GPIOJ_HWCFGR6 {
        &self.gpioj_hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn gpioj_hwcfgr5(&self) -> &GPIOJ_HWCFGR5 {
        &self.gpioj_hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn gpioj_hwcfgr4(&self) -> &GPIOJ_HWCFGR4 {
        &self.gpioj_hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn gpioj_hwcfgr3(&self) -> &GPIOJ_HWCFGR3 {
        &self.gpioj_hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn gpioj_hwcfgr2(&self) -> &GPIOJ_HWCFGR2 {
        &self.gpioj_hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn gpioj_hwcfgr1(&self) -> &GPIOJ_HWCFGR1 {
        &self.gpioj_hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn gpioj_hwcfgr0(&self) -> &GPIOJ_HWCFGR0 {
        &self.gpioj_hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn gpioj_verr(&self) -> &GPIOJ_VERR {
        &self.gpioj_verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn gpioj_ipidr(&self) -> &GPIOJ_IPIDR {
        &self.gpioj_ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn gpioj_sidr(&self) -> &GPIOJ_SIDR {
        &self.gpioj_sidr
    }
}
/**GPIOJ_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioj_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_MODER)

For information about available fields see [`mod@gpioj_moder`]
module*/
pub type GPIOJ_MODER = crate::Reg<gpioj_moder::GPIOJ_MODERrs>;
///GPIO port mode register
pub mod gpioj_moder;
/**GPIOJ_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpioj_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_OTYPER)

For information about available fields see [`mod@gpioj_otyper`]
module*/
pub type GPIOJ_OTYPER = crate::Reg<gpioj_otyper::GPIOJ_OTYPERrs>;
///GPIO port output type register
pub mod gpioj_otyper;
/**GPIOJ_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioj_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_OSPEEDR)

For information about available fields see [`mod@gpioj_ospeedr`]
module*/
pub type GPIOJ_OSPEEDR = crate::Reg<gpioj_ospeedr::GPIOJ_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpioj_ospeedr;
/**GPIOJ_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioj_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_PUPDR)

For information about available fields see [`mod@gpioj_pupdr`]
module*/
pub type GPIOJ_PUPDR = crate::Reg<gpioj_pupdr::GPIOJ_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpioj_pupdr;
/**GPIOJ_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioj_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_IDR)

For information about available fields see [`mod@gpioj_idr`]
module*/
pub type GPIOJ_IDR = crate::Reg<gpioj_idr::GPIOJ_IDRrs>;
///GPIO port input data register
pub mod gpioj_idr;
/**GPIOJ_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpioj_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_ODR)

For information about available fields see [`mod@gpioj_odr`]
module*/
pub type GPIOJ_ODR = crate::Reg<gpioj_odr::GPIOJ_ODRrs>;
///GPIO port output data register
pub mod gpioj_odr;
/**GPIOJ_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_BSRR)

For information about available fields see [`mod@gpioj_bsrr`]
module*/
pub type GPIOJ_BSRR = crate::Reg<gpioj_bsrr::GPIOJ_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpioj_bsrr;
/**GPIOJ_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).

You can [`read`](crate::Reg::read) this register and get [`gpioj_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_LCKR)

For information about available fields see [`mod@gpioj_lckr`]
module*/
pub type GPIOJ_LCKR = crate::Reg<gpioj_lckr::GPIOJ_LCKRrs>;
/**This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
pub mod gpioj_lckr;
/**GPIOJ_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioj_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_AFRL)

For information about available fields see [`mod@gpioj_afrl`]
module*/
pub type GPIOJ_AFRL = crate::Reg<gpioj_afrl::GPIOJ_AFRLrs>;
///GPIO alternate function low register
pub mod gpioj_afrl;
/**GPIOJ_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioj_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_AFRH)

For information about available fields see [`mod@gpioj_afrh`]
module*/
pub type GPIOJ_AFRH = crate::Reg<gpioj_afrh::GPIOJ_AFRHrs>;
///GPIO alternate function high register
pub mod gpioj_afrh;
/**GPIOJ_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioj_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_BRR)

For information about available fields see [`mod@gpioj_brr`]
module*/
pub type GPIOJ_BRR = crate::Reg<gpioj_brr::GPIOJ_BRRrs>;
///GPIO port bit reset register
pub mod gpioj_brr;
/**GPIOJ_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR10)

For information about available fields see [`mod@gpioj_hwcfgr10`]
module*/
pub type GPIOJ_HWCFGR10 = crate::Reg<gpioj_hwcfgr10::GPIOJ_HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpioj_hwcfgr10;
/**GPIOJ_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR9)

For information about available fields see [`mod@gpioj_hwcfgr9`]
module*/
pub type GPIOJ_HWCFGR9 = crate::Reg<gpioj_hwcfgr9::GPIOJ_HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioj_hwcfgr9;
/**GPIOJ_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR8)

For information about available fields see [`mod@gpioj_hwcfgr8`]
module*/
pub type GPIOJ_HWCFGR8 = crate::Reg<gpioj_hwcfgr8::GPIOJ_HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioj_hwcfgr8;
/**GPIOJ_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR7)

For information about available fields see [`mod@gpioj_hwcfgr7`]
module*/
pub type GPIOJ_HWCFGR7 = crate::Reg<gpioj_hwcfgr7::GPIOJ_HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod gpioj_hwcfgr7;
/**GPIOJ_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR6)

For information about available fields see [`mod@gpioj_hwcfgr6`]
module*/
pub type GPIOJ_HWCFGR6 = crate::Reg<gpioj_hwcfgr6::GPIOJ_HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod gpioj_hwcfgr6;
/**GPIOJ_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR5)

For information about available fields see [`mod@gpioj_hwcfgr5`]
module*/
pub type GPIOJ_HWCFGR5 = crate::Reg<gpioj_hwcfgr5::GPIOJ_HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod gpioj_hwcfgr5;
/**GPIOJ_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR4)

For information about available fields see [`mod@gpioj_hwcfgr4`]
module*/
pub type GPIOJ_HWCFGR4 = crate::Reg<gpioj_hwcfgr4::GPIOJ_HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod gpioj_hwcfgr4;
/**GPIOJ_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR3)

For information about available fields see [`mod@gpioj_hwcfgr3`]
module*/
pub type GPIOJ_HWCFGR3 = crate::Reg<gpioj_hwcfgr3::GPIOJ_HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod gpioj_hwcfgr3;
/**GPIOJ_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR2)

For information about available fields see [`mod@gpioj_hwcfgr2`]
module*/
pub type GPIOJ_HWCFGR2 = crate::Reg<gpioj_hwcfgr2::GPIOJ_HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod gpioj_hwcfgr2;
/**GPIOJ_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR1)

For information about available fields see [`mod@gpioj_hwcfgr1`]
module*/
pub type GPIOJ_HWCFGR1 = crate::Reg<gpioj_hwcfgr1::GPIOJ_HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod gpioj_hwcfgr1;
/**GPIOJ_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`gpioj_hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_HWCFGR0)

For information about available fields see [`mod@gpioj_hwcfgr0`]
module*/
pub type GPIOJ_HWCFGR0 = crate::Reg<gpioj_hwcfgr0::GPIOJ_HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod gpioj_hwcfgr0;
/**GPIOJ_VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`gpioj_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_VERR)

For information about available fields see [`mod@gpioj_verr`]
module*/
pub type GPIOJ_VERR = crate::Reg<gpioj_verr::GPIOJ_VERRrs>;
///GPIO version register
pub mod gpioj_verr;
/**GPIOJ_IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpioj_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_IPIDR)

For information about available fields see [`mod@gpioj_ipidr`]
module*/
pub type GPIOJ_IPIDR = crate::Reg<gpioj_ipidr::GPIOJ_IPIDRrs>;
///GPIO identification register
pub mod gpioj_ipidr;
/**GPIOJ_SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpioj_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:GPIOJ_SIDR)

For information about available fields see [`mod@gpioj_sidr`]
module*/
pub type GPIOJ_SIDR = crate::Reg<gpioj_sidr::GPIOJ_SIDRrs>;
///GPIO size identification register
pub mod gpioj_sidr;
