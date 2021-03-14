#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register"]
    pub kr: KR,
    #[doc = "0x04 - Prescaler register"]
    pub pr: PR,
    #[doc = "0x08 - Reload register"]
    pub rlr: RLR,
    #[doc = "0x0c - Status register"]
    pub sr: SR,
    #[doc = "0x10 - Window register"]
    pub winr: WINR,
    _reserved5: [u8; 988usize],
    #[doc = "0x3f0 - hardware configuration register"]
    pub hwcfgr: HWCFGR,
    #[doc = "0x3f4 - EXTI IP Version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - EXTI Identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - EXTI Size ID register"]
    pub sidr: SIDR,
}
#[doc = "Key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kr](kr) module"]
pub type KR = crate::Reg<u32, _KR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KR;
#[doc = "`write(|w| ..)` method takes [kr::W](kr::W) writer structure"]
impl crate::Writable for KR {}
#[doc = "Key register"]
pub mod kr;
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "`write(|w| ..)` method takes [pr::W](pr::W) writer structure"]
impl crate::Writable for PR {}
#[doc = "Prescaler register"]
pub mod pr;
#[doc = "Reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr](rlr) module"]
pub type RLR = crate::Reg<u32, _RLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR;
#[doc = "`read()` method returns [rlr::R](rlr::R) reader structure"]
impl crate::Readable for RLR {}
#[doc = "`write(|w| ..)` method takes [rlr::W](rlr::W) writer structure"]
impl crate::Writable for RLR {}
#[doc = "Reload register"]
pub mod rlr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status register"]
pub mod sr;
#[doc = "Window register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winr](winr) module"]
pub type WINR = crate::Reg<u32, _WINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINR;
#[doc = "`read()` method returns [winr::R](winr::R) reader structure"]
impl crate::Readable for WINR {}
#[doc = "`write(|w| ..)` method takes [winr::W](winr::W) writer structure"]
impl crate::Writable for WINR {}
#[doc = "Window register"]
pub mod winr;
#[doc = "hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](hwcfgr) module"]
pub type HWCFGR = crate::Reg<u32, _HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR;
#[doc = "`read()` method returns [hwcfgr::R](hwcfgr::R) reader structure"]
impl crate::Readable for HWCFGR {}
#[doc = "`write(|w| ..)` method takes [hwcfgr::W](hwcfgr::W) writer structure"]
impl crate::Writable for HWCFGR {}
#[doc = "hardware configuration register"]
pub mod hwcfgr;
#[doc = "EXTI IP Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verr](verr) module"]
pub type VERR = crate::Reg<u32, _VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERR;
#[doc = "`read()` method returns [verr::R](verr::R) reader structure"]
impl crate::Readable for VERR {}
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "EXTI Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipidr](ipidr) module"]
pub type IPIDR = crate::Reg<u32, _IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPIDR;
#[doc = "`read()` method returns [ipidr::R](ipidr::R) reader structure"]
impl crate::Readable for IPIDR {}
#[doc = "EXTI Identification register"]
pub mod ipidr;
#[doc = "EXTI Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidr](sidr) module"]
pub type SIDR = crate::Reg<u32, _SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDR;
#[doc = "`read()` method returns [sidr::R](sidr::R) reader structure"]
impl crate::Readable for SIDR {}
#[doc = "EXTI Size ID register"]
pub mod sidr;
