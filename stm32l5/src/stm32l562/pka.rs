#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PKA control register"]
    pub pka_cr: PKA_CR,
    #[doc = "0x04 - PKA status register"]
    pub pka_sr: PKA_SR,
    #[doc = "0x08 - PKA clear flag register"]
    pub pka_clrfr: PKA_CLRFR,
}
#[doc = "PKA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pka_cr](pka_cr) module"]
pub type PKA_CR = crate::Reg<u32, _PKA_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKA_CR;
#[doc = "`read()` method returns [pka_cr::R](pka_cr::R) reader structure"]
impl crate::Readable for PKA_CR {}
#[doc = "`write(|w| ..)` method takes [pka_cr::W](pka_cr::W) writer structure"]
impl crate::Writable for PKA_CR {}
#[doc = "PKA control register"]
pub mod pka_cr;
#[doc = "PKA status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pka_sr](pka_sr) module"]
pub type PKA_SR = crate::Reg<u32, _PKA_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKA_SR;
#[doc = "`read()` method returns [pka_sr::R](pka_sr::R) reader structure"]
impl crate::Readable for PKA_SR {}
#[doc = "PKA status register"]
pub mod pka_sr;
#[doc = "PKA clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pka_clrfr](pka_clrfr) module"]
pub type PKA_CLRFR = crate::Reg<u32, _PKA_CLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKA_CLRFR;
#[doc = "`write(|w| ..)` method takes [pka_clrfr::W](pka_clrfr::W) writer structure"]
impl crate::Writable for PKA_CLRFR {}
#[doc = "PKA clear flag register"]
pub mod pka_clrfr;
