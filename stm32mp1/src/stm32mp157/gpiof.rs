#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpiof_moder: GPIOF_MODER,
    gpiof_otyper: GPIOF_OTYPER,
    gpiof_ospeedr: GPIOF_OSPEEDR,
    gpiof_pupdr: GPIOF_PUPDR,
    gpiof_idr: GPIOF_IDR,
    gpiof_odr: GPIOF_ODR,
    gpiof_bsrr: GPIOF_BSRR,
    gpiof_lckr: GPIOF_LCKR,
    gpiof_afrl: GPIOF_AFRL,
    gpiof_afrh: GPIOF_AFRH,
    gpiof_brr: GPIOF_BRR,
    _reserved11: [u8; 0x039c],
    gpiof_hwcfgr10: GPIOF_HWCFGR10,
    gpiof_hwcfgr9: GPIOF_HWCFGR9,
    gpiof_hwcfgr8: GPIOF_HWCFGR8,
    gpiof_hwcfgr7: GPIOF_HWCFGR7,
    gpiof_hwcfgr6: GPIOF_HWCFGR6,
    gpiof_hwcfgr5: GPIOF_HWCFGR5,
    gpiof_hwcfgr4: GPIOF_HWCFGR4,
    gpiof_hwcfgr3: GPIOF_HWCFGR3,
    gpiof_hwcfgr2: GPIOF_HWCFGR2,
    gpiof_hwcfgr1: GPIOF_HWCFGR1,
    gpiof_hwcfgr0: GPIOF_HWCFGR0,
    gpiof_verr: GPIOF_VERR,
    gpiof_ipidr: GPIOF_IPIDR,
    gpiof_sidr: GPIOF_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpiof_moder(&self) -> &GPIOF_MODER {
        &self.gpiof_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpiof_otyper(&self) -> &GPIOF_OTYPER {
        &self.gpiof_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpiof_ospeedr(&self) -> &GPIOF_OSPEEDR {
        &self.gpiof_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpiof_pupdr(&self) -> &GPIOF_PUPDR {
        &self.gpiof_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpiof_idr(&self) -> &GPIOF_IDR {
        &self.gpiof_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpiof_odr(&self) -> &GPIOF_ODR {
        &self.gpiof_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpiof_bsrr(&self) -> &GPIOF_BSRR {
        &self.gpiof_bsrr
    }
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    #[inline(always)]
    pub const fn gpiof_lckr(&self) -> &GPIOF_LCKR {
        &self.gpiof_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpiof_afrl(&self) -> &GPIOF_AFRL {
        &self.gpiof_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpiof_afrh(&self) -> &GPIOF_AFRH {
        &self.gpiof_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpiof_brr(&self) -> &GPIOF_BRR {
        &self.gpiof_brr
    }
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr10(&self) -> &GPIOF_HWCFGR10 {
        &self.gpiof_hwcfgr10
    }
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr9(&self) -> &GPIOF_HWCFGR9 {
        &self.gpiof_hwcfgr9
    }
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr8(&self) -> &GPIOF_HWCFGR8 {
        &self.gpiof_hwcfgr8
    }
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr7(&self) -> &GPIOF_HWCFGR7 {
        &self.gpiof_hwcfgr7
    }
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr6(&self) -> &GPIOF_HWCFGR6 {
        &self.gpiof_hwcfgr6
    }
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr5(&self) -> &GPIOF_HWCFGR5 {
        &self.gpiof_hwcfgr5
    }
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr4(&self) -> &GPIOF_HWCFGR4 {
        &self.gpiof_hwcfgr4
    }
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr3(&self) -> &GPIOF_HWCFGR3 {
        &self.gpiof_hwcfgr3
    }
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr2(&self) -> &GPIOF_HWCFGR2 {
        &self.gpiof_hwcfgr2
    }
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr1(&self) -> &GPIOF_HWCFGR1 {
        &self.gpiof_hwcfgr1
    }
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    #[inline(always)]
    pub const fn gpiof_hwcfgr0(&self) -> &GPIOF_HWCFGR0 {
        &self.gpiof_hwcfgr0
    }
    #[doc = "0x3f4 - GPIO version register"]
    #[inline(always)]
    pub const fn gpiof_verr(&self) -> &GPIOF_VERR {
        &self.gpiof_verr
    }
    #[doc = "0x3f8 - GPIO identification register"]
    #[inline(always)]
    pub const fn gpiof_ipidr(&self) -> &GPIOF_IPIDR {
        &self.gpiof_ipidr
    }
    #[doc = "0x3fc - GPIO size identification register"]
    #[inline(always)]
    pub const fn gpiof_sidr(&self) -> &GPIOF_SIDR {
        &self.gpiof_sidr
    }
}
#[doc = "GPIOF_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_moder`]
module"]
pub type GPIOF_MODER = crate::Reg<gpiof_moder::GPIOF_MODERrs>;
#[doc = "GPIO port mode register"]
pub mod gpiof_moder;
#[doc = "GPIOF_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_otyper`]
module"]
pub type GPIOF_OTYPER = crate::Reg<gpiof_otyper::GPIOF_OTYPERrs>;
#[doc = "GPIO port output type register"]
pub mod gpiof_otyper;
#[doc = "GPIOF_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_ospeedr`]
module"]
pub type GPIOF_OSPEEDR = crate::Reg<gpiof_ospeedr::GPIOF_OSPEEDRrs>;
#[doc = "GPIO port output speed register"]
pub mod gpiof_ospeedr;
#[doc = "GPIOF_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_pupdr`]
module"]
pub type GPIOF_PUPDR = crate::Reg<gpiof_pupdr::GPIOF_PUPDRrs>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiof_pupdr;
#[doc = "GPIOF_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_idr`]
module"]
pub type GPIOF_IDR = crate::Reg<gpiof_idr::GPIOF_IDRrs>;
#[doc = "GPIO port input data register"]
pub mod gpiof_idr;
#[doc = "GPIOF_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_odr`]
module"]
pub type GPIOF_ODR = crate::Reg<gpiof_odr::GPIOF_ODRrs>;
#[doc = "GPIO port output data register"]
pub mod gpiof_odr;
#[doc = "GPIOF_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_bsrr`]
module"]
pub type GPIOF_BSRR = crate::Reg<gpiof_bsrr::GPIOF_BSRRrs>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiof_bsrr;
#[doc = "GPIOF_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_lckr`]
module"]
pub type GPIOF_LCKR = crate::Reg<gpiof_lckr::GPIOF_LCKRrs>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiof_lckr;
#[doc = "GPIOF_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_afrl`]
module"]
pub type GPIOF_AFRL = crate::Reg<gpiof_afrl::GPIOF_AFRLrs>;
#[doc = "GPIO alternate function low register"]
pub mod gpiof_afrl;
#[doc = "GPIOF_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_afrh`]
module"]
pub type GPIOF_AFRH = crate::Reg<gpiof_afrh::GPIOF_AFRHrs>;
#[doc = "GPIO alternate function high register"]
pub mod gpiof_afrh;
#[doc = "GPIOF_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_brr`]
module"]
pub type GPIOF_BRR = crate::Reg<gpiof_brr::GPIOF_BRRrs>;
#[doc = "GPIO port bit reset register"]
pub mod gpiof_brr;
#[doc = "GPIOF_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr10`]
module"]
pub type GPIOF_HWCFGR10 = crate::Reg<gpiof_hwcfgr10::GPIOF_HWCFGR10rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiof_hwcfgr10;
#[doc = "GPIOF_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr9`]
module"]
pub type GPIOF_HWCFGR9 = crate::Reg<gpiof_hwcfgr9::GPIOF_HWCFGR9rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiof_hwcfgr9;
#[doc = "GPIOF_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr8`]
module"]
pub type GPIOF_HWCFGR8 = crate::Reg<gpiof_hwcfgr8::GPIOF_HWCFGR8rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiof_hwcfgr8;
#[doc = "GPIOF_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr7`]
module"]
pub type GPIOF_HWCFGR7 = crate::Reg<gpiof_hwcfgr7::GPIOF_HWCFGR7rs>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiof_hwcfgr7;
#[doc = "GPIOF_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr6`]
module"]
pub type GPIOF_HWCFGR6 = crate::Reg<gpiof_hwcfgr6::GPIOF_HWCFGR6rs>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiof_hwcfgr6;
#[doc = "GPIOF_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr5`]
module"]
pub type GPIOF_HWCFGR5 = crate::Reg<gpiof_hwcfgr5::GPIOF_HWCFGR5rs>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiof_hwcfgr5;
#[doc = "GPIOF_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr4`]
module"]
pub type GPIOF_HWCFGR4 = crate::Reg<gpiof_hwcfgr4::GPIOF_HWCFGR4rs>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiof_hwcfgr4;
#[doc = "GPIOF_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr3`]
module"]
pub type GPIOF_HWCFGR3 = crate::Reg<gpiof_hwcfgr3::GPIOF_HWCFGR3rs>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiof_hwcfgr3;
#[doc = "GPIOF_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr2`]
module"]
pub type GPIOF_HWCFGR2 = crate::Reg<gpiof_hwcfgr2::GPIOF_HWCFGR2rs>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiof_hwcfgr2;
#[doc = "GPIOF_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr1`]
module"]
pub type GPIOF_HWCFGR1 = crate::Reg<gpiof_hwcfgr1::GPIOF_HWCFGR1rs>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiof_hwcfgr1;
#[doc = "GPIOF_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_hwcfgr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_hwcfgr0`]
module"]
pub type GPIOF_HWCFGR0 = crate::Reg<gpiof_hwcfgr0::GPIOF_HWCFGR0rs>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiof_hwcfgr0;
#[doc = "GPIOF_VERR (r) register accessor: GPIO version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_verr`]
module"]
pub type GPIOF_VERR = crate::Reg<gpiof_verr::GPIOF_VERRrs>;
#[doc = "GPIO version register"]
pub mod gpiof_verr;
#[doc = "GPIOF_IPIDR (r) register accessor: GPIO identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_ipidr`]
module"]
pub type GPIOF_IPIDR = crate::Reg<gpiof_ipidr::GPIOF_IPIDRrs>;
#[doc = "GPIO identification register"]
pub mod gpiof_ipidr;
#[doc = "GPIOF_SIDR (r) register accessor: GPIO size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiof_sidr`]
module"]
pub type GPIOF_SIDR = crate::Reg<gpiof_sidr::GPIOF_SIDRrs>;
#[doc = "GPIO size identification register"]
pub mod gpiof_sidr;
