#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioe_moder: GPIOE_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioe_otyper: GPIOE_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioe_ospeedr: GPIOE_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioe_pupdr: GPIOE_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioe_idr: GPIOE_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioe_odr: GPIOE_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioe_bsrr: GPIOE_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioe_lckr: GPIOE_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioe_afrl: GPIOE_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioe_afrh: GPIOE_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioe_brr: GPIOE_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioe_hwcfgr10: GPIOE_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioe_hwcfgr9: GPIOE_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioe_hwcfgr8: GPIOE_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioe_hwcfgr7: GPIOE_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioe_hwcfgr6: GPIOE_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioe_hwcfgr5: GPIOE_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioe_hwcfgr4: GPIOE_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioe_hwcfgr3: GPIOE_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioe_hwcfgr2: GPIOE_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioe_hwcfgr1: GPIOE_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioe_hwcfgr0: GPIOE_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioe_verr: GPIOE_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioe_ipidr: GPIOE_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioe_sidr: GPIOE_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_moder](gpioe_moder) module"]
pub type GPIOE_MODER = crate::Reg<u32, _GPIOE_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_MODER;
#[doc = "`read()` method returns [gpioe_moder::R](gpioe_moder::R) reader structure"]
impl crate::Readable for GPIOE_MODER {}
#[doc = "`write(|w| ..)` method takes [gpioe_moder::W](gpioe_moder::W) writer structure"]
impl crate::Writable for GPIOE_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpioe_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_otyper](gpioe_otyper) module"]
pub type GPIOE_OTYPER = crate::Reg<u32, _GPIOE_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_OTYPER;
#[doc = "`read()` method returns [gpioe_otyper::R](gpioe_otyper::R) reader structure"]
impl crate::Readable for GPIOE_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpioe_otyper::W](gpioe_otyper::W) writer structure"]
impl crate::Writable for GPIOE_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpioe_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_ospeedr](gpioe_ospeedr) module"]
pub type GPIOE_OSPEEDR = crate::Reg<u32, _GPIOE_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_OSPEEDR;
#[doc = "`read()` method returns [gpioe_ospeedr::R](gpioe_ospeedr::R) reader structure"]
impl crate::Readable for GPIOE_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpioe_ospeedr::W](gpioe_ospeedr::W) writer structure"]
impl crate::Writable for GPIOE_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpioe_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_pupdr](gpioe_pupdr) module"]
pub type GPIOE_PUPDR = crate::Reg<u32, _GPIOE_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_PUPDR;
#[doc = "`read()` method returns [gpioe_pupdr::R](gpioe_pupdr::R) reader structure"]
impl crate::Readable for GPIOE_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpioe_pupdr::W](gpioe_pupdr::W) writer structure"]
impl crate::Writable for GPIOE_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioe_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_idr](gpioe_idr) module"]
pub type GPIOE_IDR = crate::Reg<u32, _GPIOE_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_IDR;
#[doc = "`read()` method returns [gpioe_idr::R](gpioe_idr::R) reader structure"]
impl crate::Readable for GPIOE_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpioe_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_odr](gpioe_odr) module"]
pub type GPIOE_ODR = crate::Reg<u32, _GPIOE_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_ODR;
#[doc = "`read()` method returns [gpioe_odr::R](gpioe_odr::R) reader structure"]
impl crate::Readable for GPIOE_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioe_odr::W](gpioe_odr::W) writer structure"]
impl crate::Writable for GPIOE_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpioe_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_bsrr](gpioe_bsrr) module"]
pub type GPIOE_BSRR = crate::Reg<u32, _GPIOE_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_BSRR;
#[doc = "`write(|w| ..)` method takes [gpioe_bsrr::W](gpioe_bsrr::W) writer structure"]
impl crate::Writable for GPIOE_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpioe_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_lckr](gpioe_lckr) module"]
pub type GPIOE_LCKR = crate::Reg<u32, _GPIOE_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_LCKR;
#[doc = "`read()` method returns [gpioe_lckr::R](gpioe_lckr::R) reader structure"]
impl crate::Readable for GPIOE_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpioe_lckr::W](gpioe_lckr::W) writer structure"]
impl crate::Writable for GPIOE_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioe_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_afrl](gpioe_afrl) module"]
pub type GPIOE_AFRL = crate::Reg<u32, _GPIOE_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_AFRL;
#[doc = "`read()` method returns [gpioe_afrl::R](gpioe_afrl::R) reader structure"]
impl crate::Readable for GPIOE_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpioe_afrl::W](gpioe_afrl::W) writer structure"]
impl crate::Writable for GPIOE_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpioe_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_afrh](gpioe_afrh) module"]
pub type GPIOE_AFRH = crate::Reg<u32, _GPIOE_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_AFRH;
#[doc = "`read()` method returns [gpioe_afrh::R](gpioe_afrh::R) reader structure"]
impl crate::Readable for GPIOE_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpioe_afrh::W](gpioe_afrh::W) writer structure"]
impl crate::Writable for GPIOE_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpioe_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_brr](gpioe_brr) module"]
pub type GPIOE_BRR = crate::Reg<u32, _GPIOE_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_BRR;
#[doc = "`write(|w| ..)` method takes [gpioe_brr::W](gpioe_brr::W) writer structure"]
impl crate::Writable for GPIOE_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpioe_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr10](gpioe_hwcfgr10) module"]
pub type GPIOE_HWCFGR10 = crate::Reg<u32, _GPIOE_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR10;
#[doc = "`read()` method returns [gpioe_hwcfgr10::R](gpioe_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioe_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr9](gpioe_hwcfgr9) module"]
pub type GPIOE_HWCFGR9 = crate::Reg<u32, _GPIOE_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR9;
#[doc = "`read()` method returns [gpioe_hwcfgr9::R](gpioe_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioe_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr8](gpioe_hwcfgr8) module"]
pub type GPIOE_HWCFGR8 = crate::Reg<u32, _GPIOE_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR8;
#[doc = "`read()` method returns [gpioe_hwcfgr8::R](gpioe_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioe_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr7](gpioe_hwcfgr7) module"]
pub type GPIOE_HWCFGR7 = crate::Reg<u32, _GPIOE_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR7;
#[doc = "`read()` method returns [gpioe_hwcfgr7::R](gpioe_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioe_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr6](gpioe_hwcfgr6) module"]
pub type GPIOE_HWCFGR6 = crate::Reg<u32, _GPIOE_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR6;
#[doc = "`read()` method returns [gpioe_hwcfgr6::R](gpioe_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioe_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr5](gpioe_hwcfgr5) module"]
pub type GPIOE_HWCFGR5 = crate::Reg<u32, _GPIOE_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR5;
#[doc = "`read()` method returns [gpioe_hwcfgr5::R](gpioe_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioe_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr4](gpioe_hwcfgr4) module"]
pub type GPIOE_HWCFGR4 = crate::Reg<u32, _GPIOE_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR4;
#[doc = "`read()` method returns [gpioe_hwcfgr4::R](gpioe_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioe_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr3](gpioe_hwcfgr3) module"]
pub type GPIOE_HWCFGR3 = crate::Reg<u32, _GPIOE_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR3;
#[doc = "`read()` method returns [gpioe_hwcfgr3::R](gpioe_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioe_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr2](gpioe_hwcfgr2) module"]
pub type GPIOE_HWCFGR2 = crate::Reg<u32, _GPIOE_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR2;
#[doc = "`read()` method returns [gpioe_hwcfgr2::R](gpioe_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioe_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr1](gpioe_hwcfgr1) module"]
pub type GPIOE_HWCFGR1 = crate::Reg<u32, _GPIOE_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR1;
#[doc = "`read()` method returns [gpioe_hwcfgr1::R](gpioe_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioe_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr0](gpioe_hwcfgr0) module"]
pub type GPIOE_HWCFGR0 = crate::Reg<u32, _GPIOE_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_HWCFGR0;
#[doc = "`read()` method returns [gpioe_hwcfgr0::R](gpioe_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioe_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_verr](gpioe_verr) module"]
pub type GPIOE_VERR = crate::Reg<u32, _GPIOE_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_VERR;
#[doc = "`read()` method returns [gpioe_verr::R](gpioe_verr::R) reader structure"]
impl crate::Readable for GPIOE_VERR {}
#[doc = "GPIO version register"]
pub mod gpioe_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_ipidr](gpioe_ipidr) module"]
pub type GPIOE_IPIDR = crate::Reg<u32, _GPIOE_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_IPIDR;
#[doc = "`read()` method returns [gpioe_ipidr::R](gpioe_ipidr::R) reader structure"]
impl crate::Readable for GPIOE_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpioe_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_sidr](gpioe_sidr) module"]
pub type GPIOE_SIDR = crate::Reg<u32, _GPIOE_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_SIDR;
#[doc = "`read()` method returns [gpioe_sidr::R](gpioe_sidr::R) reader structure"]
impl crate::Readable for GPIOE_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpioe_sidr;
