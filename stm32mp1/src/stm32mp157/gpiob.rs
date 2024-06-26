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
    _reserved11: [u8; 0x039c],
    gpiob_hwcfgr10: GPIOB_HWCFGR10,
    gpiob_hwcfgr9: GPIOB_HWCFGR9,
    gpiob_hwcfgr8: GPIOB_HWCFGR8,
    gpiob_hwcfgr7: GPIOB_HWCFGR7,
    gpiob_hwcfgr6: GPIOB_HWCFGR6,
    gpiob_hwcfgr5: GPIOB_HWCFGR5,
    gpiob_hwcfgr4: GPIOB_HWCFGR4,
    gpiob_hwcfgr3: GPIOB_HWCFGR3,
    gpiob_hwcfgr2: GPIOB_HWCFGR2,
    gpiob_hwcfgr1: GPIOB_HWCFGR1,
    gpiob_hwcfgr0: GPIOB_HWCFGR0,
    gpiob_verr: GPIOB_VERR,
    gpiob_ipidr: GPIOB_IPIDR,
    gpiob_sidr: GPIOB_SIDR,
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
    /**0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
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
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn gpiob_hwcfgr10(&self) -> &GPIOB_HWCFGR10 {
        &self.gpiob_hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpiob_hwcfgr9(&self) -> &GPIOB_HWCFGR9 {
        &self.gpiob_hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpiob_hwcfgr8(&self) -> &GPIOB_HWCFGR8 {
        &self.gpiob_hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn gpiob_hwcfgr7(&self) -> &GPIOB_HWCFGR7 {
        &self.gpiob_hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn gpiob_hwcfgr6(&self) -> &GPIOB_HWCFGR6 {
        &self.gpiob_hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn gpiob_hwcfgr5(&self) -> &GPIOB_HWCFGR5 {
        &self.gpiob_hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn gpiob_hwcfgr4(&self) -> &GPIOB_HWCFGR4 {
        &self.gpiob_hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn gpiob_hwcfgr3(&self) -> &GPIOB_HWCFGR3 {
        &self.gpiob_hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn gpiob_hwcfgr2(&self) -> &GPIOB_HWCFGR2 {
        &self.gpiob_hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn gpiob_hwcfgr1(&self) -> &GPIOB_HWCFGR1 {
        &self.gpiob_hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn gpiob_hwcfgr0(&self) -> &GPIOB_HWCFGR0 {
        &self.gpiob_hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn gpiob_verr(&self) -> &GPIOB_VERR {
        &self.gpiob_verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn gpiob_ipidr(&self) -> &GPIOB_IPIDR {
        &self.gpiob_ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn gpiob_sidr(&self) -> &GPIOB_SIDR {
        &self.gpiob_sidr
    }
}
/**GPIOB_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpiob_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_MODER)

For information about available fields see [`mod@gpiob_moder`]
module*/
pub type GPIOB_MODER = crate::Reg<gpiob_moder::GPIOB_MODERrs>;
///GPIO port mode register
pub mod gpiob_moder;
/**GPIOB_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpiob_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_OTYPER)

For information about available fields see [`mod@gpiob_otyper`]
module*/
pub type GPIOB_OTYPER = crate::Reg<gpiob_otyper::GPIOB_OTYPERrs>;
///GPIO port output type register
pub mod gpiob_otyper;
/**GPIOB_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpiob_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_OSPEEDR)

For information about available fields see [`mod@gpiob_ospeedr`]
module*/
pub type GPIOB_OSPEEDR = crate::Reg<gpiob_ospeedr::GPIOB_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpiob_ospeedr;
/**GPIOB_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpiob_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_PUPDR)

For information about available fields see [`mod@gpiob_pupdr`]
module*/
pub type GPIOB_PUPDR = crate::Reg<gpiob_pupdr::GPIOB_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpiob_pupdr;
/**GPIOB_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpiob_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_IDR)

For information about available fields see [`mod@gpiob_idr`]
module*/
pub type GPIOB_IDR = crate::Reg<gpiob_idr::GPIOB_IDRrs>;
///GPIO port input data register
pub mod gpiob_idr;
/**GPIOB_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpiob_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_ODR)

For information about available fields see [`mod@gpiob_odr`]
module*/
pub type GPIOB_ODR = crate::Reg<gpiob_odr::GPIOB_ODRrs>;
///GPIO port output data register
pub mod gpiob_odr;
/**GPIOB_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_BSRR)

For information about available fields see [`mod@gpiob_bsrr`]
module*/
pub type GPIOB_BSRR = crate::Reg<gpiob_bsrr::GPIOB_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpiob_bsrr;
/**GPIOB_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).

You can [`read`](crate::Reg::read) this register and get [`gpiob_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_LCKR)

For information about available fields see [`mod@gpiob_lckr`]
module*/
pub type GPIOB_LCKR = crate::Reg<gpiob_lckr::GPIOB_LCKRrs>;
/**This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
pub mod gpiob_lckr;
/**GPIOB_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpiob_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_AFRL)

For information about available fields see [`mod@gpiob_afrl`]
module*/
pub type GPIOB_AFRL = crate::Reg<gpiob_afrl::GPIOB_AFRLrs>;
///GPIO alternate function low register
pub mod gpiob_afrl;
/**GPIOB_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpiob_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_AFRH)

For information about available fields see [`mod@gpiob_afrh`]
module*/
pub type GPIOB_AFRH = crate::Reg<gpiob_afrh::GPIOB_AFRHrs>;
///GPIO alternate function high register
pub mod gpiob_afrh;
/**GPIOB_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_BRR)

For information about available fields see [`mod@gpiob_brr`]
module*/
pub type GPIOB_BRR = crate::Reg<gpiob_brr::GPIOB_BRRrs>;
///GPIO port bit reset register
pub mod gpiob_brr;
/**GPIOB_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR10)

For information about available fields see [`mod@gpiob_hwcfgr10`]
module*/
pub type GPIOB_HWCFGR10 = crate::Reg<gpiob_hwcfgr10::GPIOB_HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpiob_hwcfgr10;
/**GPIOB_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR9)

For information about available fields see [`mod@gpiob_hwcfgr9`]
module*/
pub type GPIOB_HWCFGR9 = crate::Reg<gpiob_hwcfgr9::GPIOB_HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiob_hwcfgr9;
/**GPIOB_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR8)

For information about available fields see [`mod@gpiob_hwcfgr8`]
module*/
pub type GPIOB_HWCFGR8 = crate::Reg<gpiob_hwcfgr8::GPIOB_HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiob_hwcfgr8;
/**GPIOB_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR7)

For information about available fields see [`mod@gpiob_hwcfgr7`]
module*/
pub type GPIOB_HWCFGR7 = crate::Reg<gpiob_hwcfgr7::GPIOB_HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod gpiob_hwcfgr7;
/**GPIOB_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR6)

For information about available fields see [`mod@gpiob_hwcfgr6`]
module*/
pub type GPIOB_HWCFGR6 = crate::Reg<gpiob_hwcfgr6::GPIOB_HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod gpiob_hwcfgr6;
/**GPIOB_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR5)

For information about available fields see [`mod@gpiob_hwcfgr5`]
module*/
pub type GPIOB_HWCFGR5 = crate::Reg<gpiob_hwcfgr5::GPIOB_HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod gpiob_hwcfgr5;
/**GPIOB_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR4)

For information about available fields see [`mod@gpiob_hwcfgr4`]
module*/
pub type GPIOB_HWCFGR4 = crate::Reg<gpiob_hwcfgr4::GPIOB_HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod gpiob_hwcfgr4;
/**GPIOB_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR3)

For information about available fields see [`mod@gpiob_hwcfgr3`]
module*/
pub type GPIOB_HWCFGR3 = crate::Reg<gpiob_hwcfgr3::GPIOB_HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod gpiob_hwcfgr3;
/**GPIOB_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR2)

For information about available fields see [`mod@gpiob_hwcfgr2`]
module*/
pub type GPIOB_HWCFGR2 = crate::Reg<gpiob_hwcfgr2::GPIOB_HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod gpiob_hwcfgr2;
/**GPIOB_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR1)

For information about available fields see [`mod@gpiob_hwcfgr1`]
module*/
pub type GPIOB_HWCFGR1 = crate::Reg<gpiob_hwcfgr1::GPIOB_HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod gpiob_hwcfgr1;
/**GPIOB_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`gpiob_hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_HWCFGR0)

For information about available fields see [`mod@gpiob_hwcfgr0`]
module*/
pub type GPIOB_HWCFGR0 = crate::Reg<gpiob_hwcfgr0::GPIOB_HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod gpiob_hwcfgr0;
/**GPIOB_VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`gpiob_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_VERR)

For information about available fields see [`mod@gpiob_verr`]
module*/
pub type GPIOB_VERR = crate::Reg<gpiob_verr::GPIOB_VERRrs>;
///GPIO version register
pub mod gpiob_verr;
/**GPIOB_IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpiob_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_IPIDR)

For information about available fields see [`mod@gpiob_ipidr`]
module*/
pub type GPIOB_IPIDR = crate::Reg<gpiob_ipidr::GPIOB_IPIDRrs>;
///GPIO identification register
pub mod gpiob_ipidr;
/**GPIOB_SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpiob_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOB:GPIOB_SIDR)

For information about available fields see [`mod@gpiob_sidr`]
module*/
pub type GPIOB_SIDR = crate::Reg<gpiob_sidr::GPIOB_SIDRrs>;
///GPIO size identification register
pub mod gpiob_sidr;
