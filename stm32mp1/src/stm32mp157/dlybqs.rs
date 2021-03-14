#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DLYB control register"]
    pub dlyb_cr: DLYB_CR,
    #[doc = "0x04 - DLYB configuration register"]
    pub dlyb_cfgr: DLYB_CFGR,
    _reserved2: [u8; 1004usize],
    #[doc = "0x3f4 - DLYB IP version register"]
    pub dlyb_verr: DLYB_VERR,
    #[doc = "0x3f8 - DLYB IP identification register"]
    pub dlyb_ipidr: DLYB_IPIDR,
    #[doc = "0x3fc - DLYB size ID register"]
    pub dlyb_sidr: DLYB_SIDR,
}
#[doc = "DLYB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_cr](dlyb_cr) module"]
pub type DLYB_CR = crate::Reg<u32, _DLYB_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLYB_CR;
#[doc = "`read()` method returns [dlyb_cr::R](dlyb_cr::R) reader structure"]
impl crate::Readable for DLYB_CR {}
#[doc = "`write(|w| ..)` method takes [dlyb_cr::W](dlyb_cr::W) writer structure"]
impl crate::Writable for DLYB_CR {}
#[doc = "DLYB control register"]
pub mod dlyb_cr;
#[doc = "DLYB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_cfgr](dlyb_cfgr) module"]
pub type DLYB_CFGR = crate::Reg<u32, _DLYB_CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLYB_CFGR;
#[doc = "`read()` method returns [dlyb_cfgr::R](dlyb_cfgr::R) reader structure"]
impl crate::Readable for DLYB_CFGR {}
#[doc = "`write(|w| ..)` method takes [dlyb_cfgr::W](dlyb_cfgr::W) writer structure"]
impl crate::Writable for DLYB_CFGR {}
#[doc = "DLYB configuration register"]
pub mod dlyb_cfgr;
#[doc = "DLYB IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_verr](dlyb_verr) module"]
pub type DLYB_VERR = crate::Reg<u32, _DLYB_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLYB_VERR;
#[doc = "`read()` method returns [dlyb_verr::R](dlyb_verr::R) reader structure"]
impl crate::Readable for DLYB_VERR {}
#[doc = "DLYB IP version register"]
pub mod dlyb_verr;
#[doc = "DLYB IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_ipidr](dlyb_ipidr) module"]
pub type DLYB_IPIDR = crate::Reg<u32, _DLYB_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLYB_IPIDR;
#[doc = "`read()` method returns [dlyb_ipidr::R](dlyb_ipidr::R) reader structure"]
impl crate::Readable for DLYB_IPIDR {}
#[doc = "DLYB IP identification register"]
pub mod dlyb_ipidr;
#[doc = "DLYB size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_sidr](dlyb_sidr) module"]
pub type DLYB_SIDR = crate::Reg<u32, _DLYB_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLYB_SIDR;
#[doc = "`read()` method returns [dlyb_sidr::R](dlyb_sidr::R) reader structure"]
impl crate::Readable for DLYB_SIDR {}
#[doc = "DLYB size ID register"]
pub mod dlyb_sidr;
