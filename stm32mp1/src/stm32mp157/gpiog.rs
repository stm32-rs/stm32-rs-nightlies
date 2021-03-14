#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiog_moder: GPIOG_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiog_otyper: GPIOG_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiog_ospeedr: GPIOG_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiog_pupdr: GPIOG_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiog_idr: GPIOG_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiog_odr: GPIOG_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiog_bsrr: GPIOG_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiog_lckr: GPIOG_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiog_afrl: GPIOG_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiog_afrh: GPIOG_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiog_brr: GPIOG_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiog_hwcfgr10: GPIOG_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiog_hwcfgr9: GPIOG_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiog_hwcfgr8: GPIOG_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiog_hwcfgr7: GPIOG_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiog_hwcfgr6: GPIOG_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiog_hwcfgr5: GPIOG_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiog_hwcfgr4: GPIOG_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiog_hwcfgr3: GPIOG_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiog_hwcfgr2: GPIOG_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiog_hwcfgr1: GPIOG_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiog_hwcfgr0: GPIOG_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiog_verr: GPIOG_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiog_ipidr: GPIOG_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiog_sidr: GPIOG_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_moder](gpiog_moder) module"]
pub type GPIOG_MODER = crate::Reg<u32, _GPIOG_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_MODER;
#[doc = "`read()` method returns [gpiog_moder::R](gpiog_moder::R) reader structure"]
impl crate::Readable for GPIOG_MODER {}
#[doc = "`write(|w| ..)` method takes [gpiog_moder::W](gpiog_moder::W) writer structure"]
impl crate::Writable for GPIOG_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpiog_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_otyper](gpiog_otyper) module"]
pub type GPIOG_OTYPER = crate::Reg<u32, _GPIOG_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_OTYPER;
#[doc = "`read()` method returns [gpiog_otyper::R](gpiog_otyper::R) reader structure"]
impl crate::Readable for GPIOG_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpiog_otyper::W](gpiog_otyper::W) writer structure"]
impl crate::Writable for GPIOG_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpiog_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_ospeedr](gpiog_ospeedr) module"]
pub type GPIOG_OSPEEDR = crate::Reg<u32, _GPIOG_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_OSPEEDR;
#[doc = "`read()` method returns [gpiog_ospeedr::R](gpiog_ospeedr::R) reader structure"]
impl crate::Readable for GPIOG_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpiog_ospeedr::W](gpiog_ospeedr::W) writer structure"]
impl crate::Writable for GPIOG_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpiog_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_pupdr](gpiog_pupdr) module"]
pub type GPIOG_PUPDR = crate::Reg<u32, _GPIOG_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_PUPDR;
#[doc = "`read()` method returns [gpiog_pupdr::R](gpiog_pupdr::R) reader structure"]
impl crate::Readable for GPIOG_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpiog_pupdr::W](gpiog_pupdr::W) writer structure"]
impl crate::Writable for GPIOG_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiog_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_idr](gpiog_idr) module"]
pub type GPIOG_IDR = crate::Reg<u32, _GPIOG_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_IDR;
#[doc = "`read()` method returns [gpiog_idr::R](gpiog_idr::R) reader structure"]
impl crate::Readable for GPIOG_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpiog_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_odr](gpiog_odr) module"]
pub type GPIOG_ODR = crate::Reg<u32, _GPIOG_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_ODR;
#[doc = "`read()` method returns [gpiog_odr::R](gpiog_odr::R) reader structure"]
impl crate::Readable for GPIOG_ODR {}
#[doc = "`write(|w| ..)` method takes [gpiog_odr::W](gpiog_odr::W) writer structure"]
impl crate::Writable for GPIOG_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpiog_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_bsrr](gpiog_bsrr) module"]
pub type GPIOG_BSRR = crate::Reg<u32, _GPIOG_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_BSRR;
#[doc = "`write(|w| ..)` method takes [gpiog_bsrr::W](gpiog_bsrr::W) writer structure"]
impl crate::Writable for GPIOG_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpiog_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_lckr](gpiog_lckr) module"]
pub type GPIOG_LCKR = crate::Reg<u32, _GPIOG_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_LCKR;
#[doc = "`read()` method returns [gpiog_lckr::R](gpiog_lckr::R) reader structure"]
impl crate::Readable for GPIOG_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpiog_lckr::W](gpiog_lckr::W) writer structure"]
impl crate::Writable for GPIOG_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiog_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_afrl](gpiog_afrl) module"]
pub type GPIOG_AFRL = crate::Reg<u32, _GPIOG_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_AFRL;
#[doc = "`read()` method returns [gpiog_afrl::R](gpiog_afrl::R) reader structure"]
impl crate::Readable for GPIOG_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpiog_afrl::W](gpiog_afrl::W) writer structure"]
impl crate::Writable for GPIOG_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpiog_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_afrh](gpiog_afrh) module"]
pub type GPIOG_AFRH = crate::Reg<u32, _GPIOG_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_AFRH;
#[doc = "`read()` method returns [gpiog_afrh::R](gpiog_afrh::R) reader structure"]
impl crate::Readable for GPIOG_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpiog_afrh::W](gpiog_afrh::W) writer structure"]
impl crate::Writable for GPIOG_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpiog_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_brr](gpiog_brr) module"]
pub type GPIOG_BRR = crate::Reg<u32, _GPIOG_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_BRR;
#[doc = "`write(|w| ..)` method takes [gpiog_brr::W](gpiog_brr::W) writer structure"]
impl crate::Writable for GPIOG_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpiog_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr10](gpiog_hwcfgr10) module"]
pub type GPIOG_HWCFGR10 = crate::Reg<u32, _GPIOG_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR10;
#[doc = "`read()` method returns [gpiog_hwcfgr10::R](gpiog_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiog_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr9](gpiog_hwcfgr9) module"]
pub type GPIOG_HWCFGR9 = crate::Reg<u32, _GPIOG_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR9;
#[doc = "`read()` method returns [gpiog_hwcfgr9::R](gpiog_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiog_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr8](gpiog_hwcfgr8) module"]
pub type GPIOG_HWCFGR8 = crate::Reg<u32, _GPIOG_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR8;
#[doc = "`read()` method returns [gpiog_hwcfgr8::R](gpiog_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiog_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr7](gpiog_hwcfgr7) module"]
pub type GPIOG_HWCFGR7 = crate::Reg<u32, _GPIOG_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR7;
#[doc = "`read()` method returns [gpiog_hwcfgr7::R](gpiog_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiog_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr6](gpiog_hwcfgr6) module"]
pub type GPIOG_HWCFGR6 = crate::Reg<u32, _GPIOG_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR6;
#[doc = "`read()` method returns [gpiog_hwcfgr6::R](gpiog_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiog_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr5](gpiog_hwcfgr5) module"]
pub type GPIOG_HWCFGR5 = crate::Reg<u32, _GPIOG_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR5;
#[doc = "`read()` method returns [gpiog_hwcfgr5::R](gpiog_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiog_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr4](gpiog_hwcfgr4) module"]
pub type GPIOG_HWCFGR4 = crate::Reg<u32, _GPIOG_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR4;
#[doc = "`read()` method returns [gpiog_hwcfgr4::R](gpiog_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiog_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr3](gpiog_hwcfgr3) module"]
pub type GPIOG_HWCFGR3 = crate::Reg<u32, _GPIOG_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR3;
#[doc = "`read()` method returns [gpiog_hwcfgr3::R](gpiog_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiog_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr2](gpiog_hwcfgr2) module"]
pub type GPIOG_HWCFGR2 = crate::Reg<u32, _GPIOG_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR2;
#[doc = "`read()` method returns [gpiog_hwcfgr2::R](gpiog_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiog_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr1](gpiog_hwcfgr1) module"]
pub type GPIOG_HWCFGR1 = crate::Reg<u32, _GPIOG_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR1;
#[doc = "`read()` method returns [gpiog_hwcfgr1::R](gpiog_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiog_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_hwcfgr0](gpiog_hwcfgr0) module"]
pub type GPIOG_HWCFGR0 = crate::Reg<u32, _GPIOG_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_HWCFGR0;
#[doc = "`read()` method returns [gpiog_hwcfgr0::R](gpiog_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiog_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_verr](gpiog_verr) module"]
pub type GPIOG_VERR = crate::Reg<u32, _GPIOG_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_VERR;
#[doc = "`read()` method returns [gpiog_verr::R](gpiog_verr::R) reader structure"]
impl crate::Readable for GPIOG_VERR {}
#[doc = "GPIO version register"]
pub mod gpiog_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_ipidr](gpiog_ipidr) module"]
pub type GPIOG_IPIDR = crate::Reg<u32, _GPIOG_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_IPIDR;
#[doc = "`read()` method returns [gpiog_ipidr::R](gpiog_ipidr::R) reader structure"]
impl crate::Readable for GPIOG_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpiog_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_sidr](gpiog_sidr) module"]
pub type GPIOG_SIDR = crate::Reg<u32, _GPIOG_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOG_SIDR;
#[doc = "`read()` method returns [gpiog_sidr::R](gpiog_sidr::R) reader structure"]
impl crate::Readable for GPIOG_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpiog_sidr;
