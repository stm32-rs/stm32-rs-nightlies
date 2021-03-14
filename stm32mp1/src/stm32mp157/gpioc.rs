#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioc_moder: GPIOC_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioc_otyper: GPIOC_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioc_ospeedr: GPIOC_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioc_pupdr: GPIOC_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioc_idr: GPIOC_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioc_odr: GPIOC_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioc_bsrr: GPIOC_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioc_lckr: GPIOC_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioc_afrl: GPIOC_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioc_afrh: GPIOC_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioc_brr: GPIOC_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioc_hwcfgr10: GPIOC_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioc_hwcfgr9: GPIOC_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioc_hwcfgr8: GPIOC_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioc_hwcfgr7: GPIOC_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioc_hwcfgr6: GPIOC_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioc_hwcfgr5: GPIOC_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioc_hwcfgr4: GPIOC_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioc_hwcfgr3: GPIOC_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioc_hwcfgr2: GPIOC_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioc_hwcfgr1: GPIOC_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioc_hwcfgr0: GPIOC_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioc_verr: GPIOC_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioc_ipidr: GPIOC_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioc_sidr: GPIOC_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_moder](gpioc_moder) module"]
pub type GPIOC_MODER = crate::Reg<u32, _GPIOC_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_MODER;
#[doc = "`read()` method returns [gpioc_moder::R](gpioc_moder::R) reader structure"]
impl crate::Readable for GPIOC_MODER {}
#[doc = "`write(|w| ..)` method takes [gpioc_moder::W](gpioc_moder::W) writer structure"]
impl crate::Writable for GPIOC_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpioc_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_otyper](gpioc_otyper) module"]
pub type GPIOC_OTYPER = crate::Reg<u32, _GPIOC_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_OTYPER;
#[doc = "`read()` method returns [gpioc_otyper::R](gpioc_otyper::R) reader structure"]
impl crate::Readable for GPIOC_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpioc_otyper::W](gpioc_otyper::W) writer structure"]
impl crate::Writable for GPIOC_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpioc_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_ospeedr](gpioc_ospeedr) module"]
pub type GPIOC_OSPEEDR = crate::Reg<u32, _GPIOC_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_OSPEEDR;
#[doc = "`read()` method returns [gpioc_ospeedr::R](gpioc_ospeedr::R) reader structure"]
impl crate::Readable for GPIOC_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpioc_ospeedr::W](gpioc_ospeedr::W) writer structure"]
impl crate::Writable for GPIOC_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpioc_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_pupdr](gpioc_pupdr) module"]
pub type GPIOC_PUPDR = crate::Reg<u32, _GPIOC_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_PUPDR;
#[doc = "`read()` method returns [gpioc_pupdr::R](gpioc_pupdr::R) reader structure"]
impl crate::Readable for GPIOC_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpioc_pupdr::W](gpioc_pupdr::W) writer structure"]
impl crate::Writable for GPIOC_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioc_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_idr](gpioc_idr) module"]
pub type GPIOC_IDR = crate::Reg<u32, _GPIOC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_IDR;
#[doc = "`read()` method returns [gpioc_idr::R](gpioc_idr::R) reader structure"]
impl crate::Readable for GPIOC_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpioc_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_odr](gpioc_odr) module"]
pub type GPIOC_ODR = crate::Reg<u32, _GPIOC_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_ODR;
#[doc = "`read()` method returns [gpioc_odr::R](gpioc_odr::R) reader structure"]
impl crate::Readable for GPIOC_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioc_odr::W](gpioc_odr::W) writer structure"]
impl crate::Writable for GPIOC_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpioc_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_bsrr](gpioc_bsrr) module"]
pub type GPIOC_BSRR = crate::Reg<u32, _GPIOC_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_BSRR;
#[doc = "`write(|w| ..)` method takes [gpioc_bsrr::W](gpioc_bsrr::W) writer structure"]
impl crate::Writable for GPIOC_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpioc_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_lckr](gpioc_lckr) module"]
pub type GPIOC_LCKR = crate::Reg<u32, _GPIOC_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_LCKR;
#[doc = "`read()` method returns [gpioc_lckr::R](gpioc_lckr::R) reader structure"]
impl crate::Readable for GPIOC_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpioc_lckr::W](gpioc_lckr::W) writer structure"]
impl crate::Writable for GPIOC_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioc_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_afrl](gpioc_afrl) module"]
pub type GPIOC_AFRL = crate::Reg<u32, _GPIOC_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_AFRL;
#[doc = "`read()` method returns [gpioc_afrl::R](gpioc_afrl::R) reader structure"]
impl crate::Readable for GPIOC_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpioc_afrl::W](gpioc_afrl::W) writer structure"]
impl crate::Writable for GPIOC_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpioc_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_afrh](gpioc_afrh) module"]
pub type GPIOC_AFRH = crate::Reg<u32, _GPIOC_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_AFRH;
#[doc = "`read()` method returns [gpioc_afrh::R](gpioc_afrh::R) reader structure"]
impl crate::Readable for GPIOC_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpioc_afrh::W](gpioc_afrh::W) writer structure"]
impl crate::Writable for GPIOC_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpioc_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_brr](gpioc_brr) module"]
pub type GPIOC_BRR = crate::Reg<u32, _GPIOC_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_BRR;
#[doc = "`write(|w| ..)` method takes [gpioc_brr::W](gpioc_brr::W) writer structure"]
impl crate::Writable for GPIOC_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpioc_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr10](gpioc_hwcfgr10) module"]
pub type GPIOC_HWCFGR10 = crate::Reg<u32, _GPIOC_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR10;
#[doc = "`read()` method returns [gpioc_hwcfgr10::R](gpioc_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioc_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr9](gpioc_hwcfgr9) module"]
pub type GPIOC_HWCFGR9 = crate::Reg<u32, _GPIOC_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR9;
#[doc = "`read()` method returns [gpioc_hwcfgr9::R](gpioc_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioc_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr8](gpioc_hwcfgr8) module"]
pub type GPIOC_HWCFGR8 = crate::Reg<u32, _GPIOC_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR8;
#[doc = "`read()` method returns [gpioc_hwcfgr8::R](gpioc_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioc_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr7](gpioc_hwcfgr7) module"]
pub type GPIOC_HWCFGR7 = crate::Reg<u32, _GPIOC_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR7;
#[doc = "`read()` method returns [gpioc_hwcfgr7::R](gpioc_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioc_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr6](gpioc_hwcfgr6) module"]
pub type GPIOC_HWCFGR6 = crate::Reg<u32, _GPIOC_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR6;
#[doc = "`read()` method returns [gpioc_hwcfgr6::R](gpioc_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioc_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr5](gpioc_hwcfgr5) module"]
pub type GPIOC_HWCFGR5 = crate::Reg<u32, _GPIOC_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR5;
#[doc = "`read()` method returns [gpioc_hwcfgr5::R](gpioc_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioc_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr4](gpioc_hwcfgr4) module"]
pub type GPIOC_HWCFGR4 = crate::Reg<u32, _GPIOC_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR4;
#[doc = "`read()` method returns [gpioc_hwcfgr4::R](gpioc_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioc_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr3](gpioc_hwcfgr3) module"]
pub type GPIOC_HWCFGR3 = crate::Reg<u32, _GPIOC_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR3;
#[doc = "`read()` method returns [gpioc_hwcfgr3::R](gpioc_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioc_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr2](gpioc_hwcfgr2) module"]
pub type GPIOC_HWCFGR2 = crate::Reg<u32, _GPIOC_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR2;
#[doc = "`read()` method returns [gpioc_hwcfgr2::R](gpioc_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioc_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr1](gpioc_hwcfgr1) module"]
pub type GPIOC_HWCFGR1 = crate::Reg<u32, _GPIOC_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR1;
#[doc = "`read()` method returns [gpioc_hwcfgr1::R](gpioc_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioc_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr0](gpioc_hwcfgr0) module"]
pub type GPIOC_HWCFGR0 = crate::Reg<u32, _GPIOC_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_HWCFGR0;
#[doc = "`read()` method returns [gpioc_hwcfgr0::R](gpioc_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioc_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_verr](gpioc_verr) module"]
pub type GPIOC_VERR = crate::Reg<u32, _GPIOC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_VERR;
#[doc = "`read()` method returns [gpioc_verr::R](gpioc_verr::R) reader structure"]
impl crate::Readable for GPIOC_VERR {}
#[doc = "GPIO version register"]
pub mod gpioc_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_ipidr](gpioc_ipidr) module"]
pub type GPIOC_IPIDR = crate::Reg<u32, _GPIOC_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_IPIDR;
#[doc = "`read()` method returns [gpioc_ipidr::R](gpioc_ipidr::R) reader structure"]
impl crate::Readable for GPIOC_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpioc_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_sidr](gpioc_sidr) module"]
pub type GPIOC_SIDR = crate::Reg<u32, _GPIOC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_SIDR;
#[doc = "`read()` method returns [gpioc_sidr::R](gpioc_sidr::R) reader structure"]
impl crate::Readable for GPIOC_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpioc_sidr;
