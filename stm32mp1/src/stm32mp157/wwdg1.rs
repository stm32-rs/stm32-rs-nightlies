#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub wwdg_cr: WWDG_CR,
    #[doc = "0x04 - Configuration register"]
    pub wwdg_cfr: WWDG_CFR,
    #[doc = "0x08 - Status register"]
    pub wwdg_sr: WWDG_SR,
    _reserved3: [u8; 996usize],
    #[doc = "0x3f0 - WWDG hardware configuration register"]
    pub wwdg_hwcfgr: WWDG_HWCFGR,
    #[doc = "0x3f4 - WWDG version register"]
    pub wwdg_verr: WWDG_VERR,
    #[doc = "0x3f8 - WWDG ID register"]
    pub wwdg_ipidr: WWDG_IPIDR,
    #[doc = "0x3fc - WWDG size ID register"]
    pub wwdg_sidr: WWDG_SIDR,
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_cr](wwdg_cr) module"]
pub type WWDG_CR = crate::Reg<u32, _WWDG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_CR;
#[doc = "`read()` method returns [wwdg_cr::R](wwdg_cr::R) reader structure"]
impl crate::Readable for WWDG_CR {}
#[doc = "`write(|w| ..)` method takes [wwdg_cr::W](wwdg_cr::W) writer structure"]
impl crate::Writable for WWDG_CR {}
#[doc = "Control register"]
pub mod wwdg_cr;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_cfr](wwdg_cfr) module"]
pub type WWDG_CFR = crate::Reg<u32, _WWDG_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_CFR;
#[doc = "`read()` method returns [wwdg_cfr::R](wwdg_cfr::R) reader structure"]
impl crate::Readable for WWDG_CFR {}
#[doc = "`write(|w| ..)` method takes [wwdg_cfr::W](wwdg_cfr::W) writer structure"]
impl crate::Writable for WWDG_CFR {}
#[doc = "Configuration register"]
pub mod wwdg_cfr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_sr](wwdg_sr) module"]
pub type WWDG_SR = crate::Reg<u32, _WWDG_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_SR;
#[doc = "`read()` method returns [wwdg_sr::R](wwdg_sr::R) reader structure"]
impl crate::Readable for WWDG_SR {}
#[doc = "`write(|w| ..)` method takes [wwdg_sr::W](wwdg_sr::W) writer structure"]
impl crate::Writable for WWDG_SR {}
#[doc = "Status register"]
pub mod wwdg_sr;
#[doc = "WWDG hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_hwcfgr](wwdg_hwcfgr) module"]
pub type WWDG_HWCFGR = crate::Reg<u32, _WWDG_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_HWCFGR;
#[doc = "`read()` method returns [wwdg_hwcfgr::R](wwdg_hwcfgr::R) reader structure"]
impl crate::Readable for WWDG_HWCFGR {}
#[doc = "WWDG hardware configuration register"]
pub mod wwdg_hwcfgr;
#[doc = "WWDG version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_verr](wwdg_verr) module"]
pub type WWDG_VERR = crate::Reg<u32, _WWDG_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_VERR;
#[doc = "`read()` method returns [wwdg_verr::R](wwdg_verr::R) reader structure"]
impl crate::Readable for WWDG_VERR {}
#[doc = "WWDG version register"]
pub mod wwdg_verr;
#[doc = "WWDG ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_ipidr](wwdg_ipidr) module"]
pub type WWDG_IPIDR = crate::Reg<u32, _WWDG_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_IPIDR;
#[doc = "`read()` method returns [wwdg_ipidr::R](wwdg_ipidr::R) reader structure"]
impl crate::Readable for WWDG_IPIDR {}
#[doc = "WWDG ID register"]
pub mod wwdg_ipidr;
#[doc = "WWDG size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_sidr](wwdg_sidr) module"]
pub type WWDG_SIDR = crate::Reg<u32, _WWDG_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WWDG_SIDR;
#[doc = "`read()` method returns [wwdg_sidr::R](wwdg_sidr::R) reader structure"]
impl crate::Readable for WWDG_SIDR {}
#[doc = "WWDG size ID register"]
pub mod wwdg_sidr;
