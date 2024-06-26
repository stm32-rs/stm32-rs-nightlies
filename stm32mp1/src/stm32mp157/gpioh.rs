#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpioh_moder: GPIOH_MODER,
    gpioh_otyper: GPIOH_OTYPER,
    gpioh_ospeedr: GPIOH_OSPEEDR,
    gpioh_pupdr: GPIOH_PUPDR,
    gpioh_idr: GPIOH_IDR,
    gpioh_odr: GPIOH_ODR,
    gpioh_bsrr: GPIOH_BSRR,
    gpioh_lckr: GPIOH_LCKR,
    gpioh_afrl: GPIOH_AFRL,
    gpioh_afrh: GPIOH_AFRH,
    gpioh_brr: GPIOH_BRR,
    _reserved11: [u8; 0x039c],
    gpioh_hwcfgr10: GPIOH_HWCFGR10,
    gpioh_hwcfgr9: GPIOH_HWCFGR9,
    gpioh_hwcfgr8: GPIOH_HWCFGR8,
    gpioh_hwcfgr7: GPIOH_HWCFGR7,
    gpioh_hwcfgr6: GPIOH_HWCFGR6,
    gpioh_hwcfgr5: GPIOH_HWCFGR5,
    gpioh_hwcfgr4: GPIOH_HWCFGR4,
    gpioh_hwcfgr3: GPIOH_HWCFGR3,
    gpioh_hwcfgr2: GPIOH_HWCFGR2,
    gpioh_hwcfgr1: GPIOH_HWCFGR1,
    gpioh_hwcfgr0: GPIOH_HWCFGR0,
    gpioh_verr: GPIOH_VERR,
    gpioh_ipidr: GPIOH_IPIDR,
    gpioh_sidr: GPIOH_SIDR,
}
impl RegisterBlock {
    ///0x00 - GPIO port mode register
    #[inline(always)]
    pub const fn gpioh_moder(&self) -> &GPIOH_MODER {
        &self.gpioh_moder
    }
    ///0x04 - GPIO port output type register
    #[inline(always)]
    pub const fn gpioh_otyper(&self) -> &GPIOH_OTYPER {
        &self.gpioh_otyper
    }
    ///0x08 - GPIO port output speed register
    #[inline(always)]
    pub const fn gpioh_ospeedr(&self) -> &GPIOH_OSPEEDR {
        &self.gpioh_ospeedr
    }
    ///0x0c - GPIO port pull-up/pull-down register
    #[inline(always)]
    pub const fn gpioh_pupdr(&self) -> &GPIOH_PUPDR {
        &self.gpioh_pupdr
    }
    ///0x10 - GPIO port input data register
    #[inline(always)]
    pub const fn gpioh_idr(&self) -> &GPIOH_IDR {
        &self.gpioh_idr
    }
    ///0x14 - GPIO port output data register
    #[inline(always)]
    pub const fn gpioh_odr(&self) -> &GPIOH_ODR {
        &self.gpioh_odr
    }
    ///0x18 - GPIO port bit set/reset register
    #[inline(always)]
    pub const fn gpioh_bsrr(&self) -> &GPIOH_BSRR {
        &self.gpioh_bsrr
    }
    /**0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
    #[inline(always)]
    pub const fn gpioh_lckr(&self) -> &GPIOH_LCKR {
        &self.gpioh_lckr
    }
    ///0x20 - GPIO alternate function low register
    #[inline(always)]
    pub const fn gpioh_afrl(&self) -> &GPIOH_AFRL {
        &self.gpioh_afrl
    }
    ///0x24 - GPIO alternate function high register
    #[inline(always)]
    pub const fn gpioh_afrh(&self) -> &GPIOH_AFRH {
        &self.gpioh_afrh
    }
    ///0x28 - GPIO port bit reset register
    #[inline(always)]
    pub const fn gpioh_brr(&self) -> &GPIOH_BRR {
        &self.gpioh_brr
    }
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    #[inline(always)]
    pub const fn gpioh_hwcfgr10(&self) -> &GPIOH_HWCFGR10 {
        &self.gpioh_hwcfgr10
    }
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioh_hwcfgr9(&self) -> &GPIOH_HWCFGR9 {
        &self.gpioh_hwcfgr9
    }
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    #[inline(always)]
    pub const fn gpioh_hwcfgr8(&self) -> &GPIOH_HWCFGR8 {
        &self.gpioh_hwcfgr8
    }
    ///0x3d4 - GPIO hardware configuration register 7
    #[inline(always)]
    pub const fn gpioh_hwcfgr7(&self) -> &GPIOH_HWCFGR7 {
        &self.gpioh_hwcfgr7
    }
    ///0x3d8 - GPIO hardware configuration register 6
    #[inline(always)]
    pub const fn gpioh_hwcfgr6(&self) -> &GPIOH_HWCFGR6 {
        &self.gpioh_hwcfgr6
    }
    ///0x3dc - GPIO hardware configuration register 5
    #[inline(always)]
    pub const fn gpioh_hwcfgr5(&self) -> &GPIOH_HWCFGR5 {
        &self.gpioh_hwcfgr5
    }
    ///0x3e0 - GPIO hardware configuration register 4
    #[inline(always)]
    pub const fn gpioh_hwcfgr4(&self) -> &GPIOH_HWCFGR4 {
        &self.gpioh_hwcfgr4
    }
    ///0x3e4 - GPIO hardware configuration register 3
    #[inline(always)]
    pub const fn gpioh_hwcfgr3(&self) -> &GPIOH_HWCFGR3 {
        &self.gpioh_hwcfgr3
    }
    ///0x3e8 - GPIO hardware configuration register 2
    #[inline(always)]
    pub const fn gpioh_hwcfgr2(&self) -> &GPIOH_HWCFGR2 {
        &self.gpioh_hwcfgr2
    }
    ///0x3ec - GPIO hardware configuration register 1
    #[inline(always)]
    pub const fn gpioh_hwcfgr1(&self) -> &GPIOH_HWCFGR1 {
        &self.gpioh_hwcfgr1
    }
    ///0x3f0 - GPIO hardware configuration register 0
    #[inline(always)]
    pub const fn gpioh_hwcfgr0(&self) -> &GPIOH_HWCFGR0 {
        &self.gpioh_hwcfgr0
    }
    ///0x3f4 - GPIO version register
    #[inline(always)]
    pub const fn gpioh_verr(&self) -> &GPIOH_VERR {
        &self.gpioh_verr
    }
    ///0x3f8 - GPIO identification register
    #[inline(always)]
    pub const fn gpioh_ipidr(&self) -> &GPIOH_IPIDR {
        &self.gpioh_ipidr
    }
    ///0x3fc - GPIO size identification register
    #[inline(always)]
    pub const fn gpioh_sidr(&self) -> &GPIOH_SIDR {
        &self.gpioh_sidr
    }
}
/**GPIOH_MODER (rw) register accessor: GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioh_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_MODER)

For information about available fields see [`mod@gpioh_moder`]
module*/
pub type GPIOH_MODER = crate::Reg<gpioh_moder::GPIOH_MODERrs>;
///GPIO port mode register
pub mod gpioh_moder;
/**GPIOH_OTYPER (rw) register accessor: GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpioh_otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_OTYPER)

For information about available fields see [`mod@gpioh_otyper`]
module*/
pub type GPIOH_OTYPER = crate::Reg<gpioh_otyper::GPIOH_OTYPERrs>;
///GPIO port output type register
pub mod gpioh_otyper;
/**GPIOH_OSPEEDR (rw) register accessor: GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioh_ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_OSPEEDR)

For information about available fields see [`mod@gpioh_ospeedr`]
module*/
pub type GPIOH_OSPEEDR = crate::Reg<gpioh_ospeedr::GPIOH_OSPEEDRrs>;
///GPIO port output speed register
pub mod gpioh_ospeedr;
/**GPIOH_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioh_pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_PUPDR)

For information about available fields see [`mod@gpioh_pupdr`]
module*/
pub type GPIOH_PUPDR = crate::Reg<gpioh_pupdr::GPIOH_PUPDRrs>;
///GPIO port pull-up/pull-down register
pub mod gpioh_pupdr;
/**GPIOH_IDR (r) register accessor: GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpioh_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_IDR)

For information about available fields see [`mod@gpioh_idr`]
module*/
pub type GPIOH_IDR = crate::Reg<gpioh_idr::GPIOH_IDRrs>;
///GPIO port input data register
pub mod gpioh_idr;
/**GPIOH_ODR (rw) register accessor: GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`gpioh_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_ODR)

For information about available fields see [`mod@gpioh_odr`]
module*/
pub type GPIOH_ODR = crate::Reg<gpioh_odr::GPIOH_ODRrs>;
///GPIO port output data register
pub mod gpioh_odr;
/**GPIOH_BSRR (w) register accessor: GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_BSRR)

For information about available fields see [`mod@gpioh_bsrr`]
module*/
pub type GPIOH_BSRR = crate::Reg<gpioh_bsrr::GPIOH_BSRRrs>;
///GPIO port bit set/reset register
pub mod gpioh_bsrr;
/**GPIOH_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).

You can [`read`](crate::Reg::read) this register and get [`gpioh_lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_LCKR)

For information about available fields see [`mod@gpioh_lckr`]
module*/
pub type GPIOH_LCKR = crate::Reg<gpioh_lckr::GPIOH_LCKRrs>;
/**This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).*/
pub mod gpioh_lckr;
/**GPIOH_AFRL (rw) register accessor: GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioh_afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_AFRL)

For information about available fields see [`mod@gpioh_afrl`]
module*/
pub type GPIOH_AFRL = crate::Reg<gpioh_afrl::GPIOH_AFRLrs>;
///GPIO alternate function low register
pub mod gpioh_afrl;
/**GPIOH_AFRH (rw) register accessor: GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioh_afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_AFRH)

For information about available fields see [`mod@gpioh_afrh`]
module*/
pub type GPIOH_AFRH = crate::Reg<gpioh_afrh::GPIOH_AFRHrs>;
///GPIO alternate function high register
pub mod gpioh_afrh;
/**GPIOH_BRR (w) register accessor: GPIO port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioh_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_BRR)

For information about available fields see [`mod@gpioh_brr`]
module*/
pub type GPIOH_BRR = crate::Reg<gpioh_brr::GPIOH_BRRrs>;
///GPIO port bit reset register
pub mod gpioh_brr;
/**GPIOH_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR10)

For information about available fields see [`mod@gpioh_hwcfgr10`]
module*/
pub type GPIOH_HWCFGR10 = crate::Reg<gpioh_hwcfgr10::GPIOH_HWCFGR10rs>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpioh_hwcfgr10;
/**GPIOH_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR9)

For information about available fields see [`mod@gpioh_hwcfgr9`]
module*/
pub type GPIOH_HWCFGR9 = crate::Reg<gpioh_hwcfgr9::GPIOH_HWCFGR9rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioh_hwcfgr9;
/**GPIOH_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR8)

For information about available fields see [`mod@gpioh_hwcfgr8`]
module*/
pub type GPIOH_HWCFGR8 = crate::Reg<gpioh_hwcfgr8::GPIOH_HWCFGR8rs>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioh_hwcfgr8;
/**GPIOH_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR7)

For information about available fields see [`mod@gpioh_hwcfgr7`]
module*/
pub type GPIOH_HWCFGR7 = crate::Reg<gpioh_hwcfgr7::GPIOH_HWCFGR7rs>;
///GPIO hardware configuration register 7
pub mod gpioh_hwcfgr7;
/**GPIOH_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR6)

For information about available fields see [`mod@gpioh_hwcfgr6`]
module*/
pub type GPIOH_HWCFGR6 = crate::Reg<gpioh_hwcfgr6::GPIOH_HWCFGR6rs>;
///GPIO hardware configuration register 6
pub mod gpioh_hwcfgr6;
/**GPIOH_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR5)

For information about available fields see [`mod@gpioh_hwcfgr5`]
module*/
pub type GPIOH_HWCFGR5 = crate::Reg<gpioh_hwcfgr5::GPIOH_HWCFGR5rs>;
///GPIO hardware configuration register 5
pub mod gpioh_hwcfgr5;
/**GPIOH_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR4)

For information about available fields see [`mod@gpioh_hwcfgr4`]
module*/
pub type GPIOH_HWCFGR4 = crate::Reg<gpioh_hwcfgr4::GPIOH_HWCFGR4rs>;
///GPIO hardware configuration register 4
pub mod gpioh_hwcfgr4;
/**GPIOH_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR3)

For information about available fields see [`mod@gpioh_hwcfgr3`]
module*/
pub type GPIOH_HWCFGR3 = crate::Reg<gpioh_hwcfgr3::GPIOH_HWCFGR3rs>;
///GPIO hardware configuration register 3
pub mod gpioh_hwcfgr3;
/**GPIOH_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR2)

For information about available fields see [`mod@gpioh_hwcfgr2`]
module*/
pub type GPIOH_HWCFGR2 = crate::Reg<gpioh_hwcfgr2::GPIOH_HWCFGR2rs>;
///GPIO hardware configuration register 2
pub mod gpioh_hwcfgr2;
/**GPIOH_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR1)

For information about available fields see [`mod@gpioh_hwcfgr1`]
module*/
pub type GPIOH_HWCFGR1 = crate::Reg<gpioh_hwcfgr1::GPIOH_HWCFGR1rs>;
///GPIO hardware configuration register 1
pub mod gpioh_hwcfgr1;
/**GPIOH_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR0)

For information about available fields see [`mod@gpioh_hwcfgr0`]
module*/
pub type GPIOH_HWCFGR0 = crate::Reg<gpioh_hwcfgr0::GPIOH_HWCFGR0rs>;
///GPIO hardware configuration register 0
pub mod gpioh_hwcfgr0;
/**GPIOH_VERR (r) register accessor: GPIO version register

You can [`read`](crate::Reg::read) this register and get [`gpioh_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_VERR)

For information about available fields see [`mod@gpioh_verr`]
module*/
pub type GPIOH_VERR = crate::Reg<gpioh_verr::GPIOH_VERRrs>;
///GPIO version register
pub mod gpioh_verr;
/**GPIOH_IPIDR (r) register accessor: GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpioh_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_IPIDR)

For information about available fields see [`mod@gpioh_ipidr`]
module*/
pub type GPIOH_IPIDR = crate::Reg<gpioh_ipidr::GPIOH_IPIDRrs>;
///GPIO identification register
pub mod gpioh_ipidr;
/**GPIOH_SIDR (r) register accessor: GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpioh_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_SIDR)

For information about available fields see [`mod@gpioh_sidr`]
module*/
pub type GPIOH_SIDR = crate::Reg<gpioh_sidr::GPIOH_SIDRrs>;
///GPIO size identification register
pub mod gpioh_sidr;
