#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiof_moder: GPIOF_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiof_otyper: GPIOF_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiof_ospeedr: GPIOF_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiof_pupdr: GPIOF_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiof_idr: GPIOF_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiof_odr: GPIOF_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiof_bsrr: GPIOF_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiof_lckr: GPIOF_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiof_afrl: GPIOF_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiof_afrh: GPIOF_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiof_brr: GPIOF_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiof_hwcfgr10: GPIOF_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiof_hwcfgr9: GPIOF_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiof_hwcfgr8: GPIOF_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiof_hwcfgr7: GPIOF_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiof_hwcfgr6: GPIOF_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiof_hwcfgr5: GPIOF_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiof_hwcfgr4: GPIOF_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiof_hwcfgr3: GPIOF_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiof_hwcfgr2: GPIOF_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiof_hwcfgr1: GPIOF_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiof_hwcfgr0: GPIOF_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiof_verr: GPIOF_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiof_ipidr: GPIOF_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiof_sidr: GPIOF_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_moder](gpiof_moder) module"]
pub type GPIOF_MODER = crate::Reg<u32, _GPIOF_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_MODER;
#[doc = "`read()` method returns [gpiof_moder::R](gpiof_moder::R) reader structure"]
impl crate::Readable for GPIOF_MODER {}
#[doc = "`write(|w| ..)` method takes [gpiof_moder::W](gpiof_moder::W) writer structure"]
impl crate::Writable for GPIOF_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpiof_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_otyper](gpiof_otyper) module"]
pub type GPIOF_OTYPER = crate::Reg<u32, _GPIOF_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_OTYPER;
#[doc = "`read()` method returns [gpiof_otyper::R](gpiof_otyper::R) reader structure"]
impl crate::Readable for GPIOF_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpiof_otyper::W](gpiof_otyper::W) writer structure"]
impl crate::Writable for GPIOF_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpiof_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_ospeedr](gpiof_ospeedr) module"]
pub type GPIOF_OSPEEDR = crate::Reg<u32, _GPIOF_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_OSPEEDR;
#[doc = "`read()` method returns [gpiof_ospeedr::R](gpiof_ospeedr::R) reader structure"]
impl crate::Readable for GPIOF_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpiof_ospeedr::W](gpiof_ospeedr::W) writer structure"]
impl crate::Writable for GPIOF_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpiof_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_pupdr](gpiof_pupdr) module"]
pub type GPIOF_PUPDR = crate::Reg<u32, _GPIOF_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_PUPDR;
#[doc = "`read()` method returns [gpiof_pupdr::R](gpiof_pupdr::R) reader structure"]
impl crate::Readable for GPIOF_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpiof_pupdr::W](gpiof_pupdr::W) writer structure"]
impl crate::Writable for GPIOF_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiof_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_idr](gpiof_idr) module"]
pub type GPIOF_IDR = crate::Reg<u32, _GPIOF_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_IDR;
#[doc = "`read()` method returns [gpiof_idr::R](gpiof_idr::R) reader structure"]
impl crate::Readable for GPIOF_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpiof_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_odr](gpiof_odr) module"]
pub type GPIOF_ODR = crate::Reg<u32, _GPIOF_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_ODR;
#[doc = "`read()` method returns [gpiof_odr::R](gpiof_odr::R) reader structure"]
impl crate::Readable for GPIOF_ODR {}
#[doc = "`write(|w| ..)` method takes [gpiof_odr::W](gpiof_odr::W) writer structure"]
impl crate::Writable for GPIOF_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpiof_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_bsrr](gpiof_bsrr) module"]
pub type GPIOF_BSRR = crate::Reg<u32, _GPIOF_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_BSRR;
#[doc = "`write(|w| ..)` method takes [gpiof_bsrr::W](gpiof_bsrr::W) writer structure"]
impl crate::Writable for GPIOF_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpiof_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_lckr](gpiof_lckr) module"]
pub type GPIOF_LCKR = crate::Reg<u32, _GPIOF_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_LCKR;
#[doc = "`read()` method returns [gpiof_lckr::R](gpiof_lckr::R) reader structure"]
impl crate::Readable for GPIOF_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpiof_lckr::W](gpiof_lckr::W) writer structure"]
impl crate::Writable for GPIOF_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiof_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_afrl](gpiof_afrl) module"]
pub type GPIOF_AFRL = crate::Reg<u32, _GPIOF_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_AFRL;
#[doc = "`read()` method returns [gpiof_afrl::R](gpiof_afrl::R) reader structure"]
impl crate::Readable for GPIOF_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpiof_afrl::W](gpiof_afrl::W) writer structure"]
impl crate::Writable for GPIOF_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpiof_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_afrh](gpiof_afrh) module"]
pub type GPIOF_AFRH = crate::Reg<u32, _GPIOF_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_AFRH;
#[doc = "`read()` method returns [gpiof_afrh::R](gpiof_afrh::R) reader structure"]
impl crate::Readable for GPIOF_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpiof_afrh::W](gpiof_afrh::W) writer structure"]
impl crate::Writable for GPIOF_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpiof_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_brr](gpiof_brr) module"]
pub type GPIOF_BRR = crate::Reg<u32, _GPIOF_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_BRR;
#[doc = "`write(|w| ..)` method takes [gpiof_brr::W](gpiof_brr::W) writer structure"]
impl crate::Writable for GPIOF_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpiof_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr10](gpiof_hwcfgr10) module"]
pub type GPIOF_HWCFGR10 = crate::Reg<u32, _GPIOF_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR10;
#[doc = "`read()` method returns [gpiof_hwcfgr10::R](gpiof_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiof_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr9](gpiof_hwcfgr9) module"]
pub type GPIOF_HWCFGR9 = crate::Reg<u32, _GPIOF_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR9;
#[doc = "`read()` method returns [gpiof_hwcfgr9::R](gpiof_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiof_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr8](gpiof_hwcfgr8) module"]
pub type GPIOF_HWCFGR8 = crate::Reg<u32, _GPIOF_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR8;
#[doc = "`read()` method returns [gpiof_hwcfgr8::R](gpiof_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiof_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr7](gpiof_hwcfgr7) module"]
pub type GPIOF_HWCFGR7 = crate::Reg<u32, _GPIOF_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR7;
#[doc = "`read()` method returns [gpiof_hwcfgr7::R](gpiof_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiof_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr6](gpiof_hwcfgr6) module"]
pub type GPIOF_HWCFGR6 = crate::Reg<u32, _GPIOF_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR6;
#[doc = "`read()` method returns [gpiof_hwcfgr6::R](gpiof_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiof_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr5](gpiof_hwcfgr5) module"]
pub type GPIOF_HWCFGR5 = crate::Reg<u32, _GPIOF_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR5;
#[doc = "`read()` method returns [gpiof_hwcfgr5::R](gpiof_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiof_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr4](gpiof_hwcfgr4) module"]
pub type GPIOF_HWCFGR4 = crate::Reg<u32, _GPIOF_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR4;
#[doc = "`read()` method returns [gpiof_hwcfgr4::R](gpiof_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiof_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr3](gpiof_hwcfgr3) module"]
pub type GPIOF_HWCFGR3 = crate::Reg<u32, _GPIOF_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR3;
#[doc = "`read()` method returns [gpiof_hwcfgr3::R](gpiof_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiof_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr2](gpiof_hwcfgr2) module"]
pub type GPIOF_HWCFGR2 = crate::Reg<u32, _GPIOF_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR2;
#[doc = "`read()` method returns [gpiof_hwcfgr2::R](gpiof_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiof_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr1](gpiof_hwcfgr1) module"]
pub type GPIOF_HWCFGR1 = crate::Reg<u32, _GPIOF_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR1;
#[doc = "`read()` method returns [gpiof_hwcfgr1::R](gpiof_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiof_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_hwcfgr0](gpiof_hwcfgr0) module"]
pub type GPIOF_HWCFGR0 = crate::Reg<u32, _GPIOF_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_HWCFGR0;
#[doc = "`read()` method returns [gpiof_hwcfgr0::R](gpiof_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOF_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiof_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_verr](gpiof_verr) module"]
pub type GPIOF_VERR = crate::Reg<u32, _GPIOF_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_VERR;
#[doc = "`read()` method returns [gpiof_verr::R](gpiof_verr::R) reader structure"]
impl crate::Readable for GPIOF_VERR {}
#[doc = "GPIO version register"]
pub mod gpiof_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_ipidr](gpiof_ipidr) module"]
pub type GPIOF_IPIDR = crate::Reg<u32, _GPIOF_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_IPIDR;
#[doc = "`read()` method returns [gpiof_ipidr::R](gpiof_ipidr::R) reader structure"]
impl crate::Readable for GPIOF_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpiof_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiof_sidr](gpiof_sidr) module"]
pub type GPIOF_SIDR = crate::Reg<u32, _GPIOF_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOF_SIDR;
#[doc = "`read()` method returns [gpiof_sidr::R](gpiof_sidr::R) reader structure"]
impl crate::Readable for GPIOF_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpiof_sidr;
