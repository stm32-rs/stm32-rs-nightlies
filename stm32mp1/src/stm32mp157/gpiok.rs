#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiok_moder: GPIOK_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiok_otyper: GPIOK_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiok_ospeedr: GPIOK_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiok_pupdr: GPIOK_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiok_idr: GPIOK_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiok_odr: GPIOK_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiok_bsrr: GPIOK_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiok_lckr: GPIOK_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiok_afrl: GPIOK_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiok_afrh: GPIOK_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiok_brr: GPIOK_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiok_hwcfgr10: GPIOK_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiok_hwcfgr9: GPIOK_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiok_hwcfgr8: GPIOK_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiok_hwcfgr7: GPIOK_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiok_hwcfgr6: GPIOK_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiok_hwcfgr5: GPIOK_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiok_hwcfgr4: GPIOK_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiok_hwcfgr3: GPIOK_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiok_hwcfgr2: GPIOK_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiok_hwcfgr1: GPIOK_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiok_hwcfgr0: GPIOK_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiok_verr: GPIOK_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiok_ipidr: GPIOK_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiok_sidr: GPIOK_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_moder](gpiok_moder) module"]
pub type GPIOK_MODER = crate::Reg<u32, _GPIOK_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_MODER;
#[doc = "`read()` method returns [gpiok_moder::R](gpiok_moder::R) reader structure"]
impl crate::Readable for GPIOK_MODER {}
#[doc = "`write(|w| ..)` method takes [gpiok_moder::W](gpiok_moder::W) writer structure"]
impl crate::Writable for GPIOK_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpiok_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_otyper](gpiok_otyper) module"]
pub type GPIOK_OTYPER = crate::Reg<u32, _GPIOK_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_OTYPER;
#[doc = "`read()` method returns [gpiok_otyper::R](gpiok_otyper::R) reader structure"]
impl crate::Readable for GPIOK_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpiok_otyper::W](gpiok_otyper::W) writer structure"]
impl crate::Writable for GPIOK_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpiok_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_ospeedr](gpiok_ospeedr) module"]
pub type GPIOK_OSPEEDR = crate::Reg<u32, _GPIOK_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_OSPEEDR;
#[doc = "`read()` method returns [gpiok_ospeedr::R](gpiok_ospeedr::R) reader structure"]
impl crate::Readable for GPIOK_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpiok_ospeedr::W](gpiok_ospeedr::W) writer structure"]
impl crate::Writable for GPIOK_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpiok_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_pupdr](gpiok_pupdr) module"]
pub type GPIOK_PUPDR = crate::Reg<u32, _GPIOK_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_PUPDR;
#[doc = "`read()` method returns [gpiok_pupdr::R](gpiok_pupdr::R) reader structure"]
impl crate::Readable for GPIOK_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpiok_pupdr::W](gpiok_pupdr::W) writer structure"]
impl crate::Writable for GPIOK_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiok_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_idr](gpiok_idr) module"]
pub type GPIOK_IDR = crate::Reg<u32, _GPIOK_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_IDR;
#[doc = "`read()` method returns [gpiok_idr::R](gpiok_idr::R) reader structure"]
impl crate::Readable for GPIOK_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpiok_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_odr](gpiok_odr) module"]
pub type GPIOK_ODR = crate::Reg<u32, _GPIOK_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_ODR;
#[doc = "`read()` method returns [gpiok_odr::R](gpiok_odr::R) reader structure"]
impl crate::Readable for GPIOK_ODR {}
#[doc = "`write(|w| ..)` method takes [gpiok_odr::W](gpiok_odr::W) writer structure"]
impl crate::Writable for GPIOK_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpiok_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_bsrr](gpiok_bsrr) module"]
pub type GPIOK_BSRR = crate::Reg<u32, _GPIOK_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_BSRR;
#[doc = "`write(|w| ..)` method takes [gpiok_bsrr::W](gpiok_bsrr::W) writer structure"]
impl crate::Writable for GPIOK_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpiok_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_lckr](gpiok_lckr) module"]
pub type GPIOK_LCKR = crate::Reg<u32, _GPIOK_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_LCKR;
#[doc = "`read()` method returns [gpiok_lckr::R](gpiok_lckr::R) reader structure"]
impl crate::Readable for GPIOK_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpiok_lckr::W](gpiok_lckr::W) writer structure"]
impl crate::Writable for GPIOK_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiok_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_afrl](gpiok_afrl) module"]
pub type GPIOK_AFRL = crate::Reg<u32, _GPIOK_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_AFRL;
#[doc = "`read()` method returns [gpiok_afrl::R](gpiok_afrl::R) reader structure"]
impl crate::Readable for GPIOK_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpiok_afrl::W](gpiok_afrl::W) writer structure"]
impl crate::Writable for GPIOK_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpiok_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_afrh](gpiok_afrh) module"]
pub type GPIOK_AFRH = crate::Reg<u32, _GPIOK_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_AFRH;
#[doc = "`read()` method returns [gpiok_afrh::R](gpiok_afrh::R) reader structure"]
impl crate::Readable for GPIOK_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpiok_afrh::W](gpiok_afrh::W) writer structure"]
impl crate::Writable for GPIOK_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpiok_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_brr](gpiok_brr) module"]
pub type GPIOK_BRR = crate::Reg<u32, _GPIOK_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_BRR;
#[doc = "`write(|w| ..)` method takes [gpiok_brr::W](gpiok_brr::W) writer structure"]
impl crate::Writable for GPIOK_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpiok_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr10](gpiok_hwcfgr10) module"]
pub type GPIOK_HWCFGR10 = crate::Reg<u32, _GPIOK_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR10;
#[doc = "`read()` method returns [gpiok_hwcfgr10::R](gpiok_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiok_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr9](gpiok_hwcfgr9) module"]
pub type GPIOK_HWCFGR9 = crate::Reg<u32, _GPIOK_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR9;
#[doc = "`read()` method returns [gpiok_hwcfgr9::R](gpiok_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiok_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr8](gpiok_hwcfgr8) module"]
pub type GPIOK_HWCFGR8 = crate::Reg<u32, _GPIOK_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR8;
#[doc = "`read()` method returns [gpiok_hwcfgr8::R](gpiok_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiok_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr7](gpiok_hwcfgr7) module"]
pub type GPIOK_HWCFGR7 = crate::Reg<u32, _GPIOK_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR7;
#[doc = "`read()` method returns [gpiok_hwcfgr7::R](gpiok_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiok_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr6](gpiok_hwcfgr6) module"]
pub type GPIOK_HWCFGR6 = crate::Reg<u32, _GPIOK_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR6;
#[doc = "`read()` method returns [gpiok_hwcfgr6::R](gpiok_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiok_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr5](gpiok_hwcfgr5) module"]
pub type GPIOK_HWCFGR5 = crate::Reg<u32, _GPIOK_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR5;
#[doc = "`read()` method returns [gpiok_hwcfgr5::R](gpiok_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiok_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr4](gpiok_hwcfgr4) module"]
pub type GPIOK_HWCFGR4 = crate::Reg<u32, _GPIOK_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR4;
#[doc = "`read()` method returns [gpiok_hwcfgr4::R](gpiok_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiok_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr3](gpiok_hwcfgr3) module"]
pub type GPIOK_HWCFGR3 = crate::Reg<u32, _GPIOK_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR3;
#[doc = "`read()` method returns [gpiok_hwcfgr3::R](gpiok_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiok_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr2](gpiok_hwcfgr2) module"]
pub type GPIOK_HWCFGR2 = crate::Reg<u32, _GPIOK_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR2;
#[doc = "`read()` method returns [gpiok_hwcfgr2::R](gpiok_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiok_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr1](gpiok_hwcfgr1) module"]
pub type GPIOK_HWCFGR1 = crate::Reg<u32, _GPIOK_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR1;
#[doc = "`read()` method returns [gpiok_hwcfgr1::R](gpiok_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiok_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr0](gpiok_hwcfgr0) module"]
pub type GPIOK_HWCFGR0 = crate::Reg<u32, _GPIOK_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_HWCFGR0;
#[doc = "`read()` method returns [gpiok_hwcfgr0::R](gpiok_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiok_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_verr](gpiok_verr) module"]
pub type GPIOK_VERR = crate::Reg<u32, _GPIOK_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_VERR;
#[doc = "`read()` method returns [gpiok_verr::R](gpiok_verr::R) reader structure"]
impl crate::Readable for GPIOK_VERR {}
#[doc = "GPIO version register"]
pub mod gpiok_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_ipidr](gpiok_ipidr) module"]
pub type GPIOK_IPIDR = crate::Reg<u32, _GPIOK_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_IPIDR;
#[doc = "`read()` method returns [gpiok_ipidr::R](gpiok_ipidr::R) reader structure"]
impl crate::Readable for GPIOK_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpiok_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_sidr](gpiok_sidr) module"]
pub type GPIOK_SIDR = crate::Reg<u32, _GPIOK_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOK_SIDR;
#[doc = "`read()` method returns [gpiok_sidr::R](gpiok_sidr::R) reader structure"]
impl crate::Readable for GPIOK_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpiok_sidr;
