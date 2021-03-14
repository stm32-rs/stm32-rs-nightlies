#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register (IWDG_KR)"]
    pub kr: KR,
    #[doc = "0x04 - Prescaler register (IWDG_PR)"]
    pub pr: PR,
    #[doc = "0x08 - Reload register (IWDG_RLR)"]
    pub rlr: RLR,
    #[doc = "0x0c - Status register (IWDG_SR)"]
    pub sr: SR,
}
#[doc = "Key register (IWDG_KR)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kr](kr) module"]
pub type KR = crate::Reg<u32, _KR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KR;
#[doc = "`write(|w| ..)` method takes [kr::W](kr::W) writer structure"]
impl crate::Writable for KR {}
#[doc = "Key register (IWDG_KR)"]
pub mod kr;
#[doc = "Prescaler register (IWDG_PR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "`write(|w| ..)` method takes [pr::W](pr::W) writer structure"]
impl crate::Writable for PR {}
#[doc = "Prescaler register (IWDG_PR)"]
pub mod pr;
#[doc = "Reload register (IWDG_RLR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr](rlr) module"]
pub type RLR = crate::Reg<u32, _RLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR;
#[doc = "`read()` method returns [rlr::R](rlr::R) reader structure"]
impl crate::Readable for RLR {}
#[doc = "`write(|w| ..)` method takes [rlr::W](rlr::W) writer structure"]
impl crate::Writable for RLR {}
#[doc = "Reload register (IWDG_RLR)"]
pub mod rlr;
#[doc = "Status register (IWDG_SR)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status register (IWDG_SR)"]
pub mod sr;
