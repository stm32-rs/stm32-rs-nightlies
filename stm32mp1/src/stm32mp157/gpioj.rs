#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioj_moder: GPIOJ_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioj_otyper: GPIOJ_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioj_ospeedr: GPIOJ_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioj_pupdr: GPIOJ_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioj_idr: GPIOJ_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioj_odr: GPIOJ_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioj_bsrr: GPIOJ_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioj_lckr: GPIOJ_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioj_afrl: GPIOJ_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioj_afrh: GPIOJ_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioj_brr: GPIOJ_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioj_hwcfgr10: GPIOJ_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioj_hwcfgr9: GPIOJ_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioj_hwcfgr8: GPIOJ_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioj_hwcfgr7: GPIOJ_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioj_hwcfgr6: GPIOJ_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioj_hwcfgr5: GPIOJ_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioj_hwcfgr4: GPIOJ_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioj_hwcfgr3: GPIOJ_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioj_hwcfgr2: GPIOJ_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioj_hwcfgr1: GPIOJ_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioj_hwcfgr0: GPIOJ_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioj_verr: GPIOJ_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioj_ipidr: GPIOJ_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioj_sidr: GPIOJ_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_moder](gpioj_moder) module"]
pub type GPIOJ_MODER = crate::Reg<u32, _GPIOJ_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_MODER;
#[doc = "`read()` method returns [gpioj_moder::R](gpioj_moder::R) reader structure"]
impl crate::Readable for GPIOJ_MODER {}
#[doc = "`write(|w| ..)` method takes [gpioj_moder::W](gpioj_moder::W) writer structure"]
impl crate::Writable for GPIOJ_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpioj_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_otyper](gpioj_otyper) module"]
pub type GPIOJ_OTYPER = crate::Reg<u32, _GPIOJ_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_OTYPER;
#[doc = "`read()` method returns [gpioj_otyper::R](gpioj_otyper::R) reader structure"]
impl crate::Readable for GPIOJ_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpioj_otyper::W](gpioj_otyper::W) writer structure"]
impl crate::Writable for GPIOJ_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpioj_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_ospeedr](gpioj_ospeedr) module"]
pub type GPIOJ_OSPEEDR = crate::Reg<u32, _GPIOJ_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_OSPEEDR;
#[doc = "`read()` method returns [gpioj_ospeedr::R](gpioj_ospeedr::R) reader structure"]
impl crate::Readable for GPIOJ_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpioj_ospeedr::W](gpioj_ospeedr::W) writer structure"]
impl crate::Writable for GPIOJ_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpioj_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_pupdr](gpioj_pupdr) module"]
pub type GPIOJ_PUPDR = crate::Reg<u32, _GPIOJ_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_PUPDR;
#[doc = "`read()` method returns [gpioj_pupdr::R](gpioj_pupdr::R) reader structure"]
impl crate::Readable for GPIOJ_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpioj_pupdr::W](gpioj_pupdr::W) writer structure"]
impl crate::Writable for GPIOJ_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioj_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_idr](gpioj_idr) module"]
pub type GPIOJ_IDR = crate::Reg<u32, _GPIOJ_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_IDR;
#[doc = "`read()` method returns [gpioj_idr::R](gpioj_idr::R) reader structure"]
impl crate::Readable for GPIOJ_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpioj_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_odr](gpioj_odr) module"]
pub type GPIOJ_ODR = crate::Reg<u32, _GPIOJ_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_ODR;
#[doc = "`read()` method returns [gpioj_odr::R](gpioj_odr::R) reader structure"]
impl crate::Readable for GPIOJ_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioj_odr::W](gpioj_odr::W) writer structure"]
impl crate::Writable for GPIOJ_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpioj_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_bsrr](gpioj_bsrr) module"]
pub type GPIOJ_BSRR = crate::Reg<u32, _GPIOJ_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_BSRR;
#[doc = "`write(|w| ..)` method takes [gpioj_bsrr::W](gpioj_bsrr::W) writer structure"]
impl crate::Writable for GPIOJ_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpioj_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_lckr](gpioj_lckr) module"]
pub type GPIOJ_LCKR = crate::Reg<u32, _GPIOJ_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_LCKR;
#[doc = "`read()` method returns [gpioj_lckr::R](gpioj_lckr::R) reader structure"]
impl crate::Readable for GPIOJ_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpioj_lckr::W](gpioj_lckr::W) writer structure"]
impl crate::Writable for GPIOJ_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioj_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_afrl](gpioj_afrl) module"]
pub type GPIOJ_AFRL = crate::Reg<u32, _GPIOJ_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_AFRL;
#[doc = "`read()` method returns [gpioj_afrl::R](gpioj_afrl::R) reader structure"]
impl crate::Readable for GPIOJ_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpioj_afrl::W](gpioj_afrl::W) writer structure"]
impl crate::Writable for GPIOJ_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpioj_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_afrh](gpioj_afrh) module"]
pub type GPIOJ_AFRH = crate::Reg<u32, _GPIOJ_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_AFRH;
#[doc = "`read()` method returns [gpioj_afrh::R](gpioj_afrh::R) reader structure"]
impl crate::Readable for GPIOJ_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpioj_afrh::W](gpioj_afrh::W) writer structure"]
impl crate::Writable for GPIOJ_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpioj_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_brr](gpioj_brr) module"]
pub type GPIOJ_BRR = crate::Reg<u32, _GPIOJ_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_BRR;
#[doc = "`write(|w| ..)` method takes [gpioj_brr::W](gpioj_brr::W) writer structure"]
impl crate::Writable for GPIOJ_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpioj_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr10](gpioj_hwcfgr10) module"]
pub type GPIOJ_HWCFGR10 = crate::Reg<u32, _GPIOJ_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR10;
#[doc = "`read()` method returns [gpioj_hwcfgr10::R](gpioj_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioj_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr9](gpioj_hwcfgr9) module"]
pub type GPIOJ_HWCFGR9 = crate::Reg<u32, _GPIOJ_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR9;
#[doc = "`read()` method returns [gpioj_hwcfgr9::R](gpioj_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioj_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr8](gpioj_hwcfgr8) module"]
pub type GPIOJ_HWCFGR8 = crate::Reg<u32, _GPIOJ_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR8;
#[doc = "`read()` method returns [gpioj_hwcfgr8::R](gpioj_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioj_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr7](gpioj_hwcfgr7) module"]
pub type GPIOJ_HWCFGR7 = crate::Reg<u32, _GPIOJ_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR7;
#[doc = "`read()` method returns [gpioj_hwcfgr7::R](gpioj_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioj_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr6](gpioj_hwcfgr6) module"]
pub type GPIOJ_HWCFGR6 = crate::Reg<u32, _GPIOJ_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR6;
#[doc = "`read()` method returns [gpioj_hwcfgr6::R](gpioj_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioj_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr5](gpioj_hwcfgr5) module"]
pub type GPIOJ_HWCFGR5 = crate::Reg<u32, _GPIOJ_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR5;
#[doc = "`read()` method returns [gpioj_hwcfgr5::R](gpioj_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioj_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr4](gpioj_hwcfgr4) module"]
pub type GPIOJ_HWCFGR4 = crate::Reg<u32, _GPIOJ_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR4;
#[doc = "`read()` method returns [gpioj_hwcfgr4::R](gpioj_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioj_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr3](gpioj_hwcfgr3) module"]
pub type GPIOJ_HWCFGR3 = crate::Reg<u32, _GPIOJ_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR3;
#[doc = "`read()` method returns [gpioj_hwcfgr3::R](gpioj_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioj_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr2](gpioj_hwcfgr2) module"]
pub type GPIOJ_HWCFGR2 = crate::Reg<u32, _GPIOJ_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR2;
#[doc = "`read()` method returns [gpioj_hwcfgr2::R](gpioj_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioj_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr1](gpioj_hwcfgr1) module"]
pub type GPIOJ_HWCFGR1 = crate::Reg<u32, _GPIOJ_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR1;
#[doc = "`read()` method returns [gpioj_hwcfgr1::R](gpioj_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioj_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr0](gpioj_hwcfgr0) module"]
pub type GPIOJ_HWCFGR0 = crate::Reg<u32, _GPIOJ_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_HWCFGR0;
#[doc = "`read()` method returns [gpioj_hwcfgr0::R](gpioj_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioj_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_verr](gpioj_verr) module"]
pub type GPIOJ_VERR = crate::Reg<u32, _GPIOJ_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_VERR;
#[doc = "`read()` method returns [gpioj_verr::R](gpioj_verr::R) reader structure"]
impl crate::Readable for GPIOJ_VERR {}
#[doc = "GPIO version register"]
pub mod gpioj_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_ipidr](gpioj_ipidr) module"]
pub type GPIOJ_IPIDR = crate::Reg<u32, _GPIOJ_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_IPIDR;
#[doc = "`read()` method returns [gpioj_ipidr::R](gpioj_ipidr::R) reader structure"]
impl crate::Readable for GPIOJ_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpioj_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_sidr](gpioj_sidr) module"]
pub type GPIOJ_SIDR = crate::Reg<u32, _GPIOJ_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOJ_SIDR;
#[doc = "`read()` method returns [gpioj_sidr::R](gpioj_sidr::R) reader structure"]
impl crate::Readable for GPIOJ_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpioj_sidr;
