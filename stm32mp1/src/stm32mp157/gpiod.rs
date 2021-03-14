#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiod_moder: GPIOD_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiod_otyper: GPIOD_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiod_ospeedr: GPIOD_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiod_pupdr: GPIOD_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiod_idr: GPIOD_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiod_odr: GPIOD_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiod_bsrr: GPIOD_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiod_lckr: GPIOD_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiod_afrl: GPIOD_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiod_afrh: GPIOD_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiod_brr: GPIOD_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiod_hwcfgr10: GPIOD_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiod_hwcfgr9: GPIOD_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiod_hwcfgr8: GPIOD_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiod_hwcfgr7: GPIOD_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiod_hwcfgr6: GPIOD_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiod_hwcfgr5: GPIOD_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiod_hwcfgr4: GPIOD_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiod_hwcfgr3: GPIOD_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiod_hwcfgr2: GPIOD_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiod_hwcfgr1: GPIOD_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiod_hwcfgr0: GPIOD_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiod_verr: GPIOD_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiod_ipidr: GPIOD_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiod_sidr: GPIOD_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_moder](gpiod_moder) module"]
pub type GPIOD_MODER = crate::Reg<u32, _GPIOD_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_MODER;
#[doc = "`read()` method returns [gpiod_moder::R](gpiod_moder::R) reader structure"]
impl crate::Readable for GPIOD_MODER {}
#[doc = "`write(|w| ..)` method takes [gpiod_moder::W](gpiod_moder::W) writer structure"]
impl crate::Writable for GPIOD_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpiod_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_otyper](gpiod_otyper) module"]
pub type GPIOD_OTYPER = crate::Reg<u32, _GPIOD_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_OTYPER;
#[doc = "`read()` method returns [gpiod_otyper::R](gpiod_otyper::R) reader structure"]
impl crate::Readable for GPIOD_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpiod_otyper::W](gpiod_otyper::W) writer structure"]
impl crate::Writable for GPIOD_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpiod_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_ospeedr](gpiod_ospeedr) module"]
pub type GPIOD_OSPEEDR = crate::Reg<u32, _GPIOD_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_OSPEEDR;
#[doc = "`read()` method returns [gpiod_ospeedr::R](gpiod_ospeedr::R) reader structure"]
impl crate::Readable for GPIOD_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpiod_ospeedr::W](gpiod_ospeedr::W) writer structure"]
impl crate::Writable for GPIOD_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpiod_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_pupdr](gpiod_pupdr) module"]
pub type GPIOD_PUPDR = crate::Reg<u32, _GPIOD_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_PUPDR;
#[doc = "`read()` method returns [gpiod_pupdr::R](gpiod_pupdr::R) reader structure"]
impl crate::Readable for GPIOD_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpiod_pupdr::W](gpiod_pupdr::W) writer structure"]
impl crate::Writable for GPIOD_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiod_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_idr](gpiod_idr) module"]
pub type GPIOD_IDR = crate::Reg<u32, _GPIOD_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_IDR;
#[doc = "`read()` method returns [gpiod_idr::R](gpiod_idr::R) reader structure"]
impl crate::Readable for GPIOD_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpiod_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_odr](gpiod_odr) module"]
pub type GPIOD_ODR = crate::Reg<u32, _GPIOD_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_ODR;
#[doc = "`read()` method returns [gpiod_odr::R](gpiod_odr::R) reader structure"]
impl crate::Readable for GPIOD_ODR {}
#[doc = "`write(|w| ..)` method takes [gpiod_odr::W](gpiod_odr::W) writer structure"]
impl crate::Writable for GPIOD_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpiod_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_bsrr](gpiod_bsrr) module"]
pub type GPIOD_BSRR = crate::Reg<u32, _GPIOD_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_BSRR;
#[doc = "`write(|w| ..)` method takes [gpiod_bsrr::W](gpiod_bsrr::W) writer structure"]
impl crate::Writable for GPIOD_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpiod_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_lckr](gpiod_lckr) module"]
pub type GPIOD_LCKR = crate::Reg<u32, _GPIOD_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_LCKR;
#[doc = "`read()` method returns [gpiod_lckr::R](gpiod_lckr::R) reader structure"]
impl crate::Readable for GPIOD_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpiod_lckr::W](gpiod_lckr::W) writer structure"]
impl crate::Writable for GPIOD_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiod_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_afrl](gpiod_afrl) module"]
pub type GPIOD_AFRL = crate::Reg<u32, _GPIOD_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_AFRL;
#[doc = "`read()` method returns [gpiod_afrl::R](gpiod_afrl::R) reader structure"]
impl crate::Readable for GPIOD_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpiod_afrl::W](gpiod_afrl::W) writer structure"]
impl crate::Writable for GPIOD_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpiod_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_afrh](gpiod_afrh) module"]
pub type GPIOD_AFRH = crate::Reg<u32, _GPIOD_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_AFRH;
#[doc = "`read()` method returns [gpiod_afrh::R](gpiod_afrh::R) reader structure"]
impl crate::Readable for GPIOD_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpiod_afrh::W](gpiod_afrh::W) writer structure"]
impl crate::Writable for GPIOD_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpiod_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_brr](gpiod_brr) module"]
pub type GPIOD_BRR = crate::Reg<u32, _GPIOD_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_BRR;
#[doc = "`write(|w| ..)` method takes [gpiod_brr::W](gpiod_brr::W) writer structure"]
impl crate::Writable for GPIOD_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpiod_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr10](gpiod_hwcfgr10) module"]
pub type GPIOD_HWCFGR10 = crate::Reg<u32, _GPIOD_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR10;
#[doc = "`read()` method returns [gpiod_hwcfgr10::R](gpiod_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiod_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr9](gpiod_hwcfgr9) module"]
pub type GPIOD_HWCFGR9 = crate::Reg<u32, _GPIOD_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR9;
#[doc = "`read()` method returns [gpiod_hwcfgr9::R](gpiod_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiod_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr8](gpiod_hwcfgr8) module"]
pub type GPIOD_HWCFGR8 = crate::Reg<u32, _GPIOD_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR8;
#[doc = "`read()` method returns [gpiod_hwcfgr8::R](gpiod_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiod_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr7](gpiod_hwcfgr7) module"]
pub type GPIOD_HWCFGR7 = crate::Reg<u32, _GPIOD_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR7;
#[doc = "`read()` method returns [gpiod_hwcfgr7::R](gpiod_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiod_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr6](gpiod_hwcfgr6) module"]
pub type GPIOD_HWCFGR6 = crate::Reg<u32, _GPIOD_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR6;
#[doc = "`read()` method returns [gpiod_hwcfgr6::R](gpiod_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiod_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr5](gpiod_hwcfgr5) module"]
pub type GPIOD_HWCFGR5 = crate::Reg<u32, _GPIOD_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR5;
#[doc = "`read()` method returns [gpiod_hwcfgr5::R](gpiod_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiod_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr4](gpiod_hwcfgr4) module"]
pub type GPIOD_HWCFGR4 = crate::Reg<u32, _GPIOD_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR4;
#[doc = "`read()` method returns [gpiod_hwcfgr4::R](gpiod_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiod_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr3](gpiod_hwcfgr3) module"]
pub type GPIOD_HWCFGR3 = crate::Reg<u32, _GPIOD_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR3;
#[doc = "`read()` method returns [gpiod_hwcfgr3::R](gpiod_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiod_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr2](gpiod_hwcfgr2) module"]
pub type GPIOD_HWCFGR2 = crate::Reg<u32, _GPIOD_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR2;
#[doc = "`read()` method returns [gpiod_hwcfgr2::R](gpiod_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiod_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr1](gpiod_hwcfgr1) module"]
pub type GPIOD_HWCFGR1 = crate::Reg<u32, _GPIOD_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR1;
#[doc = "`read()` method returns [gpiod_hwcfgr1::R](gpiod_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiod_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_hwcfgr0](gpiod_hwcfgr0) module"]
pub type GPIOD_HWCFGR0 = crate::Reg<u32, _GPIOD_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_HWCFGR0;
#[doc = "`read()` method returns [gpiod_hwcfgr0::R](gpiod_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOD_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiod_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_verr](gpiod_verr) module"]
pub type GPIOD_VERR = crate::Reg<u32, _GPIOD_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_VERR;
#[doc = "`read()` method returns [gpiod_verr::R](gpiod_verr::R) reader structure"]
impl crate::Readable for GPIOD_VERR {}
#[doc = "GPIO version register"]
pub mod gpiod_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_ipidr](gpiod_ipidr) module"]
pub type GPIOD_IPIDR = crate::Reg<u32, _GPIOD_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_IPIDR;
#[doc = "`read()` method returns [gpiod_ipidr::R](gpiod_ipidr::R) reader structure"]
impl crate::Readable for GPIOD_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpiod_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_sidr](gpiod_sidr) module"]
pub type GPIOD_SIDR = crate::Reg<u32, _GPIOD_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_SIDR;
#[doc = "`read()` method returns [gpiod_sidr::R](gpiod_sidr::R) reader structure"]
impl crate::Readable for GPIOD_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpiod_sidr;
