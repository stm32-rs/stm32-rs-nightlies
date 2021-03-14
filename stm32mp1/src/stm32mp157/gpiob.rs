#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiob_moder: GPIOB_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiob_otyper: GPIOB_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiob_ospeedr: GPIOB_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiob_pupdr: GPIOB_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiob_idr: GPIOB_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiob_odr: GPIOB_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiob_bsrr: GPIOB_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiob_lckr: GPIOB_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiob_afrl: GPIOB_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiob_afrh: GPIOB_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiob_brr: GPIOB_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiob_hwcfgr10: GPIOB_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiob_hwcfgr9: GPIOB_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiob_hwcfgr8: GPIOB_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiob_hwcfgr7: GPIOB_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiob_hwcfgr6: GPIOB_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiob_hwcfgr5: GPIOB_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiob_hwcfgr4: GPIOB_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiob_hwcfgr3: GPIOB_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiob_hwcfgr2: GPIOB_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiob_hwcfgr1: GPIOB_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiob_hwcfgr0: GPIOB_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiob_verr: GPIOB_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiob_ipidr: GPIOB_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiob_sidr: GPIOB_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_moder](gpiob_moder) module"]
pub type GPIOB_MODER = crate::Reg<u32, _GPIOB_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_MODER;
#[doc = "`read()` method returns [gpiob_moder::R](gpiob_moder::R) reader structure"]
impl crate::Readable for GPIOB_MODER {}
#[doc = "`write(|w| ..)` method takes [gpiob_moder::W](gpiob_moder::W) writer structure"]
impl crate::Writable for GPIOB_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpiob_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_otyper](gpiob_otyper) module"]
pub type GPIOB_OTYPER = crate::Reg<u32, _GPIOB_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_OTYPER;
#[doc = "`read()` method returns [gpiob_otyper::R](gpiob_otyper::R) reader structure"]
impl crate::Readable for GPIOB_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpiob_otyper::W](gpiob_otyper::W) writer structure"]
impl crate::Writable for GPIOB_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpiob_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_ospeedr](gpiob_ospeedr) module"]
pub type GPIOB_OSPEEDR = crate::Reg<u32, _GPIOB_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_OSPEEDR;
#[doc = "`read()` method returns [gpiob_ospeedr::R](gpiob_ospeedr::R) reader structure"]
impl crate::Readable for GPIOB_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpiob_ospeedr::W](gpiob_ospeedr::W) writer structure"]
impl crate::Writable for GPIOB_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpiob_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_pupdr](gpiob_pupdr) module"]
pub type GPIOB_PUPDR = crate::Reg<u32, _GPIOB_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_PUPDR;
#[doc = "`read()` method returns [gpiob_pupdr::R](gpiob_pupdr::R) reader structure"]
impl crate::Readable for GPIOB_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpiob_pupdr::W](gpiob_pupdr::W) writer structure"]
impl crate::Writable for GPIOB_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiob_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_idr](gpiob_idr) module"]
pub type GPIOB_IDR = crate::Reg<u32, _GPIOB_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_IDR;
#[doc = "`read()` method returns [gpiob_idr::R](gpiob_idr::R) reader structure"]
impl crate::Readable for GPIOB_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpiob_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_odr](gpiob_odr) module"]
pub type GPIOB_ODR = crate::Reg<u32, _GPIOB_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_ODR;
#[doc = "`read()` method returns [gpiob_odr::R](gpiob_odr::R) reader structure"]
impl crate::Readable for GPIOB_ODR {}
#[doc = "`write(|w| ..)` method takes [gpiob_odr::W](gpiob_odr::W) writer structure"]
impl crate::Writable for GPIOB_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpiob_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_bsrr](gpiob_bsrr) module"]
pub type GPIOB_BSRR = crate::Reg<u32, _GPIOB_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_BSRR;
#[doc = "`write(|w| ..)` method takes [gpiob_bsrr::W](gpiob_bsrr::W) writer structure"]
impl crate::Writable for GPIOB_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpiob_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_lckr](gpiob_lckr) module"]
pub type GPIOB_LCKR = crate::Reg<u32, _GPIOB_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_LCKR;
#[doc = "`read()` method returns [gpiob_lckr::R](gpiob_lckr::R) reader structure"]
impl crate::Readable for GPIOB_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpiob_lckr::W](gpiob_lckr::W) writer structure"]
impl crate::Writable for GPIOB_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiob_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_afrl](gpiob_afrl) module"]
pub type GPIOB_AFRL = crate::Reg<u32, _GPIOB_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_AFRL;
#[doc = "`read()` method returns [gpiob_afrl::R](gpiob_afrl::R) reader structure"]
impl crate::Readable for GPIOB_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpiob_afrl::W](gpiob_afrl::W) writer structure"]
impl crate::Writable for GPIOB_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpiob_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_afrh](gpiob_afrh) module"]
pub type GPIOB_AFRH = crate::Reg<u32, _GPIOB_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_AFRH;
#[doc = "`read()` method returns [gpiob_afrh::R](gpiob_afrh::R) reader structure"]
impl crate::Readable for GPIOB_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpiob_afrh::W](gpiob_afrh::W) writer structure"]
impl crate::Writable for GPIOB_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpiob_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_brr](gpiob_brr) module"]
pub type GPIOB_BRR = crate::Reg<u32, _GPIOB_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_BRR;
#[doc = "`write(|w| ..)` method takes [gpiob_brr::W](gpiob_brr::W) writer structure"]
impl crate::Writable for GPIOB_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpiob_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr10](gpiob_hwcfgr10) module"]
pub type GPIOB_HWCFGR10 = crate::Reg<u32, _GPIOB_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR10;
#[doc = "`read()` method returns [gpiob_hwcfgr10::R](gpiob_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiob_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr9](gpiob_hwcfgr9) module"]
pub type GPIOB_HWCFGR9 = crate::Reg<u32, _GPIOB_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR9;
#[doc = "`read()` method returns [gpiob_hwcfgr9::R](gpiob_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiob_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr8](gpiob_hwcfgr8) module"]
pub type GPIOB_HWCFGR8 = crate::Reg<u32, _GPIOB_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR8;
#[doc = "`read()` method returns [gpiob_hwcfgr8::R](gpiob_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiob_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr7](gpiob_hwcfgr7) module"]
pub type GPIOB_HWCFGR7 = crate::Reg<u32, _GPIOB_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR7;
#[doc = "`read()` method returns [gpiob_hwcfgr7::R](gpiob_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiob_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr6](gpiob_hwcfgr6) module"]
pub type GPIOB_HWCFGR6 = crate::Reg<u32, _GPIOB_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR6;
#[doc = "`read()` method returns [gpiob_hwcfgr6::R](gpiob_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiob_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr5](gpiob_hwcfgr5) module"]
pub type GPIOB_HWCFGR5 = crate::Reg<u32, _GPIOB_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR5;
#[doc = "`read()` method returns [gpiob_hwcfgr5::R](gpiob_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiob_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr4](gpiob_hwcfgr4) module"]
pub type GPIOB_HWCFGR4 = crate::Reg<u32, _GPIOB_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR4;
#[doc = "`read()` method returns [gpiob_hwcfgr4::R](gpiob_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiob_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr3](gpiob_hwcfgr3) module"]
pub type GPIOB_HWCFGR3 = crate::Reg<u32, _GPIOB_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR3;
#[doc = "`read()` method returns [gpiob_hwcfgr3::R](gpiob_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiob_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr2](gpiob_hwcfgr2) module"]
pub type GPIOB_HWCFGR2 = crate::Reg<u32, _GPIOB_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR2;
#[doc = "`read()` method returns [gpiob_hwcfgr2::R](gpiob_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiob_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr1](gpiob_hwcfgr1) module"]
pub type GPIOB_HWCFGR1 = crate::Reg<u32, _GPIOB_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR1;
#[doc = "`read()` method returns [gpiob_hwcfgr1::R](gpiob_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiob_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr0](gpiob_hwcfgr0) module"]
pub type GPIOB_HWCFGR0 = crate::Reg<u32, _GPIOB_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_HWCFGR0;
#[doc = "`read()` method returns [gpiob_hwcfgr0::R](gpiob_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiob_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_verr](gpiob_verr) module"]
pub type GPIOB_VERR = crate::Reg<u32, _GPIOB_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_VERR;
#[doc = "`read()` method returns [gpiob_verr::R](gpiob_verr::R) reader structure"]
impl crate::Readable for GPIOB_VERR {}
#[doc = "GPIO version register"]
pub mod gpiob_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_ipidr](gpiob_ipidr) module"]
pub type GPIOB_IPIDR = crate::Reg<u32, _GPIOB_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_IPIDR;
#[doc = "`read()` method returns [gpiob_ipidr::R](gpiob_ipidr::R) reader structure"]
impl crate::Readable for GPIOB_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpiob_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_sidr](gpiob_sidr) module"]
pub type GPIOB_SIDR = crate::Reg<u32, _GPIOB_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_SIDR;
#[doc = "`read()` method returns [gpiob_sidr::R](gpiob_sidr::R) reader structure"]
impl crate::Readable for GPIOB_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpiob_sidr;
