#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck."]
    pub i2c_cr1: I2C_CR1,
    #[doc = "0x04 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    pub i2c_cr2: I2C_CR2,
    #[doc = "0x08 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    pub i2c_oar1: I2C_OAR1,
    #[doc = "0x0c - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    pub i2c_oar2: I2C_OAR2,
    #[doc = "0x10 - Access: No wait states"]
    pub i2c_timingr: I2C_TIMINGR,
    #[doc = "0x14 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    pub i2c_timeoutr: I2C_TIMEOUTR,
    #[doc = "0x18 - Access: No wait states"]
    pub i2c_isr: I2C_ISR,
    #[doc = "0x1c - Access: No wait states"]
    pub i2c_icr: I2C_ICR,
    #[doc = "0x20 - Access: No wait states"]
    pub i2c_pecr: I2C_PECR,
    #[doc = "0x24 - Access: No wait states"]
    pub i2c_rxdr: I2C_RXDR,
    #[doc = "0x28 - Access: No wait states"]
    pub i2c_txdr: I2C_TXDR,
    _reserved11: [u8; 964usize],
    #[doc = "0x3f0 - I2C hardware configuration register"]
    pub i2c_hwcfgr: I2C_HWCFGR,
    #[doc = "0x3f4 - I2C version register"]
    pub i2c_verr: I2C_VERR,
    #[doc = "0x3f8 - I2C identification register"]
    pub i2c_ipidr: I2C_IPIDR,
    #[doc = "0x3fc - I2C size identification register"]
    pub i2c_sidr: I2C_SIDR,
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cr1](i2c_cr1) module"]
pub type I2C_CR1 = crate::Reg<u32, _I2C_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CR1;
#[doc = "`read()` method returns [i2c_cr1::R](i2c_cr1::R) reader structure"]
impl crate::Readable for I2C_CR1 {}
#[doc = "`write(|w| ..)` method takes [i2c_cr1::W](i2c_cr1::W) writer structure"]
impl crate::Writable for I2C_CR1 {}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck."]
pub mod i2c_cr1;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cr2](i2c_cr2) module"]
pub type I2C_CR2 = crate::Reg<u32, _I2C_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CR2;
#[doc = "`read()` method returns [i2c_cr2::R](i2c_cr2::R) reader structure"]
impl crate::Readable for I2C_CR2 {}
#[doc = "`write(|w| ..)` method takes [i2c_cr2::W](i2c_cr2::W) writer structure"]
impl crate::Writable for I2C_CR2 {}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_cr2;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_oar1](i2c_oar1) module"]
pub type I2C_OAR1 = crate::Reg<u32, _I2C_OAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_OAR1;
#[doc = "`read()` method returns [i2c_oar1::R](i2c_oar1::R) reader structure"]
impl crate::Readable for I2C_OAR1 {}
#[doc = "`write(|w| ..)` method takes [i2c_oar1::W](i2c_oar1::W) writer structure"]
impl crate::Writable for I2C_OAR1 {}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_oar1;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_oar2](i2c_oar2) module"]
pub type I2C_OAR2 = crate::Reg<u32, _I2C_OAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_OAR2;
#[doc = "`read()` method returns [i2c_oar2::R](i2c_oar2::R) reader structure"]
impl crate::Readable for I2C_OAR2 {}
#[doc = "`write(|w| ..)` method takes [i2c_oar2::W](i2c_oar2::W) writer structure"]
impl crate::Writable for I2C_OAR2 {}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_oar2;
#[doc = "Access: No wait states\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_timingr](i2c_timingr) module"]
pub type I2C_TIMINGR = crate::Reg<u32, _I2C_TIMINGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TIMINGR;
#[doc = "`read()` method returns [i2c_timingr::R](i2c_timingr::R) reader structure"]
impl crate::Readable for I2C_TIMINGR {}
#[doc = "`write(|w| ..)` method takes [i2c_timingr::W](i2c_timingr::W) writer structure"]
impl crate::Writable for I2C_TIMINGR {}
#[doc = "Access: No wait states"]
pub mod i2c_timingr;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_timeoutr](i2c_timeoutr) module"]
pub type I2C_TIMEOUTR = crate::Reg<u32, _I2C_TIMEOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TIMEOUTR;
#[doc = "`read()` method returns [i2c_timeoutr::R](i2c_timeoutr::R) reader structure"]
impl crate::Readable for I2C_TIMEOUTR {}
#[doc = "`write(|w| ..)` method takes [i2c_timeoutr::W](i2c_timeoutr::W) writer structure"]
impl crate::Writable for I2C_TIMEOUTR {}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_timeoutr;
#[doc = "Access: No wait states\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_isr](i2c_isr) module"]
pub type I2C_ISR = crate::Reg<u32, _I2C_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_ISR;
#[doc = "`read()` method returns [i2c_isr::R](i2c_isr::R) reader structure"]
impl crate::Readable for I2C_ISR {}
#[doc = "`write(|w| ..)` method takes [i2c_isr::W](i2c_isr::W) writer structure"]
impl crate::Writable for I2C_ISR {}
#[doc = "Access: No wait states"]
pub mod i2c_isr;
#[doc = "Access: No wait states\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_icr](i2c_icr) module"]
pub type I2C_ICR = crate::Reg<u32, _I2C_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_ICR;
#[doc = "`write(|w| ..)` method takes [i2c_icr::W](i2c_icr::W) writer structure"]
impl crate::Writable for I2C_ICR {}
#[doc = "Access: No wait states"]
pub mod i2c_icr;
#[doc = "Access: No wait states\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_pecr](i2c_pecr) module"]
pub type I2C_PECR = crate::Reg<u32, _I2C_PECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_PECR;
#[doc = "`read()` method returns [i2c_pecr::R](i2c_pecr::R) reader structure"]
impl crate::Readable for I2C_PECR {}
#[doc = "Access: No wait states"]
pub mod i2c_pecr;
#[doc = "Access: No wait states\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_rxdr](i2c_rxdr) module"]
pub type I2C_RXDR = crate::Reg<u32, _I2C_RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_RXDR;
#[doc = "`read()` method returns [i2c_rxdr::R](i2c_rxdr::R) reader structure"]
impl crate::Readable for I2C_RXDR {}
#[doc = "Access: No wait states"]
pub mod i2c_rxdr;
#[doc = "Access: No wait states\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_txdr](i2c_txdr) module"]
pub type I2C_TXDR = crate::Reg<u32, _I2C_TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_TXDR;
#[doc = "`read()` method returns [i2c_txdr::R](i2c_txdr::R) reader structure"]
impl crate::Readable for I2C_TXDR {}
#[doc = "`write(|w| ..)` method takes [i2c_txdr::W](i2c_txdr::W) writer structure"]
impl crate::Writable for I2C_TXDR {}
#[doc = "Access: No wait states"]
pub mod i2c_txdr;
#[doc = "I2C hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_hwcfgr](i2c_hwcfgr) module"]
pub type I2C_HWCFGR = crate::Reg<u32, _I2C_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_HWCFGR;
#[doc = "`read()` method returns [i2c_hwcfgr::R](i2c_hwcfgr::R) reader structure"]
impl crate::Readable for I2C_HWCFGR {}
#[doc = "I2C hardware configuration register"]
pub mod i2c_hwcfgr;
#[doc = "I2C version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_verr](i2c_verr) module"]
pub type I2C_VERR = crate::Reg<u32, _I2C_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_VERR;
#[doc = "`read()` method returns [i2c_verr::R](i2c_verr::R) reader structure"]
impl crate::Readable for I2C_VERR {}
#[doc = "I2C version register"]
pub mod i2c_verr;
#[doc = "I2C identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ipidr](i2c_ipidr) module"]
pub type I2C_IPIDR = crate::Reg<u32, _I2C_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_IPIDR;
#[doc = "`read()` method returns [i2c_ipidr::R](i2c_ipidr::R) reader structure"]
impl crate::Readable for I2C_IPIDR {}
#[doc = "I2C identification register"]
pub mod i2c_ipidr;
#[doc = "I2C size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sidr](i2c_sidr) module"]
pub type I2C_SIDR = crate::Reg<u32, _I2C_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SIDR;
#[doc = "`read()` method returns [i2c_sidr::R](i2c_sidr::R) reader structure"]
impl crate::Readable for I2C_SIDR {}
#[doc = "I2C size identification register"]
pub mod i2c_sidr;
