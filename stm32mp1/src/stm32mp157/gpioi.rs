#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioi_moder: GPIOI_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioi_otyper: GPIOI_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioi_ospeedr: GPIOI_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioi_pupdr: GPIOI_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioi_idr: GPIOI_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioi_odr: GPIOI_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioi_bsrr: GPIOI_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioi_lckr: GPIOI_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioi_afrl: GPIOI_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioi_afrh: GPIOI_AFRH,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioi_brr: GPIOI_BRR,
    _reserved11: [u8; 924usize],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioi_hwcfgr10: GPIOI_HWCFGR10,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioi_hwcfgr9: GPIOI_HWCFGR9,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioi_hwcfgr8: GPIOI_HWCFGR8,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioi_hwcfgr7: GPIOI_HWCFGR7,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioi_hwcfgr6: GPIOI_HWCFGR6,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioi_hwcfgr5: GPIOI_HWCFGR5,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioi_hwcfgr4: GPIOI_HWCFGR4,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioi_hwcfgr3: GPIOI_HWCFGR3,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioi_hwcfgr2: GPIOI_HWCFGR2,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioi_hwcfgr1: GPIOI_HWCFGR1,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioi_hwcfgr0: GPIOI_HWCFGR0,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioi_verr: GPIOI_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioi_ipidr: GPIOI_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioi_sidr: GPIOI_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_moder](gpioi_moder) module"]
pub type GPIOI_MODER = crate::Reg<u32, _GPIOI_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_MODER;
#[doc = "`read()` method returns [gpioi_moder::R](gpioi_moder::R) reader structure"]
impl crate::Readable for GPIOI_MODER {}
#[doc = "`write(|w| ..)` method takes [gpioi_moder::W](gpioi_moder::W) writer structure"]
impl crate::Writable for GPIOI_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpioi_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_otyper](gpioi_otyper) module"]
pub type GPIOI_OTYPER = crate::Reg<u32, _GPIOI_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_OTYPER;
#[doc = "`read()` method returns [gpioi_otyper::R](gpioi_otyper::R) reader structure"]
impl crate::Readable for GPIOI_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpioi_otyper::W](gpioi_otyper::W) writer structure"]
impl crate::Writable for GPIOI_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpioi_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_ospeedr](gpioi_ospeedr) module"]
pub type GPIOI_OSPEEDR = crate::Reg<u32, _GPIOI_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_OSPEEDR;
#[doc = "`read()` method returns [gpioi_ospeedr::R](gpioi_ospeedr::R) reader structure"]
impl crate::Readable for GPIOI_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpioi_ospeedr::W](gpioi_ospeedr::W) writer structure"]
impl crate::Writable for GPIOI_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpioi_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_pupdr](gpioi_pupdr) module"]
pub type GPIOI_PUPDR = crate::Reg<u32, _GPIOI_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_PUPDR;
#[doc = "`read()` method returns [gpioi_pupdr::R](gpioi_pupdr::R) reader structure"]
impl crate::Readable for GPIOI_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpioi_pupdr::W](gpioi_pupdr::W) writer structure"]
impl crate::Writable for GPIOI_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioi_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_idr](gpioi_idr) module"]
pub type GPIOI_IDR = crate::Reg<u32, _GPIOI_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_IDR;
#[doc = "`read()` method returns [gpioi_idr::R](gpioi_idr::R) reader structure"]
impl crate::Readable for GPIOI_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpioi_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_odr](gpioi_odr) module"]
pub type GPIOI_ODR = crate::Reg<u32, _GPIOI_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_ODR;
#[doc = "`read()` method returns [gpioi_odr::R](gpioi_odr::R) reader structure"]
impl crate::Readable for GPIOI_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioi_odr::W](gpioi_odr::W) writer structure"]
impl crate::Writable for GPIOI_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpioi_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_bsrr](gpioi_bsrr) module"]
pub type GPIOI_BSRR = crate::Reg<u32, _GPIOI_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_BSRR;
#[doc = "`write(|w| ..)` method takes [gpioi_bsrr::W](gpioi_bsrr::W) writer structure"]
impl crate::Writable for GPIOI_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpioi_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_lckr](gpioi_lckr) module"]
pub type GPIOI_LCKR = crate::Reg<u32, _GPIOI_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_LCKR;
#[doc = "`read()` method returns [gpioi_lckr::R](gpioi_lckr::R) reader structure"]
impl crate::Readable for GPIOI_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpioi_lckr::W](gpioi_lckr::W) writer structure"]
impl crate::Writable for GPIOI_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioi_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_afrl](gpioi_afrl) module"]
pub type GPIOI_AFRL = crate::Reg<u32, _GPIOI_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_AFRL;
#[doc = "`read()` method returns [gpioi_afrl::R](gpioi_afrl::R) reader structure"]
impl crate::Readable for GPIOI_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpioi_afrl::W](gpioi_afrl::W) writer structure"]
impl crate::Writable for GPIOI_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpioi_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_afrh](gpioi_afrh) module"]
pub type GPIOI_AFRH = crate::Reg<u32, _GPIOI_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_AFRH;
#[doc = "`read()` method returns [gpioi_afrh::R](gpioi_afrh::R) reader structure"]
impl crate::Readable for GPIOI_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpioi_afrh::W](gpioi_afrh::W) writer structure"]
impl crate::Writable for GPIOI_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpioi_afrh;
#[doc = "GPIO port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_brr](gpioi_brr) module"]
pub type GPIOI_BRR = crate::Reg<u32, _GPIOI_BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_BRR;
#[doc = "`write(|w| ..)` method takes [gpioi_brr::W](gpioi_brr::W) writer structure"]
impl crate::Writable for GPIOI_BRR {}
#[doc = "GPIO port bit reset register"]
pub mod gpioi_brr;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr10](gpioi_hwcfgr10) module"]
pub type GPIOI_HWCFGR10 = crate::Reg<u32, _GPIOI_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR10;
#[doc = "`read()` method returns [gpioi_hwcfgr10::R](gpioi_hwcfgr10::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR10 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioi_hwcfgr10;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr9](gpioi_hwcfgr9) module"]
pub type GPIOI_HWCFGR9 = crate::Reg<u32, _GPIOI_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR9;
#[doc = "`read()` method returns [gpioi_hwcfgr9::R](gpioi_hwcfgr9::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR9 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioi_hwcfgr9;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr8](gpioi_hwcfgr8) module"]
pub type GPIOI_HWCFGR8 = crate::Reg<u32, _GPIOI_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR8;
#[doc = "`read()` method returns [gpioi_hwcfgr8::R](gpioi_hwcfgr8::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR8 {}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioi_hwcfgr8;
#[doc = "GPIO hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr7](gpioi_hwcfgr7) module"]
pub type GPIOI_HWCFGR7 = crate::Reg<u32, _GPIOI_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR7;
#[doc = "`read()` method returns [gpioi_hwcfgr7::R](gpioi_hwcfgr7::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR7 {}
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioi_hwcfgr7;
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr6](gpioi_hwcfgr6) module"]
pub type GPIOI_HWCFGR6 = crate::Reg<u32, _GPIOI_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR6;
#[doc = "`read()` method returns [gpioi_hwcfgr6::R](gpioi_hwcfgr6::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR6 {}
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioi_hwcfgr6;
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr5](gpioi_hwcfgr5) module"]
pub type GPIOI_HWCFGR5 = crate::Reg<u32, _GPIOI_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR5;
#[doc = "`read()` method returns [gpioi_hwcfgr5::R](gpioi_hwcfgr5::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR5 {}
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioi_hwcfgr5;
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr4](gpioi_hwcfgr4) module"]
pub type GPIOI_HWCFGR4 = crate::Reg<u32, _GPIOI_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR4;
#[doc = "`read()` method returns [gpioi_hwcfgr4::R](gpioi_hwcfgr4::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR4 {}
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioi_hwcfgr4;
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr3](gpioi_hwcfgr3) module"]
pub type GPIOI_HWCFGR3 = crate::Reg<u32, _GPIOI_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR3;
#[doc = "`read()` method returns [gpioi_hwcfgr3::R](gpioi_hwcfgr3::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR3 {}
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioi_hwcfgr3;
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr2](gpioi_hwcfgr2) module"]
pub type GPIOI_HWCFGR2 = crate::Reg<u32, _GPIOI_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR2;
#[doc = "`read()` method returns [gpioi_hwcfgr2::R](gpioi_hwcfgr2::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR2 {}
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioi_hwcfgr2;
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr1](gpioi_hwcfgr1) module"]
pub type GPIOI_HWCFGR1 = crate::Reg<u32, _GPIOI_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR1;
#[doc = "`read()` method returns [gpioi_hwcfgr1::R](gpioi_hwcfgr1::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR1 {}
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioi_hwcfgr1;
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr0](gpioi_hwcfgr0) module"]
pub type GPIOI_HWCFGR0 = crate::Reg<u32, _GPIOI_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_HWCFGR0;
#[doc = "`read()` method returns [gpioi_hwcfgr0::R](gpioi_hwcfgr0::R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR0 {}
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioi_hwcfgr0;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_verr](gpioi_verr) module"]
pub type GPIOI_VERR = crate::Reg<u32, _GPIOI_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_VERR;
#[doc = "`read()` method returns [gpioi_verr::R](gpioi_verr::R) reader structure"]
impl crate::Readable for GPIOI_VERR {}
#[doc = "GPIO version register"]
pub mod gpioi_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_ipidr](gpioi_ipidr) module"]
pub type GPIOI_IPIDR = crate::Reg<u32, _GPIOI_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_IPIDR;
#[doc = "`read()` method returns [gpioi_ipidr::R](gpioi_ipidr::R) reader structure"]
impl crate::Readable for GPIOI_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpioi_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_sidr](gpioi_sidr) module"]
pub type GPIOI_SIDR = crate::Reg<u32, _GPIOI_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOI_SIDR;
#[doc = "`read()` method returns [gpioi_sidr::R](gpioi_sidr::R) reader structure"]
impl crate::Readable for GPIOI_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpioi_sidr;
