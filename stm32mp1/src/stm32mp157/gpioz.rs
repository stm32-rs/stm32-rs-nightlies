#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpioz_moder: GPIOZ_MODER,
    gpioz_otyper: GPIOZ_OTYPER,
    gpioz_ospeedr: GPIOZ_OSPEEDR,
    gpioz_pupdr: GPIOZ_PUPDR,
    gpioz_idr: GPIOZ_IDR,
    gpioz_odr: GPIOZ_ODR,
    gpioz_bsrr: GPIOZ_BSRR,
    gpioz_lckr: GPIOZ_LCKR,
    gpioz_afrl: GPIOZ_AFRL,
    gpioz_afrh: GPIOZ_AFRH,
    gpioz_brr: GPIOZ_BRR,
    _reserved11: [u8; 0x04],
    gpioz_seccfgr: GPIOZ_SECCFGR,
    _reserved12: [u8; 0x0394],
    gpioz_hwcfgr10: GPIOZ_HWCFGR10,
    gpioz_hwcfgr9: GPIOZ_HWCFGR9,
    gpioz_hwcfgr8: GPIOZ_HWCFGR8,
    gpioz_hwcfgr7: GPIOZ_HWCFGR7,
    gpioz_hwcfgr6: GPIOZ_HWCFGR6,
    gpioz_hwcfgr5: GPIOZ_HWCFGR5,
    gpioz_hwcfgr4: GPIOZ_HWCFGR4,
    gpioz_hwcfgr3: GPIOZ_HWCFGR3,
    gpioz_hwcfgr2: GPIOZ_HWCFGR2,
    gpioz_hwcfgr1: GPIOZ_HWCFGR1,
    gpioz_hwcfgr0: GPIOZ_HWCFGR0,
    gpioz_verr: GPIOZ_VERR,
    gpioz_ipidr: GPIOZ_IPIDR,
    gpioz_sidr: GPIOZ_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn gpioz_moder(&self) -> &GPIOZ_MODER {
        &self.gpioz_moder
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn gpioz_otyper(&self) -> &GPIOZ_OTYPER {
        &self.gpioz_otyper
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn gpioz_ospeedr(&self) -> &GPIOZ_OSPEEDR {
        &self.gpioz_ospeedr
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn gpioz_pupdr(&self) -> &GPIOZ_PUPDR {
        &self.gpioz_pupdr
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn gpioz_idr(&self) -> &GPIOZ_IDR {
        &self.gpioz_idr
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn gpioz_odr(&self) -> &GPIOZ_ODR {
        &self.gpioz_odr
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn gpioz_bsrr(&self) -> &GPIOZ_BSRR {
        &self.gpioz_bsrr
    }
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    #[inline(always)]
    pub const fn gpioz_lckr(&self) -> &GPIOZ_LCKR {
        &self.gpioz_lckr
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn gpioz_afrl(&self) -> &GPIOZ_AFRL {
        &self.gpioz_afrl
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn gpioz_afrh(&self) -> &GPIOZ_AFRH {
        &self.gpioz_afrh
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn gpioz_brr(&self) -> &GPIOZ_BRR {
        &self.gpioz_brr
    }
    #[doc = "0x30 - This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded."]
    #[inline(always)]
    pub const fn gpioz_seccfgr(&self) -> &GPIOZ_SECCFGR {
        &self.gpioz_seccfgr
    }
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr10(&self) -> &GPIOZ_HWCFGR10 {
        &self.gpioz_hwcfgr10
    }
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr9(&self) -> &GPIOZ_HWCFGR9 {
        &self.gpioz_hwcfgr9
    }
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr8(&self) -> &GPIOZ_HWCFGR8 {
        &self.gpioz_hwcfgr8
    }
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr7(&self) -> &GPIOZ_HWCFGR7 {
        &self.gpioz_hwcfgr7
    }
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr6(&self) -> &GPIOZ_HWCFGR6 {
        &self.gpioz_hwcfgr6
    }
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr5(&self) -> &GPIOZ_HWCFGR5 {
        &self.gpioz_hwcfgr5
    }
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr4(&self) -> &GPIOZ_HWCFGR4 {
        &self.gpioz_hwcfgr4
    }
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr3(&self) -> &GPIOZ_HWCFGR3 {
        &self.gpioz_hwcfgr3
    }
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr2(&self) -> &GPIOZ_HWCFGR2 {
        &self.gpioz_hwcfgr2
    }
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr1(&self) -> &GPIOZ_HWCFGR1 {
        &self.gpioz_hwcfgr1
    }
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    #[inline(always)]
    pub const fn gpioz_hwcfgr0(&self) -> &GPIOZ_HWCFGR0 {
        &self.gpioz_hwcfgr0
    }
    #[doc = "0x3f4 - GPIO version register"]
    #[inline(always)]
    pub const fn gpioz_verr(&self) -> &GPIOZ_VERR {
        &self.gpioz_verr
    }
    #[doc = "0x3f8 - GPIO identification register"]
    #[inline(always)]
    pub const fn gpioz_ipidr(&self) -> &GPIOZ_IPIDR {
        &self.gpioz_ipidr
    }
    #[doc = "0x3fc - GPIO size identification register"]
    #[inline(always)]
    pub const fn gpioz_sidr(&self) -> &GPIOZ_SIDR {
        &self.gpioz_sidr
    }
}
#[doc = "GPIOZ_MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_moder::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_moder::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_moder`]
module"]
pub type GPIOZ_MODER = crate::Reg<gpioz_moder::GPIOZ_MODERrs>;
#[doc = "GPIO port mode register"]
pub mod gpioz_moder;
#[doc = "GPIOZ_OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_otyper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_otyper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_otyper`]
module"]
pub type GPIOZ_OTYPER = crate::Reg<gpioz_otyper::GPIOZ_OTYPERrs>;
#[doc = "GPIO port output type register"]
pub mod gpioz_otyper;
#[doc = "GPIOZ_OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_ospeedr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_ospeedr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_ospeedr`]
module"]
pub type GPIOZ_OSPEEDR = crate::Reg<gpioz_ospeedr::GPIOZ_OSPEEDRrs>;
#[doc = "GPIO port output speed register"]
pub mod gpioz_ospeedr;
#[doc = "GPIOZ_PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_pupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_pupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_pupdr`]
module"]
pub type GPIOZ_PUPDR = crate::Reg<gpioz_pupdr::GPIOZ_PUPDRrs>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioz_pupdr;
#[doc = "GPIOZ_IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_idr`]
module"]
pub type GPIOZ_IDR = crate::Reg<gpioz_idr::GPIOZ_IDRrs>;
#[doc = "GPIO port input data register"]
pub mod gpioz_idr;
#[doc = "GPIOZ_ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_odr`]
module"]
pub type GPIOZ_ODR = crate::Reg<gpioz_odr::GPIOZ_ODRrs>;
#[doc = "GPIO port output data register"]
pub mod gpioz_odr;
#[doc = "GPIOZ_BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_bsrr`]
module"]
pub type GPIOZ_BSRR = crate::Reg<gpioz_bsrr::GPIOZ_BSRRrs>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioz_bsrr;
#[doc = "GPIOZ_LCKR (rw) register accessor: This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_lckr`]
module"]
pub type GPIOZ_LCKR = crate::Reg<gpioz_lckr::GPIOZ_LCKRrs>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioz_lckr;
#[doc = "GPIOZ_AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_afrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_afrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_afrl`]
module"]
pub type GPIOZ_AFRL = crate::Reg<gpioz_afrl::GPIOZ_AFRLrs>;
#[doc = "GPIO alternate function low register"]
pub mod gpioz_afrl;
#[doc = "GPIOZ_AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_afrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_afrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_afrh`]
module"]
pub type GPIOZ_AFRH = crate::Reg<gpioz_afrh::GPIOZ_AFRHrs>;
#[doc = "GPIO alternate function high register"]
pub mod gpioz_afrh;
#[doc = "GPIOZ_BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_brr`]
module"]
pub type GPIOZ_BRR = crate::Reg<gpioz_brr::GPIOZ_BRRrs>;
#[doc = "GPIO port bit reset register"]
pub mod gpioz_brr;
#[doc = "GPIOZ_SECCFGR (w) register accessor: This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_seccfgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_seccfgr`]
module"]
pub type GPIOZ_SECCFGR = crate::Reg<gpioz_seccfgr::GPIOZ_SECCFGRrs>;
#[doc = "This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded."]
pub mod gpioz_seccfgr;
#[doc = "GPIOZ_HWCFGR10 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr10`]
module"]
pub type GPIOZ_HWCFGR10 = crate::Reg<gpioz_hwcfgr10::GPIOZ_HWCFGR10rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioz_hwcfgr10;
#[doc = "GPIOZ_HWCFGR9 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr9`]
module"]
pub type GPIOZ_HWCFGR9 = crate::Reg<gpioz_hwcfgr9::GPIOZ_HWCFGR9rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioz_hwcfgr9;
#[doc = "GPIOZ_HWCFGR8 (r) register accessor: For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr8`]
module"]
pub type GPIOZ_HWCFGR8 = crate::Reg<gpioz_hwcfgr8::GPIOZ_HWCFGR8rs>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioz_hwcfgr8;
#[doc = "GPIOZ_HWCFGR7 (r) register accessor: GPIO hardware configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr7`]
module"]
pub type GPIOZ_HWCFGR7 = crate::Reg<gpioz_hwcfgr7::GPIOZ_HWCFGR7rs>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioz_hwcfgr7;
#[doc = "GPIOZ_HWCFGR6 (r) register accessor: GPIO hardware configuration register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr6`]
module"]
pub type GPIOZ_HWCFGR6 = crate::Reg<gpioz_hwcfgr6::GPIOZ_HWCFGR6rs>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioz_hwcfgr6;
#[doc = "GPIOZ_HWCFGR5 (r) register accessor: GPIO hardware configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr5`]
module"]
pub type GPIOZ_HWCFGR5 = crate::Reg<gpioz_hwcfgr5::GPIOZ_HWCFGR5rs>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioz_hwcfgr5;
#[doc = "GPIOZ_HWCFGR4 (r) register accessor: GPIO hardware configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr4`]
module"]
pub type GPIOZ_HWCFGR4 = crate::Reg<gpioz_hwcfgr4::GPIOZ_HWCFGR4rs>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioz_hwcfgr4;
#[doc = "GPIOZ_HWCFGR3 (r) register accessor: GPIO hardware configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr3`]
module"]
pub type GPIOZ_HWCFGR3 = crate::Reg<gpioz_hwcfgr3::GPIOZ_HWCFGR3rs>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioz_hwcfgr3;
#[doc = "GPIOZ_HWCFGR2 (r) register accessor: GPIO hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr2`]
module"]
pub type GPIOZ_HWCFGR2 = crate::Reg<gpioz_hwcfgr2::GPIOZ_HWCFGR2rs>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioz_hwcfgr2;
#[doc = "GPIOZ_HWCFGR1 (r) register accessor: GPIO hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr1`]
module"]
pub type GPIOZ_HWCFGR1 = crate::Reg<gpioz_hwcfgr1::GPIOZ_HWCFGR1rs>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioz_hwcfgr1;
#[doc = "GPIOZ_HWCFGR0 (r) register accessor: GPIO hardware configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_hwcfgr0`]
module"]
pub type GPIOZ_HWCFGR0 = crate::Reg<gpioz_hwcfgr0::GPIOZ_HWCFGR0rs>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioz_hwcfgr0;
#[doc = "GPIOZ_VERR (r) register accessor: GPIO version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_verr`]
module"]
pub type GPIOZ_VERR = crate::Reg<gpioz_verr::GPIOZ_VERRrs>;
#[doc = "GPIO version register"]
pub mod gpioz_verr;
#[doc = "GPIOZ_IPIDR (r) register accessor: GPIO identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_ipidr`]
module"]
pub type GPIOZ_IPIDR = crate::Reg<gpioz_ipidr::GPIOZ_IPIDRrs>;
#[doc = "GPIO identification register"]
pub mod gpioz_ipidr;
#[doc = "GPIOZ_SIDR (r) register accessor: GPIO size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioz_sidr`]
module"]
pub type GPIOZ_SIDR = crate::Reg<gpioz_sidr::GPIOZ_SIDRrs>;
#[doc = "GPIO size identification register"]
pub mod gpioz_sidr;
