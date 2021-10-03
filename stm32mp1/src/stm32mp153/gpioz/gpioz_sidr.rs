#[doc = "Register `GPIOZ_SIDR` reader"]
pub struct R(crate::R<GPIOZ_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOZ_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOZ_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOZ_SIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIDR` reader - SIDR"]
pub struct SIDR_R(crate::FieldReader<u32, u32>);
impl SIDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        SIDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - SIDR"]
    #[inline(always)]
    pub fn sidr(&self) -> SIDR_R {
        SIDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_sidr](index.html) module"]
pub struct GPIOZ_SIDR_SPEC;
impl crate::RegisterSpec for GPIOZ_SIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioz_sidr::R](R) reader structure"]
impl crate::Readable for GPIOZ_SIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOZ_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for GPIOZ_SIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa3c5_dd01
    }
}
