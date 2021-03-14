#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioz_moder: GPIOZ_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioz_otyper: GPIOZ_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioz_ospeedr: GPIOZ_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioz_pupdr: GPIOZ_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioz_idr: GPIOZ_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioz_odr: GPIOZ_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioz_bsrr: GPIOZ_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioz_lckr: GPIOZ_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioz_afrl: GPIOZ_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioz_afrh: GPIOZ_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioz_brr: GPIOZ_BRR,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded."]
    pub gpioz_seccfgr: GPIOZ_SECCFGR,
    _reserved12: [u8; 916usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioz_hwcfgr10: GPIOZ_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioz_hwcfgr9: GPIOZ_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioz_hwcfgr8: GPIOZ_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioz_hwcfgr7: GPIOZ_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioz_hwcfgr6: GPIOZ_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioz_hwcfgr5: GPIOZ_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioz_hwcfgr4: GPIOZ_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioz_hwcfgr3: GPIOZ_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioz_hwcfgr2: GPIOZ_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioz_hwcfgr1: GPIOZ_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioz_hwcfgr0: GPIOZ_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioz_verr: GPIOZ_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioz_ipidr: GPIOZ_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioz_sidr: GPIOZ_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_moder](gpioz_moder) module"]
pub type GPIOZ_MODER = crate::Reg<u32, _GPIOZ_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_MODER;
#[doc = "`read()` method returns [gpioz_moder::R](gpioz_moder::R) reader structure"]
impl crate::Readable for GPIOZ_MODER {}
#[doc = "`write(|w| ..)` method takes [gpioz_moder::W](gpioz_moder::W) writer structure"]
impl crate::Writable for GPIOZ_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpioz_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_otyper](gpioz_otyper) module"]
pub type GPIOZ_OTYPER = crate::Reg<u32, _GPIOZ_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_OTYPER;
#[doc = "`read()` method returns [gpioz_otyper::R](gpioz_otyper::R) reader structure"]
impl crate::Readable for GPIOZ_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpioz_otyper::W](gpioz_otyper::W) writer structure"]
impl crate::Writable for GPIOZ_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpioz_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_ospeedr](gpioz_ospeedr) module"]
pub type GPIOZ_OSPEEDR = crate::Reg<u32, _GPIOZ_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_OSPEEDR;
#[doc = "`read()` method returns [gpioz_ospeedr::R](gpioz_ospeedr::R) reader structure"]
impl crate::Readable for GPIOZ_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpioz_ospeedr::W](gpioz_ospeedr::W) writer structure"]
impl crate::Writable for GPIOZ_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpioz_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_pupdr](gpioz_pupdr) module"]
pub type GPIOZ_PUPDR = crate::Reg<u32, _GPIOZ_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_PUPDR;
#[doc = "`read()` method returns [gpioz_pupdr::R](gpioz_pupdr::R) reader structure"]
impl crate::Readable for GPIOZ_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpioz_pupdr::W](gpioz_pupdr::W) writer structure"]
impl crate::Writable for GPIOZ_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioz_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_idr](gpioz_idr) module"]
pub type GPIOZ_IDR = crate::Reg<u32, _GPIOZ_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_IDR;
#[doc = "`read()` method returns [gpioz_idr::R](gpioz_idr::R) reader structure"]
impl crate::Readable for GPIOZ_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpioz_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_odr](gpioz_odr) module"]
pub type GPIOZ_ODR = crate::Reg<u32, _GPIOZ_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_ODR;
#[doc = "`read()` method returns [gpioz_odr::R](gpioz_odr::R) reader structure"]
impl crate::Readable for GPIOZ_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioz_odr::W](gpioz_odr::W) writer structure"]
impl crate::Writable for GPIOZ_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpioz_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_bsrr](gpioz_bsrr) module"]
pub type GPIOZ_BSRR = crate::Reg<u32, _GPIOZ_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_BSRR;
#[doc = "`write(|w| ..)` method takes [gpioz_bsrr::W](gpioz_bsrr::W) writer structure"]
impl crate::Writable for GPIOZ_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpioz_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_lckr](gpioz_lckr) module"]
pub type GPIOZ_LCKR = crate::Reg<u32, _GPIOZ_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_LCKR;
#[doc = "`read()` method returns [gpioz_lckr::R](gpioz_lckr::R) reader structure"]
impl crate::Readable for GPIOZ_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpioz_lckr::W](gpioz_lckr::W) writer structure"]
impl crate::Writable for GPIOZ_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioz_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_afrl](gpioz_afrl) module"]
pub type GPIOZ_AFRL = crate::Reg<u32, _GPIOZ_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_AFRL;
#[doc = "`read()` method returns [gpioz_afrl::R](gpioz_afrl::R) reader structure"]
impl crate::Readable for GPIOZ_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpioz_afrl::W](gpioz_afrl::W) writer structure"]
impl crate::Writable for GPIOZ_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpioz_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_afrh](gpioz_afrh) module"]
pub type GPIOZ_AFRH = crate::Reg<u32, _GPIOZ_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_AFRH;
#[doc = "`read()` method returns [gpioz_afrh::R](gpioz_afrh::R) reader structure"]
impl crate::Readable for GPIOZ_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpioz_afrh::W](gpioz_afrh::W) writer structure"]
impl crate::Writable for GPIOZ_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpioz_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_brr](gpioz_brr) module"]
pub type GPIOZ_BRR = crate::Reg<u32, _GPIOZ_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_BRR;
#[doc = "`write(|w| ..)` method takes [gpioz_brr::W](gpioz_brr::W) writer structure"]
impl crate::Writable for GPIOZ_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpioz_brr;
#[doc = "This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_seccfgr](gpioz_seccfgr) module"]
pub type GPIOZ_SECCFGR = crate::Reg<u32, _GPIOZ_SECCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_SECCFGR;
#[doc = "`write(|w| ..)` method takes [gpioz_seccfgr::W](gpioz_seccfgr::W) writer structure"]
impl crate::Writable for GPIOZ_SECCFGR {}
#[doc = "This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded."]
pub mod gpioz_seccfgr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr10](gpioz_hwcfgr10) module"]
pub type GPIOZ_HWCFGR10 = crate::Reg<u32, _GPIOZ_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR10;
#[doc = "`read()` method returns [gpioz_hwcfgr10::R](gpioz_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioz_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr9](gpioz_hwcfgr9) module"]
pub type GPIOZ_HWCFGR9 = crate::Reg<u32, _GPIOZ_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR9;
#[doc = "`read()` method returns [gpioz_hwcfgr9::R](gpioz_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioz_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr8](gpioz_hwcfgr8) module"]
pub type GPIOZ_HWCFGR8 = crate::Reg<u32, _GPIOZ_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR8;
#[doc = "`read()` method returns [gpioz_hwcfgr8::R](gpioz_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioz_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr7](gpioz_hwcfgr7) module"]
pub type GPIOZ_HWCFGR7 = crate::Reg<u32, _GPIOZ_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR7;
#[doc = "`read()` method returns [gpioz_hwcfgr7::R](gpioz_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioz_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr6](gpioz_hwcfgr6) module"]
pub type GPIOZ_HWCFGR6 = crate::Reg<u32, _GPIOZ_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR6;
#[doc = "`read()` method returns [gpioz_hwcfgr6::R](gpioz_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioz_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr5](gpioz_hwcfgr5) module"]
pub type GPIOZ_HWCFGR5 = crate::Reg<u32, _GPIOZ_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR5;
#[doc = "`read()` method returns [gpioz_hwcfgr5::R](gpioz_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioz_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr4](gpioz_hwcfgr4) module"]
pub type GPIOZ_HWCFGR4 = crate::Reg<u32, _GPIOZ_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR4;
#[doc = "`read()` method returns [gpioz_hwcfgr4::R](gpioz_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioz_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr3](gpioz_hwcfgr3) module"]
pub type GPIOZ_HWCFGR3 = crate::Reg<u32, _GPIOZ_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR3;
#[doc = "`read()` method returns [gpioz_hwcfgr3::R](gpioz_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioz_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr2](gpioz_hwcfgr2) module"]
pub type GPIOZ_HWCFGR2 = crate::Reg<u32, _GPIOZ_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR2;
#[doc = "`read()` method returns [gpioz_hwcfgr2::R](gpioz_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioz_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr1](gpioz_hwcfgr1) module"]
pub type GPIOZ_HWCFGR1 = crate::Reg<u32, _GPIOZ_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR1;
#[doc = "`read()` method returns [gpioz_hwcfgr1::R](gpioz_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioz_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr0](gpioz_hwcfgr0) module"]
pub type GPIOZ_HWCFGR0 = crate::Reg<u32, _GPIOZ_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_HWCFGR0;
#[doc = "`read()` method returns [gpioz_hwcfgr0::R](gpioz_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioz_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_verr](gpioz_verr) module"]
pub type GPIOZ_VERR = crate::Reg<u32, _GPIOZ_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_VERR;
#[doc = "`read()` method returns [gpioz_verr::R](gpioz_verr::R) reader structure"]
impl crate::Readable for GPIOZ_VERR {}
#[doc = "GPIO version register"]
pub mod gpioz_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_ipidr](gpioz_ipidr) module"]
pub type GPIOZ_IPIDR = crate::Reg<u32, _GPIOZ_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_IPIDR;
#[doc = "`read()` method returns [gpioz_ipidr::R](gpioz_ipidr::R) reader structure"]
impl crate::Readable for GPIOZ_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpioz_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_sidr](gpioz_sidr) module"]
pub type GPIOZ_SIDR = crate::Reg<u32, _GPIOZ_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOZ_SIDR;
#[doc = "`read()` method returns [gpioz_sidr::R](gpioz_sidr::R) reader structure"]
impl crate::Readable for GPIOZ_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpioz_sidr;
