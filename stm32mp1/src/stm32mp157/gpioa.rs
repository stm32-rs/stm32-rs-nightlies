#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioa_moder: GPIOA_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioa_otyper: GPIOA_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioa_ospeedr: GPIOA_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioa_pupdr: GPIOA_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioa_idr: GPIOA_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioa_odr: GPIOA_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioa_bsrr: GPIOA_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioa_lckr: GPIOA_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioa_afrl: GPIOA_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioa_afrh: GPIOA_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioa_brr: GPIOA_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioa_hwcfgr10: GPIOA_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioa_hwcfgr9: GPIOA_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioa_hwcfgr8: GPIOA_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioa_hwcfgr7: GPIOA_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioa_hwcfgr6: GPIOA_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioa_hwcfgr5: GPIOA_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioa_hwcfgr4: GPIOA_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioa_hwcfgr3: GPIOA_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioa_hwcfgr2: GPIOA_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioa_hwcfgr1: GPIOA_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioa_hwcfgr0: GPIOA_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioa_verr: GPIOA_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioa_ipidr: GPIOA_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioa_sidr: GPIOA_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_moder](gpioa_moder) module"]
pub type GPIOA_MODER = crate::Reg<u32, _GPIOA_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_MODER;
#[doc = "`read()` method returns [gpioa_moder::R](gpioa_moder::R) reader structure"]
impl crate::Readable for GPIOA_MODER {}
#[doc = "`write(|w| ..)` method takes [gpioa_moder::W](gpioa_moder::W) writer structure"]
impl crate::Writable for GPIOA_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpioa_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_otyper](gpioa_otyper) module"]
pub type GPIOA_OTYPER = crate::Reg<u32, _GPIOA_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_OTYPER;
#[doc = "`read()` method returns [gpioa_otyper::R](gpioa_otyper::R) reader structure"]
impl crate::Readable for GPIOA_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpioa_otyper::W](gpioa_otyper::W) writer structure"]
impl crate::Writable for GPIOA_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpioa_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_ospeedr](gpioa_ospeedr) module"]
pub type GPIOA_OSPEEDR = crate::Reg<u32, _GPIOA_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_OSPEEDR;
#[doc = "`read()` method returns [gpioa_ospeedr::R](gpioa_ospeedr::R) reader structure"]
impl crate::Readable for GPIOA_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpioa_ospeedr::W](gpioa_ospeedr::W) writer structure"]
impl crate::Writable for GPIOA_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpioa_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_pupdr](gpioa_pupdr) module"]
pub type GPIOA_PUPDR = crate::Reg<u32, _GPIOA_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_PUPDR;
#[doc = "`read()` method returns [gpioa_pupdr::R](gpioa_pupdr::R) reader structure"]
impl crate::Readable for GPIOA_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpioa_pupdr::W](gpioa_pupdr::W) writer structure"]
impl crate::Writable for GPIOA_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioa_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_idr](gpioa_idr) module"]
pub type GPIOA_IDR = crate::Reg<u32, _GPIOA_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_IDR;
#[doc = "`read()` method returns [gpioa_idr::R](gpioa_idr::R) reader structure"]
impl crate::Readable for GPIOA_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpioa_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_odr](gpioa_odr) module"]
pub type GPIOA_ODR = crate::Reg<u32, _GPIOA_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_ODR;
#[doc = "`read()` method returns [gpioa_odr::R](gpioa_odr::R) reader structure"]
impl crate::Readable for GPIOA_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioa_odr::W](gpioa_odr::W) writer structure"]
impl crate::Writable for GPIOA_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpioa_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_bsrr](gpioa_bsrr) module"]
pub type GPIOA_BSRR = crate::Reg<u32, _GPIOA_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_BSRR;
#[doc = "`write(|w| ..)` method takes [gpioa_bsrr::W](gpioa_bsrr::W) writer structure"]
impl crate::Writable for GPIOA_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpioa_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_lckr](gpioa_lckr) module"]
pub type GPIOA_LCKR = crate::Reg<u32, _GPIOA_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_LCKR;
#[doc = "`read()` method returns [gpioa_lckr::R](gpioa_lckr::R) reader structure"]
impl crate::Readable for GPIOA_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpioa_lckr::W](gpioa_lckr::W) writer structure"]
impl crate::Writable for GPIOA_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioa_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_afrl](gpioa_afrl) module"]
pub type GPIOA_AFRL = crate::Reg<u32, _GPIOA_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_AFRL;
#[doc = "`read()` method returns [gpioa_afrl::R](gpioa_afrl::R) reader structure"]
impl crate::Readable for GPIOA_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpioa_afrl::W](gpioa_afrl::W) writer structure"]
impl crate::Writable for GPIOA_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpioa_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_afrh](gpioa_afrh) module"]
pub type GPIOA_AFRH = crate::Reg<u32, _GPIOA_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_AFRH;
#[doc = "`read()` method returns [gpioa_afrh::R](gpioa_afrh::R) reader structure"]
impl crate::Readable for GPIOA_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpioa_afrh::W](gpioa_afrh::W) writer structure"]
impl crate::Writable for GPIOA_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpioa_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_brr](gpioa_brr) module"]
pub type GPIOA_BRR = crate::Reg<u32, _GPIOA_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_BRR;
#[doc = "`write(|w| ..)` method takes [gpioa_brr::W](gpioa_brr::W) writer structure"]
impl crate::Writable for GPIOA_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpioa_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr10](gpioa_hwcfgr10) module"]
pub type GPIOA_HWCFGR10 = crate::Reg<u32, _GPIOA_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR10;
#[doc = "`read()` method returns [gpioa_hwcfgr10::R](gpioa_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioa_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr9](gpioa_hwcfgr9) module"]
pub type GPIOA_HWCFGR9 = crate::Reg<u32, _GPIOA_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR9;
#[doc = "`read()` method returns [gpioa_hwcfgr9::R](gpioa_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioa_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr8](gpioa_hwcfgr8) module"]
pub type GPIOA_HWCFGR8 = crate::Reg<u32, _GPIOA_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR8;
#[doc = "`read()` method returns [gpioa_hwcfgr8::R](gpioa_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioa_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr7](gpioa_hwcfgr7) module"]
pub type GPIOA_HWCFGR7 = crate::Reg<u32, _GPIOA_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR7;
#[doc = "`read()` method returns [gpioa_hwcfgr7::R](gpioa_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioa_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr6](gpioa_hwcfgr6) module"]
pub type GPIOA_HWCFGR6 = crate::Reg<u32, _GPIOA_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR6;
#[doc = "`read()` method returns [gpioa_hwcfgr6::R](gpioa_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioa_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr5](gpioa_hwcfgr5) module"]
pub type GPIOA_HWCFGR5 = crate::Reg<u32, _GPIOA_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR5;
#[doc = "`read()` method returns [gpioa_hwcfgr5::R](gpioa_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioa_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr4](gpioa_hwcfgr4) module"]
pub type GPIOA_HWCFGR4 = crate::Reg<u32, _GPIOA_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR4;
#[doc = "`read()` method returns [gpioa_hwcfgr4::R](gpioa_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioa_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr3](gpioa_hwcfgr3) module"]
pub type GPIOA_HWCFGR3 = crate::Reg<u32, _GPIOA_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR3;
#[doc = "`read()` method returns [gpioa_hwcfgr3::R](gpioa_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioa_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr2](gpioa_hwcfgr2) module"]
pub type GPIOA_HWCFGR2 = crate::Reg<u32, _GPIOA_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR2;
#[doc = "`read()` method returns [gpioa_hwcfgr2::R](gpioa_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioa_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr1](gpioa_hwcfgr1) module"]
pub type GPIOA_HWCFGR1 = crate::Reg<u32, _GPIOA_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR1;
#[doc = "`read()` method returns [gpioa_hwcfgr1::R](gpioa_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioa_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_hwcfgr0](gpioa_hwcfgr0) module"]
pub type GPIOA_HWCFGR0 = crate::Reg<u32, _GPIOA_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_HWCFGR0;
#[doc = "`read()` method returns [gpioa_hwcfgr0::R](gpioa_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioa_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_verr](gpioa_verr) module"]
pub type GPIOA_VERR = crate::Reg<u32, _GPIOA_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_VERR;
#[doc = "`read()` method returns [gpioa_verr::R](gpioa_verr::R) reader structure"]
impl crate::Readable for GPIOA_VERR {}
#[doc = "GPIO version register"]
pub mod gpioa_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_ipidr](gpioa_ipidr) module"]
pub type GPIOA_IPIDR = crate::Reg<u32, _GPIOA_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_IPIDR;
#[doc = "`read()` method returns [gpioa_ipidr::R](gpioa_ipidr::R) reader structure"]
impl crate::Readable for GPIOA_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpioa_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_sidr](gpioa_sidr) module"]
pub type GPIOA_SIDR = crate::Reg<u32, _GPIOA_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_SIDR;
#[doc = "`read()` method returns [gpioa_sidr::R](gpioa_sidr::R) reader structure"]
impl crate::Readable for GPIOA_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpioa_sidr;
