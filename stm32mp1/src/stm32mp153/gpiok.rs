#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpiok_moder: GPIOK_MODER,
    gpiok_otyper: GPIOK_OTYPER,
    gpiok_ospeedr: GPIOK_OSPEEDR,
    gpiok_pupdr: GPIOK_PUPDR,
    gpiok_idr: GPIOK_IDR,
    gpiok_odr: GPIOK_ODR,
    gpiok_bsrr: GPIOK_BSRR,
    gpiok_lckr: GPIOK_LCKR,
    gpiok_afrl: GPIOK_AFRL,
    gpiok_afrh: GPIOK_AFRH,
    gpiok_brr: GPIOK_BRR,
    _reserved11: [u8; 0x039c],
    gpiok_hwcfgr10: GPIOK_HWCFGR10,
    gpiok_hwcfgr9: GPIOK_HWCFGR9,
    gpiok_hwcfgr8: GPIOK_HWCFGR8,
    gpiok_hwcfgr7: GPIOK_HWCFGR7,
    gpiok_hwcfgr6: GPIOK_HWCFGR6,
    gpiok_hwcfgr5: GPIOK_HWCFGR5,
    gpiok_hwcfgr4: GPIOK_HWCFGR4,
    gpiok_hwcfgr3: GPIOK_HWCFGR3,
    gpiok_hwcfgr2: GPIOK_HWCFGR2,
    gpiok_hwcfgr1: GPIOK_HWCFGR1,
    gpiok_hwcfgr0: GPIOK_HWCFGR0,
    gpiok_verr: GPIOK_VERR,
    gpiok_ipidr: GPIOK_IPIDR,
    gpiok_sidr: GPIOK_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpiok_moder(&self) -> &GPIOK_MODER {
        &self.gpiok_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpiok_otyper(&self) -> &GPIOK_OTYPER {
        &self.gpiok_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpiok_ospeedr(&self) -> &GPIOK_OSPEEDR {
        &self.gpiok_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpiok_pupdr(&self) -> &GPIOK_PUPDR {
        &self.gpiok_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpiok_idr(&self) -> &GPIOK_IDR {
        &self.gpiok_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpiok_odr(&self) -> &GPIOK_ODR {
        &self.gpiok_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpiok_bsrr(&self) -> &GPIOK_BSRR {
        &self.gpiok_bsrr
    }
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    #[inline(always)]
    pub const fn gpiok_lckr(&self) -> &GPIOK_LCKR {
        &self.gpiok_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpiok_afrl(&self) -> &GPIOK_AFRL {
        &self.gpiok_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpiok_afrh(&self) -> &GPIOK_AFRH {
        &self.gpiok_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpiok_brr(&self) -> &GPIOK_BRR {
        &self.gpiok_brr
    }
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr10(&self) -> &GPIOK_HWCFGR10 {
        &self.gpiok_hwcfgr10
    }
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr9(&self) -> &GPIOK_HWCFGR9 {
        &self.gpiok_hwcfgr9
    }
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr8(&self) -> &GPIOK_HWCFGR8 {
        &self.gpiok_hwcfgr8
    }
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr7(&self) -> &GPIOK_HWCFGR7 {
        &self.gpiok_hwcfgr7
    }
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr6(&self) -> &GPIOK_HWCFGR6 {
        &self.gpiok_hwcfgr6
    }
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr5(&self) -> &GPIOK_HWCFGR5 {
        &self.gpiok_hwcfgr5
    }
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr4(&self) -> &GPIOK_HWCFGR4 {
        &self.gpiok_hwcfgr4
    }
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr3(&self) -> &GPIOK_HWCFGR3 {
        &self.gpiok_hwcfgr3
    }
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr2(&self) -> &GPIOK_HWCFGR2 {
        &self.gpiok_hwcfgr2
    }
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr1(&self) -> &GPIOK_HWCFGR1 {
        &self.gpiok_hwcfgr1
    }
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    #[inline(always)]
    pub const fn gpiok_hwcfgr0(&self) -> &GPIOK_HWCFGR0 {
        &self.gpiok_hwcfgr0
    }
    #[doc = "0x3f4 - GPIO version register"]
    #[inline(always)]
    pub const fn gpiok_verr(&self) -> &GPIOK_VERR {
        &self.gpiok_verr
    }
    #[doc = "0x3f8 - GPIO identification register"]
    #[inline(always)]
    pub const fn gpiok_ipidr(&self) -> &GPIOK_IPIDR {
        &self.gpiok_ipidr
    }
    #[doc = "0x3fc - GPIO size identification register"]
    #[inline(always)]
    pub const fn gpiok_sidr(&self) -> &GPIOK_SIDR {
        &self.gpiok_sidr
    }
}
#[doc = "GPIOK_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_moder`]
module"]
pub type GPIOK_MODER = crate::Reg<gpiok_moder::GPIOK_MODERrs>;
#[doc = "GPIO port mode register"]
pub mod gpiok_moder;
#[doc = "GPIOK_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_otyper`]
module"]
pub type GPIOK_OTYPER = crate::Reg<gpiok_otyper::GPIOK_OTYPERrs>;
#[doc = "GPIO port output type register"]
pub mod gpiok_otyper;
#[doc = "GPIOK_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_ospeedr`]
module"]
pub type GPIOK_OSPEEDR = crate::Reg<gpiok_ospeedr::GPIOK_OSPEEDRrs>;
#[doc = "GPIO port output speed register"]
pub mod gpiok_ospeedr;
#[doc = "GPIOK_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_pupdr`]
module"]
pub type GPIOK_PUPDR = crate::Reg<gpiok_pupdr::GPIOK_PUPDRrs>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiok_pupdr;
#[doc = "GPIOK_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_idr`]
module"]
pub type GPIOK_IDR = crate::Reg<gpiok_idr::GPIOK_IDRrs>;
#[doc = "GPIO port input data register"]
pub mod gpiok_idr;
#[doc = "GPIOK_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_odr`]
module"]
pub type GPIOK_ODR = crate::Reg<gpiok_odr::GPIOK_ODRrs>;
#[doc = "GPIO port output data register"]
pub mod gpiok_odr;
#[doc = "GPIOK_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_bsrr`]
module"]
pub type GPIOK_BSRR = crate::Reg<gpiok_bsrr::GPIOK_BSRRrs>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiok_bsrr;
#[doc = "GPIOK_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_lckr`]
module"]
pub type GPIOK_LCKR = crate::Reg<gpiok_lckr::GPIOK_LCKRrs>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiok_lckr;
#[doc = "GPIOK_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_afrl`]
module"]
pub type GPIOK_AFRL = crate::Reg<gpiok_afrl::GPIOK_AFRLrs>;
#[doc = "GPIO alternate function low register"]
pub mod gpiok_afrl;
#[doc = "GPIOK_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_afrh`]
module"]
pub type GPIOK_AFRH = crate::Reg<gpiok_afrh::GPIOK_AFRHrs>;
#[doc = "GPIO alternate function high register"]
pub mod gpiok_afrh;
#[doc = "GPIOK_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiok_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_brr`]
module"]
pub type GPIOK_BRR = crate::Reg<gpiok_brr::GPIOK_BRRrs>;
#[doc = "GPIO port bit reset register"]
pub mod gpiok_brr;
#[doc = "GPIOK_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr10`]
module"]
pub type GPIOK_HWCFGR10 = crate::Reg<gpiok_hwcfgr10::GPIOK_HWCFGR10rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiok_hwcfgr10;
#[doc = "GPIOK_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr9`]
module"]
pub type GPIOK_HWCFGR9 = crate::Reg<gpiok_hwcfgr9::GPIOK_HWCFGR9rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiok_hwcfgr9;
#[doc = "GPIOK_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr8`]
module"]
pub type GPIOK_HWCFGR8 = crate::Reg<gpiok_hwcfgr8::GPIOK_HWCFGR8rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiok_hwcfgr8;
#[doc = "GPIOK_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr7`]
module"]
pub type GPIOK_HWCFGR7 = crate::Reg<gpiok_hwcfgr7::GPIOK_HWCFGR7rs>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiok_hwcfgr7;
#[doc = "GPIOK_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr6`]
module"]
pub type GPIOK_HWCFGR6 = crate::Reg<gpiok_hwcfgr6::GPIOK_HWCFGR6rs>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiok_hwcfgr6;
#[doc = "GPIOK_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr5`]
module"]
pub type GPIOK_HWCFGR5 = crate::Reg<gpiok_hwcfgr5::GPIOK_HWCFGR5rs>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiok_hwcfgr5;
#[doc = "GPIOK_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr4`]
module"]
pub type GPIOK_HWCFGR4 = crate::Reg<gpiok_hwcfgr4::GPIOK_HWCFGR4rs>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiok_hwcfgr4;
#[doc = "GPIOK_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr3`]
module"]
pub type GPIOK_HWCFGR3 = crate::Reg<gpiok_hwcfgr3::GPIOK_HWCFGR3rs>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiok_hwcfgr3;
#[doc = "GPIOK_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr2`]
module"]
pub type GPIOK_HWCFGR2 = crate::Reg<gpiok_hwcfgr2::GPIOK_HWCFGR2rs>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiok_hwcfgr2;
#[doc = "GPIOK_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr1`]
module"]
pub type GPIOK_HWCFGR1 = crate::Reg<gpiok_hwcfgr1::GPIOK_HWCFGR1rs>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiok_hwcfgr1;
#[doc = "GPIOK_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_hwcfgr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_hwcfgr0`]
module"]
pub type GPIOK_HWCFGR0 = crate::Reg<gpiok_hwcfgr0::GPIOK_HWCFGR0rs>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiok_hwcfgr0;
#[doc = "GPIOK_VERR (r) register accessor: GPIO version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_verr`]
module"]
pub type GPIOK_VERR = crate::Reg<gpiok_verr::GPIOK_VERRrs>;
#[doc = "GPIO version register"]
pub mod gpiok_verr;
#[doc = "GPIOK_IPIDR (r) register accessor: GPIO identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_ipidr`]
module"]
pub type GPIOK_IPIDR = crate::Reg<gpiok_ipidr::GPIOK_IPIDRrs>;
#[doc = "GPIO identification register"]
pub mod gpiok_ipidr;
#[doc = "GPIOK_SIDR (r) register accessor: GPIO size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiok_sidr`]
module"]
pub type GPIOK_SIDR = crate::Reg<gpiok_sidr::GPIOK_SIDRrs>;
#[doc = "GPIO size identification register"]
pub mod gpiok_sidr;
