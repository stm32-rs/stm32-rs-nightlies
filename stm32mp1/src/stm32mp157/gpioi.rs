#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpioi_moder: GPIOI_MODER,
    gpioi_otyper: GPIOI_OTYPER,
    gpioi_ospeedr: GPIOI_OSPEEDR,
    gpioi_pupdr: GPIOI_PUPDR,
    gpioi_idr: GPIOI_IDR,
    gpioi_odr: GPIOI_ODR,
    gpioi_bsrr: GPIOI_BSRR,
    gpioi_lckr: GPIOI_LCKR,
    gpioi_afrl: GPIOI_AFRL,
    gpioi_afrh: GPIOI_AFRH,
    gpioi_brr: GPIOI_BRR,
    _reserved11: [u8; 0x039c],
    gpioi_hwcfgr10: GPIOI_HWCFGR10,
    gpioi_hwcfgr9: GPIOI_HWCFGR9,
    gpioi_hwcfgr8: GPIOI_HWCFGR8,
    gpioi_hwcfgr7: GPIOI_HWCFGR7,
    gpioi_hwcfgr6: GPIOI_HWCFGR6,
    gpioi_hwcfgr5: GPIOI_HWCFGR5,
    gpioi_hwcfgr4: GPIOI_HWCFGR4,
    gpioi_hwcfgr3: GPIOI_HWCFGR3,
    gpioi_hwcfgr2: GPIOI_HWCFGR2,
    gpioi_hwcfgr1: GPIOI_HWCFGR1,
    gpioi_hwcfgr0: GPIOI_HWCFGR0,
    gpioi_verr: GPIOI_VERR,
    gpioi_ipidr: GPIOI_IPIDR,
    gpioi_sidr: GPIOI_SIDR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpioi_moder(&self) -> &GPIOI_MODER {
        &self.gpioi_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpioi_otyper(&self) -> &GPIOI_OTYPER {
        &self.gpioi_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpioi_ospeedr(&self) -> &GPIOI_OSPEEDR {
        &self.gpioi_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpioi_pupdr(&self) -> &GPIOI_PUPDR {
        &self.gpioi_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpioi_idr(&self) -> &GPIOI_IDR {
        &self.gpioi_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpioi_odr(&self) -> &GPIOI_ODR {
        &self.gpioi_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpioi_bsrr(&self) -> &GPIOI_BSRR {
        &self.gpioi_bsrr
    }
    /**0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
    #[inline(always)]
    pub const fn gpioi_lckr(&self) -> &GPIOI_LCKR {
        &self.gpioi_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpioi_afrl(&self) -> &GPIOI_AFRL {
        &self.gpioi_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpioi_afrh(&self) -> &GPIOI_AFRH {
        &self.gpioi_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpioi_brr(&self) -> &GPIOI_BRR {
        &self.gpioi_brr
    }
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn gpioi_hwcfgr10(&self) -> &GPIOI_HWCFGR10 {
        &self.gpioi_hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioi_hwcfgr9(&self) -> &GPIOI_HWCFGR9 {
        &self.gpioi_hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioi_hwcfgr8(&self) -> &GPIOI_HWCFGR8 {
        &self.gpioi_hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn gpioi_hwcfgr7(&self) -> &GPIOI_HWCFGR7 {
        &self.gpioi_hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn gpioi_hwcfgr6(&self) -> &GPIOI_HWCFGR6 {
        &self.gpioi_hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn gpioi_hwcfgr5(&self) -> &GPIOI_HWCFGR5 {
        &self.gpioi_hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn gpioi_hwcfgr4(&self) -> &GPIOI_HWCFGR4 {
        &self.gpioi_hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn gpioi_hwcfgr3(&self) -> &GPIOI_HWCFGR3 {
        &self.gpioi_hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn gpioi_hwcfgr2(&self) -> &GPIOI_HWCFGR2 {
        &self.gpioi_hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn gpioi_hwcfgr1(&self) -> &GPIOI_HWCFGR1 {
        &self.gpioi_hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn gpioi_hwcfgr0(&self) -> &GPIOI_HWCFGR0 {
        &self.gpioi_hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn gpioi_verr(&self) -> &GPIOI_VERR {
        &self.gpioi_verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn gpioi_ipidr(&self) -> &GPIOI_IPIDR {
        &self.gpioi_ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn gpioi_sidr(&self) -> &GPIOI_SIDR {
        &self.gpioi_sidr
    }
}
/**GPIOI_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioi_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_MODER)

For information about available fields see [`mod@gpioi_moder`]
module*/
pub type GPIOI_MODER = crate::Reg<gpioi_moder::GPIOI_MODERrs>;
///GPIO port mode register
pub mod gpioi_moder;
/**GPIOI_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpioi_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_OTYPER)

For information about available fields see [`mod@gpioi_otyper`]
module*/
pub type GPIOI_OTYPER = crate::Reg<gpioi_otyper::GPIOI_OTYPERrs>;
///GPIO port output type register
pub mod gpioi_otyper;
/**GPIOI_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioi_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_OSPEEDR)

For information about available fields see [`mod@gpioi_ospeedr`]
module*/
pub type GPIOI_OSPEEDR = crate::Reg<gpioi_ospeedr::GPIOI_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpioi_ospeedr;
/**GPIOI_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioi_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_PUPDR)

For information about available fields see [`mod@gpioi_pupdr`]
module*/
pub type GPIOI_PUPDR = crate::Reg<gpioi_pupdr::GPIOI_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpioi_pupdr;
/**GPIOI_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioi_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_IDR)

For information about available fields see [`mod@gpioi_idr`]
module*/
pub type GPIOI_IDR = crate::Reg<gpioi_idr::GPIOI_IDRrs>;
///GPIO port input data register
pub mod gpioi_idr;
/**GPIOI_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpioi_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_ODR)

For information about available fields see [`mod@gpioi_odr`]
module*/
pub type GPIOI_ODR = crate::Reg<gpioi_odr::GPIOI_ODRrs>;
///GPIO port output data register
pub mod gpioi_odr;
/**GPIOI_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_BSRR)

For information about available fields see [`mod@gpioi_bsrr`]
module*/
pub type GPIOI_BSRR = crate::Reg<gpioi_bsrr::GPIOI_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpioi_bsrr;
/**GPIOI_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).

You can [`read`](crate::Reg::read) this register and get [`gpioi_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_LCKR)

For information about available fields see [`mod@gpioi_lckr`]
module*/
pub type GPIOI_LCKR = crate::Reg<gpioi_lckr::GPIOI_LCKRrs>;
/**This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
pub mod gpioi_lckr;
/**GPIOI_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioi_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_AFRL)

For information about available fields see [`mod@gpioi_afrl`]
module*/
pub type GPIOI_AFRL = crate::Reg<gpioi_afrl::GPIOI_AFRLrs>;
///GPIO alternate function low register
pub mod gpioi_afrl;
/**GPIOI_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioi_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_AFRH)

For information about available fields see [`mod@gpioi_afrh`]
module*/
pub type GPIOI_AFRH = crate::Reg<gpioi_afrh::GPIOI_AFRHrs>;
///GPIO alternate function high register
pub mod gpioi_afrh;
/**GPIOI_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioi_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_BRR)

For information about available fields see [`mod@gpioi_brr`]
module*/
pub type GPIOI_BRR = crate::Reg<gpioi_brr::GPIOI_BRRrs>;
///GPIO port bit reset register
pub mod gpioi_brr;
/**GPIOI_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR10)

For information about available fields see [`mod@gpioi_hwcfgr10`]
module*/
pub type GPIOI_HWCFGR10 = crate::Reg<gpioi_hwcfgr10::GPIOI_HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpioi_hwcfgr10;
/**GPIOI_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR9)

For information about available fields see [`mod@gpioi_hwcfgr9`]
module*/
pub type GPIOI_HWCFGR9 = crate::Reg<gpioi_hwcfgr9::GPIOI_HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioi_hwcfgr9;
/**GPIOI_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR8)

For information about available fields see [`mod@gpioi_hwcfgr8`]
module*/
pub type GPIOI_HWCFGR8 = crate::Reg<gpioi_hwcfgr8::GPIOI_HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioi_hwcfgr8;
/**GPIOI_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR7)

For information about available fields see [`mod@gpioi_hwcfgr7`]
module*/
pub type GPIOI_HWCFGR7 = crate::Reg<gpioi_hwcfgr7::GPIOI_HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod gpioi_hwcfgr7;
/**GPIOI_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR6)

For information about available fields see [`mod@gpioi_hwcfgr6`]
module*/
pub type GPIOI_HWCFGR6 = crate::Reg<gpioi_hwcfgr6::GPIOI_HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod gpioi_hwcfgr6;
/**GPIOI_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR5)

For information about available fields see [`mod@gpioi_hwcfgr5`]
module*/
pub type GPIOI_HWCFGR5 = crate::Reg<gpioi_hwcfgr5::GPIOI_HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod gpioi_hwcfgr5;
/**GPIOI_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR4)

For information about available fields see [`mod@gpioi_hwcfgr4`]
module*/
pub type GPIOI_HWCFGR4 = crate::Reg<gpioi_hwcfgr4::GPIOI_HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod gpioi_hwcfgr4;
/**GPIOI_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR3)

For information about available fields see [`mod@gpioi_hwcfgr3`]
module*/
pub type GPIOI_HWCFGR3 = crate::Reg<gpioi_hwcfgr3::GPIOI_HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod gpioi_hwcfgr3;
/**GPIOI_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR2)

For information about available fields see [`mod@gpioi_hwcfgr2`]
module*/
pub type GPIOI_HWCFGR2 = crate::Reg<gpioi_hwcfgr2::GPIOI_HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod gpioi_hwcfgr2;
/**GPIOI_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR1)

For information about available fields see [`mod@gpioi_hwcfgr1`]
module*/
pub type GPIOI_HWCFGR1 = crate::Reg<gpioi_hwcfgr1::GPIOI_HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod gpioi_hwcfgr1;
/**GPIOI_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`gpioi_hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_HWCFGR0)

For information about available fields see [`mod@gpioi_hwcfgr0`]
module*/
pub type GPIOI_HWCFGR0 = crate::Reg<gpioi_hwcfgr0::GPIOI_HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod gpioi_hwcfgr0;
/**GPIOI_VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`gpioi_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_VERR)

For information about available fields see [`mod@gpioi_verr`]
module*/
pub type GPIOI_VERR = crate::Reg<gpioi_verr::GPIOI_VERRrs>;
///GPIO version register
pub mod gpioi_verr;
/**GPIOI_IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpioi_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_IPIDR)

For information about available fields see [`mod@gpioi_ipidr`]
module*/
pub type GPIOI_IPIDR = crate::Reg<gpioi_ipidr::GPIOI_IPIDRrs>;
///GPIO identification register
pub mod gpioi_ipidr;
/**GPIOI_SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpioi_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOI:GPIOI_SIDR)

For information about available fields see [`mod@gpioi_sidr`]
module*/
pub type GPIOI_SIDR = crate::Reg<gpioi_sidr::GPIOI_SIDRrs>;
///GPIO size identification register
pub mod gpioi_sidr;
