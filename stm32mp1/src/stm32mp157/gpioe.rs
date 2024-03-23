#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpioe_moder: GPIOE_MODER,
    gpioe_otyper: GPIOE_OTYPER,
    gpioe_ospeedr: GPIOE_OSPEEDR,
    gpioe_pupdr: GPIOE_PUPDR,
    gpioe_idr: GPIOE_IDR,
    gpioe_odr: GPIOE_ODR,
    gpioe_bsrr: GPIOE_BSRR,
    gpioe_lckr: GPIOE_LCKR,
    gpioe_afrl: GPIOE_AFRL,
    gpioe_afrh: GPIOE_AFRH,
    gpioe_brr: GPIOE_BRR,
    _reserved11: [u8; 0x039c],
    gpioe_hwcfgr10: GPIOE_HWCFGR10,
    gpioe_hwcfgr9: GPIOE_HWCFGR9,
    gpioe_hwcfgr8: GPIOE_HWCFGR8,
    gpioe_hwcfgr7: GPIOE_HWCFGR7,
    gpioe_hwcfgr6: GPIOE_HWCFGR6,
    gpioe_hwcfgr5: GPIOE_HWCFGR5,
    gpioe_hwcfgr4: GPIOE_HWCFGR4,
    gpioe_hwcfgr3: GPIOE_HWCFGR3,
    gpioe_hwcfgr2: GPIOE_HWCFGR2,
    gpioe_hwcfgr1: GPIOE_HWCFGR1,
    gpioe_hwcfgr0: GPIOE_HWCFGR0,
    gpioe_verr: GPIOE_VERR,
    gpioe_ipidr: GPIOE_IPIDR,
    gpioe_sidr: GPIOE_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpioe_moder(&self) -> &GPIOE_MODER {
        &self.gpioe_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpioe_otyper(&self) -> &GPIOE_OTYPER {
        &self.gpioe_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpioe_ospeedr(&self) -> &GPIOE_OSPEEDR {
        &self.gpioe_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpioe_pupdr(&self) -> &GPIOE_PUPDR {
        &self.gpioe_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpioe_idr(&self) -> &GPIOE_IDR {
        &self.gpioe_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpioe_odr(&self) -> &GPIOE_ODR {
        &self.gpioe_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpioe_bsrr(&self) -> &GPIOE_BSRR {
        &self.gpioe_bsrr
    }
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    #[inline(always)]
    pub const fn gpioe_lckr(&self) -> &GPIOE_LCKR {
        &self.gpioe_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpioe_afrl(&self) -> &GPIOE_AFRL {
        &self.gpioe_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpioe_afrh(&self) -> &GPIOE_AFRH {
        &self.gpioe_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpioe_brr(&self) -> &GPIOE_BRR {
        &self.gpioe_brr
    }
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr10(&self) -> &GPIOE_HWCFGR10 {
        &self.gpioe_hwcfgr10
    }
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr9(&self) -> &GPIOE_HWCFGR9 {
        &self.gpioe_hwcfgr9
    }
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr8(&self) -> &GPIOE_HWCFGR8 {
        &self.gpioe_hwcfgr8
    }
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr7(&self) -> &GPIOE_HWCFGR7 {
        &self.gpioe_hwcfgr7
    }
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr6(&self) -> &GPIOE_HWCFGR6 {
        &self.gpioe_hwcfgr6
    }
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr5(&self) -> &GPIOE_HWCFGR5 {
        &self.gpioe_hwcfgr5
    }
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr4(&self) -> &GPIOE_HWCFGR4 {
        &self.gpioe_hwcfgr4
    }
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr3(&self) -> &GPIOE_HWCFGR3 {
        &self.gpioe_hwcfgr3
    }
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr2(&self) -> &GPIOE_HWCFGR2 {
        &self.gpioe_hwcfgr2
    }
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr1(&self) -> &GPIOE_HWCFGR1 {
        &self.gpioe_hwcfgr1
    }
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    #[inline(always)]
    pub const fn gpioe_hwcfgr0(&self) -> &GPIOE_HWCFGR0 {
        &self.gpioe_hwcfgr0
    }
    #[doc = "0x3f4 - GPIO version register"]
    #[inline(always)]
    pub const fn gpioe_verr(&self) -> &GPIOE_VERR {
        &self.gpioe_verr
    }
    #[doc = "0x3f8 - GPIO identification register"]
    #[inline(always)]
    pub const fn gpioe_ipidr(&self) -> &GPIOE_IPIDR {
        &self.gpioe_ipidr
    }
    #[doc = "0x3fc - GPIO size identification register"]
    #[inline(always)]
    pub const fn gpioe_sidr(&self) -> &GPIOE_SIDR {
        &self.gpioe_sidr
    }
}
#[doc = "GPIOE_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_moder`]
module"]
pub type GPIOE_MODER = crate::Reg<gpioe_moder::GPIOE_MODERrs>;
#[doc = "GPIO port mode register"]
pub mod gpioe_moder;
#[doc = "GPIOE_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_otyper`]
module"]
pub type GPIOE_OTYPER = crate::Reg<gpioe_otyper::GPIOE_OTYPERrs>;
#[doc = "GPIO port output type register"]
pub mod gpioe_otyper;
#[doc = "GPIOE_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_ospeedr`]
module"]
pub type GPIOE_OSPEEDR = crate::Reg<gpioe_ospeedr::GPIOE_OSPEEDRrs>;
#[doc = "GPIO port output speed register"]
pub mod gpioe_ospeedr;
#[doc = "GPIOE_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_pupdr`]
module"]
pub type GPIOE_PUPDR = crate::Reg<gpioe_pupdr::GPIOE_PUPDRrs>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioe_pupdr;
#[doc = "GPIOE_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_idr`]
module"]
pub type GPIOE_IDR = crate::Reg<gpioe_idr::GPIOE_IDRrs>;
#[doc = "GPIO port input data register"]
pub mod gpioe_idr;
#[doc = "GPIOE_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_odr`]
module"]
pub type GPIOE_ODR = crate::Reg<gpioe_odr::GPIOE_ODRrs>;
#[doc = "GPIO port output data register"]
pub mod gpioe_odr;
#[doc = "GPIOE_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_bsrr`]
module"]
pub type GPIOE_BSRR = crate::Reg<gpioe_bsrr::GPIOE_BSRRrs>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioe_bsrr;
#[doc = "GPIOE_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_lckr`]
module"]
pub type GPIOE_LCKR = crate::Reg<gpioe_lckr::GPIOE_LCKRrs>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioe_lckr;
#[doc = "GPIOE_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_afrl`]
module"]
pub type GPIOE_AFRL = crate::Reg<gpioe_afrl::GPIOE_AFRLrs>;
#[doc = "GPIO alternate function low register"]
pub mod gpioe_afrl;
#[doc = "GPIOE_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_afrh`]
module"]
pub type GPIOE_AFRH = crate::Reg<gpioe_afrh::GPIOE_AFRHrs>;
#[doc = "GPIO alternate function high register"]
pub mod gpioe_afrh;
#[doc = "GPIOE_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioe_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_brr`]
module"]
pub type GPIOE_BRR = crate::Reg<gpioe_brr::GPIOE_BRRrs>;
#[doc = "GPIO port bit reset register"]
pub mod gpioe_brr;
#[doc = "GPIOE_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr10`]
module"]
pub type GPIOE_HWCFGR10 = crate::Reg<gpioe_hwcfgr10::GPIOE_HWCFGR10rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioe_hwcfgr10;
#[doc = "GPIOE_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr9`]
module"]
pub type GPIOE_HWCFGR9 = crate::Reg<gpioe_hwcfgr9::GPIOE_HWCFGR9rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioe_hwcfgr9;
#[doc = "GPIOE_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr8`]
module"]
pub type GPIOE_HWCFGR8 = crate::Reg<gpioe_hwcfgr8::GPIOE_HWCFGR8rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioe_hwcfgr8;
#[doc = "GPIOE_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr7`]
module"]
pub type GPIOE_HWCFGR7 = crate::Reg<gpioe_hwcfgr7::GPIOE_HWCFGR7rs>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioe_hwcfgr7;
#[doc = "GPIOE_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr6`]
module"]
pub type GPIOE_HWCFGR6 = crate::Reg<gpioe_hwcfgr6::GPIOE_HWCFGR6rs>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioe_hwcfgr6;
#[doc = "GPIOE_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr5`]
module"]
pub type GPIOE_HWCFGR5 = crate::Reg<gpioe_hwcfgr5::GPIOE_HWCFGR5rs>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioe_hwcfgr5;
#[doc = "GPIOE_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr4`]
module"]
pub type GPIOE_HWCFGR4 = crate::Reg<gpioe_hwcfgr4::GPIOE_HWCFGR4rs>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioe_hwcfgr4;
#[doc = "GPIOE_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr3`]
module"]
pub type GPIOE_HWCFGR3 = crate::Reg<gpioe_hwcfgr3::GPIOE_HWCFGR3rs>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioe_hwcfgr3;
#[doc = "GPIOE_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr2`]
module"]
pub type GPIOE_HWCFGR2 = crate::Reg<gpioe_hwcfgr2::GPIOE_HWCFGR2rs>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioe_hwcfgr2;
#[doc = "GPIOE_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr1`]
module"]
pub type GPIOE_HWCFGR1 = crate::Reg<gpioe_hwcfgr1::GPIOE_HWCFGR1rs>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioe_hwcfgr1;
#[doc = "GPIOE_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_hwcfgr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_hwcfgr0`]
module"]
pub type GPIOE_HWCFGR0 = crate::Reg<gpioe_hwcfgr0::GPIOE_HWCFGR0rs>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioe_hwcfgr0;
#[doc = "GPIOE_VERR (r) register accessor: GPIO version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_verr`]
module"]
pub type GPIOE_VERR = crate::Reg<gpioe_verr::GPIOE_VERRrs>;
#[doc = "GPIO version register"]
pub mod gpioe_verr;
#[doc = "GPIOE_IPIDR (r) register accessor: GPIO identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_ipidr`]
module"]
pub type GPIOE_IPIDR = crate::Reg<gpioe_ipidr::GPIOE_IPIDRrs>;
#[doc = "GPIO identification register"]
pub mod gpioe_ipidr;
#[doc = "GPIOE_SIDR (r) register accessor: GPIO size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioe_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe_sidr`]
module"]
pub type GPIOE_SIDR = crate::Reg<gpioe_sidr::GPIOE_SIDRrs>;
#[doc = "GPIO size identification register"]
pub mod gpioe_sidr;
