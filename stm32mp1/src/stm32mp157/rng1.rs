#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    pub rng_cr: RNG_CR,
    #[doc = "0x04 - RNG status register"]
    pub rng_sr: RNG_SR,
    #[doc = "0x08 - The RNG_DR register is a read-only register."]
    pub rng_dr: RNG_DR,
    _reserved3: [u8; 996usize],
    #[doc = "0x3f0 - RNG hardware configuration register"]
    pub rng_hwcfgr: RNG_HWCFGR,
    #[doc = "0x3f4 - RNG version register"]
    pub rng_verr: RNG_VERR,
    #[doc = "0x3f8 - RNG identification register"]
    pub rng_ipidr: RNG_IPIDR,
    #[doc = "0x3fc - RNG size ID register"]
    pub rng_sidr: RNG_SIDR,
}
#[doc = "RNG control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_cr](rng_cr) module"]
pub type RNG_CR = crate::Reg<u32, _RNG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_CR;
#[doc = "`read()` method returns [rng_cr::R](rng_cr::R) reader structure"]
impl crate::Readable for RNG_CR {}
#[doc = "`write(|w| ..)` method takes [rng_cr::W](rng_cr::W) writer structure"]
impl crate::Writable for RNG_CR {}
#[doc = "RNG control register"]
pub mod rng_cr;
#[doc = "RNG status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_sr](rng_sr) module"]
pub type RNG_SR = crate::Reg<u32, _RNG_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_SR;
#[doc = "`read()` method returns [rng_sr::R](rng_sr::R) reader structure"]
impl crate::Readable for RNG_SR {}
#[doc = "`write(|w| ..)` method takes [rng_sr::W](rng_sr::W) writer structure"]
impl crate::Writable for RNG_SR {}
#[doc = "RNG status register"]
pub mod rng_sr;
#[doc = "The RNG_DR register is a read-only register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_dr](rng_dr) module"]
pub type RNG_DR = crate::Reg<u32, _RNG_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_DR;
#[doc = "`read()` method returns [rng_dr::R](rng_dr::R) reader structure"]
impl crate::Readable for RNG_DR {}
#[doc = "The RNG_DR register is a read-only register."]
pub mod rng_dr;
#[doc = "RNG hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_hwcfgr](rng_hwcfgr) module"]
pub type RNG_HWCFGR = crate::Reg<u32, _RNG_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_HWCFGR;
#[doc = "`read()` method returns [rng_hwcfgr::R](rng_hwcfgr::R) reader structure"]
impl crate::Readable for RNG_HWCFGR {}
#[doc = "RNG hardware configuration register"]
pub mod rng_hwcfgr;
#[doc = "RNG version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_verr](rng_verr) module"]
pub type RNG_VERR = crate::Reg<u32, _RNG_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_VERR;
#[doc = "`read()` method returns [rng_verr::R](rng_verr::R) reader structure"]
impl crate::Readable for RNG_VERR {}
#[doc = "RNG version register"]
pub mod rng_verr;
#[doc = "RNG identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_ipidr](rng_ipidr) module"]
pub type RNG_IPIDR = crate::Reg<u32, _RNG_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_IPIDR;
#[doc = "`read()` method returns [rng_ipidr::R](rng_ipidr::R) reader structure"]
impl crate::Readable for RNG_IPIDR {}
#[doc = "RNG identification register"]
pub mod rng_ipidr;
#[doc = "RNG size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_sidr](rng_sidr) module"]
pub type RNG_SIDR = crate::Reg<u32, _RNG_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_SIDR;
#[doc = "`read()` method returns [rng_sidr::R](rng_sidr::R) reader structure"]
impl crate::Readable for RNG_SIDR {}
#[doc = "RNG size ID register"]
pub mod rng_sidr;
