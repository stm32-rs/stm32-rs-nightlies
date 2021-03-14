#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioh_moder: GPIOH_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioh_otyper: GPIOH_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioh_ospeedr: GPIOH_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioh_pupdr: GPIOH_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioh_idr: GPIOH_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioh_odr: GPIOH_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioh_bsrr: GPIOH_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioh_lckr: GPIOH_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioh_afrl: GPIOH_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioh_afrh: GPIOH_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioh_brr: GPIOH_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioh_hwcfgr10: GPIOH_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioh_hwcfgr9: GPIOH_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioh_hwcfgr8: GPIOH_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioh_hwcfgr7: GPIOH_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioh_hwcfgr6: GPIOH_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioh_hwcfgr5: GPIOH_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioh_hwcfgr4: GPIOH_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioh_hwcfgr3: GPIOH_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioh_hwcfgr2: GPIOH_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioh_hwcfgr1: GPIOH_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioh_hwcfgr0: GPIOH_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioh_verr: GPIOH_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioh_ipidr: GPIOH_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioh_sidr: GPIOH_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_moder](gpioh_moder) module"]
pub type GPIOH_MODER = crate::Reg<u32, _GPIOH_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_MODER;
#[doc = "`read()` method returns [gpioh_moder::R](gpioh_moder::R) reader structure"]
impl crate::Readable for GPIOH_MODER {}
#[doc = "`write(|w| ..)` method takes [gpioh_moder::W](gpioh_moder::W) writer structure"]
impl crate::Writable for GPIOH_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpioh_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_otyper](gpioh_otyper) module"]
pub type GPIOH_OTYPER = crate::Reg<u32, _GPIOH_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_OTYPER;
#[doc = "`read()` method returns [gpioh_otyper::R](gpioh_otyper::R) reader structure"]
impl crate::Readable for GPIOH_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpioh_otyper::W](gpioh_otyper::W) writer structure"]
impl crate::Writable for GPIOH_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpioh_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_ospeedr](gpioh_ospeedr) module"]
pub type GPIOH_OSPEEDR = crate::Reg<u32, _GPIOH_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_OSPEEDR;
#[doc = "`read()` method returns [gpioh_ospeedr::R](gpioh_ospeedr::R) reader structure"]
impl crate::Readable for GPIOH_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpioh_ospeedr::W](gpioh_ospeedr::W) writer structure"]
impl crate::Writable for GPIOH_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpioh_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_pupdr](gpioh_pupdr) module"]
pub type GPIOH_PUPDR = crate::Reg<u32, _GPIOH_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_PUPDR;
#[doc = "`read()` method returns [gpioh_pupdr::R](gpioh_pupdr::R) reader structure"]
impl crate::Readable for GPIOH_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpioh_pupdr::W](gpioh_pupdr::W) writer structure"]
impl crate::Writable for GPIOH_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioh_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_idr](gpioh_idr) module"]
pub type GPIOH_IDR = crate::Reg<u32, _GPIOH_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_IDR;
#[doc = "`read()` method returns [gpioh_idr::R](gpioh_idr::R) reader structure"]
impl crate::Readable for GPIOH_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpioh_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_odr](gpioh_odr) module"]
pub type GPIOH_ODR = crate::Reg<u32, _GPIOH_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_ODR;
#[doc = "`read()` method returns [gpioh_odr::R](gpioh_odr::R) reader structure"]
impl crate::Readable for GPIOH_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioh_odr::W](gpioh_odr::W) writer structure"]
impl crate::Writable for GPIOH_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpioh_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_bsrr](gpioh_bsrr) module"]
pub type GPIOH_BSRR = crate::Reg<u32, _GPIOH_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_BSRR;
#[doc = "`write(|w| ..)` method takes [gpioh_bsrr::W](gpioh_bsrr::W) writer structure"]
impl crate::Writable for GPIOH_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpioh_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_lckr](gpioh_lckr) module"]
pub type GPIOH_LCKR = crate::Reg<u32, _GPIOH_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_LCKR;
#[doc = "`read()` method returns [gpioh_lckr::R](gpioh_lckr::R) reader structure"]
impl crate::Readable for GPIOH_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpioh_lckr::W](gpioh_lckr::W) writer structure"]
impl crate::Writable for GPIOH_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioh_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_afrl](gpioh_afrl) module"]
pub type GPIOH_AFRL = crate::Reg<u32, _GPIOH_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_AFRL;
#[doc = "`read()` method returns [gpioh_afrl::R](gpioh_afrl::R) reader structure"]
impl crate::Readable for GPIOH_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpioh_afrl::W](gpioh_afrl::W) writer structure"]
impl crate::Writable for GPIOH_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpioh_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_afrh](gpioh_afrh) module"]
pub type GPIOH_AFRH = crate::Reg<u32, _GPIOH_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_AFRH;
#[doc = "`read()` method returns [gpioh_afrh::R](gpioh_afrh::R) reader structure"]
impl crate::Readable for GPIOH_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpioh_afrh::W](gpioh_afrh::W) writer structure"]
impl crate::Writable for GPIOH_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpioh_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_brr](gpioh_brr) module"]
pub type GPIOH_BRR = crate::Reg<u32, _GPIOH_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_BRR;
#[doc = "`write(|w| ..)` method takes [gpioh_brr::W](gpioh_brr::W) writer structure"]
impl crate::Writable for GPIOH_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpioh_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr10](gpioh_hwcfgr10) module"]
pub type GPIOH_HWCFGR10 = crate::Reg<u32, _GPIOH_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR10;
#[doc = "`read()` method returns [gpioh_hwcfgr10::R](gpioh_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioh_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr9](gpioh_hwcfgr9) module"]
pub type GPIOH_HWCFGR9 = crate::Reg<u32, _GPIOH_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR9;
#[doc = "`read()` method returns [gpioh_hwcfgr9::R](gpioh_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioh_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr8](gpioh_hwcfgr8) module"]
pub type GPIOH_HWCFGR8 = crate::Reg<u32, _GPIOH_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR8;
#[doc = "`read()` method returns [gpioh_hwcfgr8::R](gpioh_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioh_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr7](gpioh_hwcfgr7) module"]
pub type GPIOH_HWCFGR7 = crate::Reg<u32, _GPIOH_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR7;
#[doc = "`read()` method returns [gpioh_hwcfgr7::R](gpioh_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioh_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr6](gpioh_hwcfgr6) module"]
pub type GPIOH_HWCFGR6 = crate::Reg<u32, _GPIOH_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR6;
#[doc = "`read()` method returns [gpioh_hwcfgr6::R](gpioh_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioh_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr5](gpioh_hwcfgr5) module"]
pub type GPIOH_HWCFGR5 = crate::Reg<u32, _GPIOH_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR5;
#[doc = "`read()` method returns [gpioh_hwcfgr5::R](gpioh_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioh_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr4](gpioh_hwcfgr4) module"]
pub type GPIOH_HWCFGR4 = crate::Reg<u32, _GPIOH_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR4;
#[doc = "`read()` method returns [gpioh_hwcfgr4::R](gpioh_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioh_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr3](gpioh_hwcfgr3) module"]
pub type GPIOH_HWCFGR3 = crate::Reg<u32, _GPIOH_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR3;
#[doc = "`read()` method returns [gpioh_hwcfgr3::R](gpioh_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioh_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr2](gpioh_hwcfgr2) module"]
pub type GPIOH_HWCFGR2 = crate::Reg<u32, _GPIOH_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR2;
#[doc = "`read()` method returns [gpioh_hwcfgr2::R](gpioh_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioh_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr1](gpioh_hwcfgr1) module"]
pub type GPIOH_HWCFGR1 = crate::Reg<u32, _GPIOH_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR1;
#[doc = "`read()` method returns [gpioh_hwcfgr1::R](gpioh_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioh_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr0](gpioh_hwcfgr0) module"]
pub type GPIOH_HWCFGR0 = crate::Reg<u32, _GPIOH_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_HWCFGR0;
#[doc = "`read()` method returns [gpioh_hwcfgr0::R](gpioh_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioh_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_verr](gpioh_verr) module"]
pub type GPIOH_VERR = crate::Reg<u32, _GPIOH_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_VERR;
#[doc = "`read()` method returns [gpioh_verr::R](gpioh_verr::R) reader structure"]
impl crate::Readable for GPIOH_VERR {}
#[doc = "GPIO version register"]
pub mod gpioh_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_ipidr](gpioh_ipidr) module"]
pub type GPIOH_IPIDR = crate::Reg<u32, _GPIOH_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_IPIDR;
#[doc = "`read()` method returns [gpioh_ipidr::R](gpioh_ipidr::R) reader structure"]
impl crate::Readable for GPIOH_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpioh_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_sidr](gpioh_sidr) module"]
pub type GPIOH_SIDR = crate::Reg<u32, _GPIOH_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOH_SIDR;
#[doc = "`read()` method returns [gpioh_sidr::R](gpioh_sidr::R) reader structure"]
impl crate::Readable for GPIOH_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpioh_sidr;
