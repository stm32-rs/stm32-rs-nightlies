#[repr(C)]
#[doc = "Register block"]
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
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpiod_moder(&self) -> &GPIOD_MODER {
        &self.gpiod_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpiod_otyper(&self) -> &GPIOD_OTYPER {
        &self.gpiod_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpiod_ospeedr(&self) -> &GPIOD_OSPEEDR {
        &self.gpiod_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpiod_pupdr(&self) -> &GPIOD_PUPDR {
        &self.gpiod_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpiod_idr(&self) -> &GPIOD_IDR {
        &self.gpiod_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpiod_odr(&self) -> &GPIOD_ODR {
        &self.gpiod_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpiod_bsrr(&self) -> &GPIOD_BSRR {
        &self.gpiod_bsrr
    }
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    #[inline(always)]
    pub const fn gpiod_lckr(&self) -> &GPIOD_LCKR {
        &self.gpiod_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpiod_afrl(&self) -> &GPIOD_AFRL {
        &self.gpiod_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpiod_afrh(&self) -> &GPIOD_AFRH {
        &self.gpiod_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpiod_brr(&self) -> &GPIOD_BRR {
        &self.gpiod_brr
    }
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr10(&self) -> &GPIOD_HWCFGR10 {
        &self.gpiod_hwcfgr10
    }
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr9(&self) -> &GPIOD_HWCFGR9 {
        &self.gpiod_hwcfgr9
    }
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr8(&self) -> &GPIOD_HWCFGR8 {
        &self.gpiod_hwcfgr8
    }
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr7(&self) -> &GPIOD_HWCFGR7 {
        &self.gpiod_hwcfgr7
    }
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr6(&self) -> &GPIOD_HWCFGR6 {
        &self.gpiod_hwcfgr6
    }
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr5(&self) -> &GPIOD_HWCFGR5 {
        &self.gpiod_hwcfgr5
    }
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr4(&self) -> &GPIOD_HWCFGR4 {
        &self.gpiod_hwcfgr4
    }
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr3(&self) -> &GPIOD_HWCFGR3 {
        &self.gpiod_hwcfgr3
    }
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr2(&self) -> &GPIOD_HWCFGR2 {
        &self.gpiod_hwcfgr2
    }
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr1(&self) -> &GPIOD_HWCFGR1 {
        &self.gpiod_hwcfgr1
    }
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    #[inline(always)]
    pub const fn gpiod_hwcfgr0(&self) -> &GPIOD_HWCFGR0 {
        &self.gpiod_hwcfgr0
    }
    #[doc = "0x3f4 - GPIO version register"]
    #[inline(always)]
    pub const fn gpiod_verr(&self) -> &GPIOD_VERR {
        &self.gpiod_verr
    }
    #[doc = "0x3f8 - GPIO identification register"]
    #[inline(always)]
    pub const fn gpiod_ipidr(&self) -> &GPIOD_IPIDR {
        &self.gpiod_ipidr
    }
    #[doc = "0x3fc - GPIO size identification register"]
    #[inline(always)]
    pub const fn gpiod_sidr(&self) -> &GPIOD_SIDR {
        &self.gpiod_sidr
    }
}
#[doc = "GPIOD_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_moder`]
module"]
pub type GPIOD_MODER = crate::Reg<gpiod_moder::GPIOD_MODERrs>;
#[doc = "GPIO port mode register"]
pub mod gpiod_moder;
#[doc = "GPIOD_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_otyper`]
module"]
pub type GPIOD_OTYPER = crate::Reg<gpiod_otyper::GPIOD_OTYPERrs>;
#[doc = "GPIO port output type register"]
pub mod gpiod_otyper;
#[doc = "GPIOD_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_ospeedr`]
module"]
pub type GPIOD_OSPEEDR = crate::Reg<gpiod_ospeedr::GPIOD_OSPEEDRrs>;
#[doc = "GPIO port output speed register"]
pub mod gpiod_ospeedr;
#[doc = "GPIOD_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_pupdr`]
module"]
pub type GPIOD_PUPDR = crate::Reg<gpiod_pupdr::GPIOD_PUPDRrs>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiod_pupdr;
#[doc = "GPIOD_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_idr`]
module"]
pub type GPIOD_IDR = crate::Reg<gpiod_idr::GPIOD_IDRrs>;
#[doc = "GPIO port input data register"]
pub mod gpiod_idr;
#[doc = "GPIOD_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_odr`]
module"]
pub type GPIOD_ODR = crate::Reg<gpiod_odr::GPIOD_ODRrs>;
#[doc = "GPIO port output data register"]
pub mod gpiod_odr;
#[doc = "GPIOD_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_bsrr`]
module"]
pub type GPIOD_BSRR = crate::Reg<gpiod_bsrr::GPIOD_BSRRrs>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiod_bsrr;
#[doc = "GPIOD_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_lckr`]
module"]
pub type GPIOD_LCKR = crate::Reg<gpiod_lckr::GPIOD_LCKRrs>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiod_lckr;
#[doc = "GPIOD_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_afrl`]
module"]
pub type GPIOD_AFRL = crate::Reg<gpiod_afrl::GPIOD_AFRLrs>;
#[doc = "GPIO alternate function low register"]
pub mod gpiod_afrl;
#[doc = "GPIOD_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_afrh`]
module"]
pub type GPIOD_AFRH = crate::Reg<gpiod_afrh::GPIOD_AFRHrs>;
#[doc = "GPIO alternate function high register"]
pub mod gpiod_afrh;
#[doc = "GPIOD_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_brr`]
module"]
pub type GPIOD_BRR = crate::Reg<gpiod_brr::GPIOD_BRRrs>;
#[doc = "GPIO port bit reset register"]
pub mod gpiod_brr;
#[doc = "GPIOD_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr10`]
module"]
pub type GPIOD_HWCFGR10 = crate::Reg<gpiod_hwcfgr10::GPIOD_HWCFGR10rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiod_hwcfgr10;
#[doc = "GPIOD_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr9`]
module"]
pub type GPIOD_HWCFGR9 = crate::Reg<gpiod_hwcfgr9::GPIOD_HWCFGR9rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiod_hwcfgr9;
#[doc = "GPIOD_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr8`]
module"]
pub type GPIOD_HWCFGR8 = crate::Reg<gpiod_hwcfgr8::GPIOD_HWCFGR8rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiod_hwcfgr8;
#[doc = "GPIOD_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr7`]
module"]
pub type GPIOD_HWCFGR7 = crate::Reg<gpiod_hwcfgr7::GPIOD_HWCFGR7rs>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiod_hwcfgr7;
#[doc = "GPIOD_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr6`]
module"]
pub type GPIOD_HWCFGR6 = crate::Reg<gpiod_hwcfgr6::GPIOD_HWCFGR6rs>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiod_hwcfgr6;
#[doc = "GPIOD_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr5`]
module"]
pub type GPIOD_HWCFGR5 = crate::Reg<gpiod_hwcfgr5::GPIOD_HWCFGR5rs>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiod_hwcfgr5;
#[doc = "GPIOD_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr4`]
module"]
pub type GPIOD_HWCFGR4 = crate::Reg<gpiod_hwcfgr4::GPIOD_HWCFGR4rs>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiod_hwcfgr4;
#[doc = "GPIOD_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr3`]
module"]
pub type GPIOD_HWCFGR3 = crate::Reg<gpiod_hwcfgr3::GPIOD_HWCFGR3rs>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiod_hwcfgr3;
#[doc = "GPIOD_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr2`]
module"]
pub type GPIOD_HWCFGR2 = crate::Reg<gpiod_hwcfgr2::GPIOD_HWCFGR2rs>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiod_hwcfgr2;
#[doc = "GPIOD_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr1`]
module"]
pub type GPIOD_HWCFGR1 = crate::Reg<gpiod_hwcfgr1::GPIOD_HWCFGR1rs>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiod_hwcfgr1;
#[doc = "GPIOD_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_hwcfgr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_hwcfgr0`]
module"]
pub type GPIOD_HWCFGR0 = crate::Reg<gpiod_hwcfgr0::GPIOD_HWCFGR0rs>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiod_hwcfgr0;
#[doc = "GPIOD_VERR (r) register accessor: GPIO version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_verr`]
module"]
pub type GPIOD_VERR = crate::Reg<gpiod_verr::GPIOD_VERRrs>;
#[doc = "GPIO version register"]
pub mod gpiod_verr;
#[doc = "GPIOD_IPIDR (r) register accessor: GPIO identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_ipidr`]
module"]
pub type GPIOD_IPIDR = crate::Reg<gpiod_ipidr::GPIOD_IPIDRrs>;
#[doc = "GPIO identification register"]
pub mod gpiod_ipidr;
#[doc = "GPIOD_SIDR (r) register accessor: GPIO size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod_sidr`]
module"]
pub type GPIOD_SIDR = crate::Reg<gpiod_sidr::GPIOD_SIDRrs>;
#[doc = "GPIO size identification register"]
pub mod gpiod_sidr;
